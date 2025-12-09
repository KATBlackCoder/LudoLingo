# Spécification 010 : Détection automatique des textes déjà traduits

## Logique simple

**Détection automatique transparente** lors de l'extraction qui analyse les textes extraits et les marque automatiquement `translated` si :

1. **Langue source** est CJK (`ja`, `zh`, `ko`)
2. **Langue cible** n'est pas CJK (`fr`, `en`, `es`, etc.)
3. **Texte nettoyé** ne contient **PAS** de caractères CJK

**Note** : La détection se fait automatiquement lors de chaque extraction. Aucun bouton manuel requis.

## Implémentation

### Installation
```bash
pnpm add cjk-regex
```

### Fonction utilitaire
```typescript
// app/utils/autoTranslation.ts
import * as cjk from 'cjk-regex'
import { useSettingsStore } from '~/composables/useStore'

export async function shouldAutoMarkTranslated(text: string): Promise<boolean> {
  // 1. Récupérer les langues depuis les settings
  const settingsStore = await useSettingsStore()
  const sourceLang = await settingsStore.get('sourceLanguage') as string || 'ja'
  const targetLang = await settingsStore.get('targetLanguage') as string || 'fr'

  // 2. Vérifier que source est CJK
  const isSourceCJK = ['ja', 'zh', 'ko'].includes(sourceLang.toLowerCase())
  if (!isSourceCJK) return false

  // 3. Vérifier que cible n'est pas CJK
  const isTargetCJK = ['ja', 'zh', 'ko'].includes(targetLang.toLowerCase())
  if (isTargetCJK) return false

  // 4. Nettoyer les placeholders
  const cleanText = text.replace(/\[[A-Z_][A-Z0-9_]*(?:_\d+)*(?:_[A-Z0-9_]+)*\]/g, '').trim()

  // 5. Vérifier longueur minimum
  if (cleanText.length < 10) return false

  // 6. Vérifier que le texte NE CONTIENT PAS de caractères CJK
  const cjkRegex = cjk.all().toRegExp()
  const hasCJK = cjkRegex.test(cleanText)

  // Si pas de CJK = probablement déjà traduit
  return !hasCJK
}
```

### Fonction utilitaire applyAutoTranslationDetection
```typescript
// app/utils/autoTranslation.ts
import * as cjk from 'cjk-regex'
import { useSettingsStore } from '~/composables/useStore'
import type { TextEntry } from '~/types/scanning-commands'

export async function applyAutoTranslationDetection(texts: TextEntry[]): Promise<TextEntry[]> {
  // 1. Récupérer les langues depuis les settings
  const settingsStore = await useSettingsStore()
  const sourceLang = await settingsStore.get('sourceLanguage') as string || 'ja'
  const targetLang = await settingsStore.get('targetLanguage') as string || 'fr'

  // 2. Vérifier que source est CJK et cible non-CJK
  const isSourceCJK = ['ja', 'zh', 'ko'].includes(sourceLang.toLowerCase())
  const isTargetCJK = ['ja', 'zh', 'ko'].includes(targetLang.toLowerCase())

  if (!isSourceCJK || isTargetCJK) {
    return texts // Pas de détection automatique pour cette configuration
  }

  // 3. Traiter chaque texte extrait
  return texts.map(text => {
    const processedText = { ...text }

    // Nettoyer les placeholders [CODE_*]
    const cleanText = text.source_text.replace(/\[[A-Z_][A-Z0-9_]*(?:_\d+)*(?:_[A-Z0-9_]+)*\]/g, '').trim()

    // Vérifier longueur minimum
    if (cleanText.length < 10) {
      return processedText // Trop court, garder status original
    }

    // Vérifier que le texte NE CONTIENT PAS de caractères CJK
    const cjkRegex = cjk.all().toRegExp()
    const hasCJK = cjkRegex.test(cleanText)

    // Si pas de CJK = probablement déjà traduit
    if (!hasCJK) {
      processedText.status = 'Translated'
    }

    return processedText
  })
}
```

### Composable useAutoTranslationDetection
```typescript
// app/composables/translation/useAutoTranslationDetection.ts
import { useSettings } from '~/composables/useTauriSetting'
import * as cjk from 'cjk-regex'
import type { TextEntry } from '~/types/scanning-commands'

export function useAutoTranslationDetection() {
  const settings = useSettings()

  /**
   * Applique la détection automatique des textes déjà traduits
   * Logique CJK → non-CJK uniquement
   */
  const applyAutoTranslationDetection = async (texts: TextEntry[]): Promise<TextEntry[]> => {
    // 1. Récupérer les langues depuis les settings
    const userSettings = await settings.loadSettings()
    const sourceLang = userSettings.translation.sourceLanguage
    const targetLang = userSettings.translation.targetLanguage

    // 2. Vérifier que source est CJK et cible non-CJK
    const isSourceCJK = ['ja', 'zh', 'ko'].includes(sourceLang.toLowerCase())
    const isTargetCJK = ['ja', 'zh', 'ko'].includes(targetLang.toLowerCase())

    if (!isSourceCJK || isTargetCJK) {
      console.log(`🔍 Auto-détection ignorée: ${sourceLang} → ${targetLang} (non CJK→non-CJK)`)
      return texts // Pas de détection automatique
    }

    console.log(`🔍 Auto-détection activée: ${sourceLang} → ${targetLang}`)

    // 3. Traiter chaque texte extrait
    let detectedCount = 0
    const processedTexts = texts.map(text => {
      const processedText = { ...text }

      // Nettoyer les placeholders [CODE_*]
      const cleanText = text.source_text.replace(/\[[A-Z_][A-Z0-9_]*(?:_\d+)*(?:_[A-Z0-9_]+)*\]/g, '').trim()

      // Vérifier que le texte n'est pas vide après nettoyage
      if (cleanText.length === 0) {
        return processedText // Texte vide, garder status original
      }

      // Vérifier que le texte NE CONTIENT PAS de caractères CJK
      const cjkRegex = cjk.all().toRegExp()
      const hasCJK = cjkRegex.test(cleanText)

      // Si pas de CJK = probablement déjà traduit (textes, symboles, ponctuation universels)
      if (!hasCJK) {
        processedText.status = 'Translated'
        processedText.translated_text = text.source_text // Copier source vers traduction
        detectedCount++
      }

      return processedText
    })

    console.log(`✅ Auto-détection terminée: ${detectedCount} textes marqués comme traduits sur ${texts.length}`)
    return processedTexts
  }

  return {
    applyAutoTranslationDetection
  }
}
```

### Enrichissement de l'injection DB dans updateProjectTexts

**Vision** : Le composable **enrichit** le workflow d'injection DB existant, il ne le remplace pas.

```typescript
// app/stores/projects.ts - enrichissement du workflow existant
import { useAutoTranslationDetection } from '~/composables/translation/useAutoTranslationDetection'

const updateProjectTexts = async (projectId: number, texts: TextEntry[]) => {
  // 1. Préparation (sauvegarde état original, validation projet) - EXISTANT

  // 🚀 2. TRAITEMENT AUTOMATIQUE : Enrichir les textes AVANT injection DB
  const { applyAutoTranslationDetection } = useAutoTranslationDetection()
  const processedTexts = await applyAutoTranslationDetection(texts)

  // 💾 3. INJECTION DB : Sauvegarder les textes enrichis (workflow existant préservé)
  const dbResult = await createBulkTextEntries(projectId, processedTexts)

  // 4. Rechargement DB et mise à jour store Pinia - EXISTANT
  // ... (logique de rollback préservée)
}
```

#### Architecture : Enrichment vs Remplacement

| Aspect | Avant | Après |
|--------|-------|-------|
| **Injection DB** | `createBulkTextEntries(projectId, texts)` | `createBulkTextEntries(projectId, processedTexts)` |
| **Workflow** | Extraction → DB | Extraction → Traitement → DB |
| **Responsabilités** | DB uniquement | DB enrichie par logique métier |
| **Robustesse** | Statuts bruts | Statuts optimisés automatiquement |
| **Maintenance** | Injection pure | Injection intelligente |

### Workflow Complet Enrichi

```
Extraction de textes (backend)
        ↓
useAutoTranslationDetection.applyAutoTranslationDetection()
        ↓ (CJK → non-CJK : statuts auto-ajustés)
updateProjectTexts() - Préparation
        ↓
createBulkTextEntries(projectId, processedTexts)
        ↓ (Injection DB avec statuts enrichis)
Rechargement depuis DB + Mise à jour store Pinia
        ↓
Interface utilisateur avec statuts optimisés
```

## Exemples

### ✅ Automatiquement marqué (extracted → translated)
- Texte extrait : `"Bienvenue [CSELF_1] !"`
- Configuration : Source `ja` (CJK) → Cible `fr` (non-CJK)
- Après nettoyage : `"Bienvenue !"` (pas de CJK, ≥ 2 caractères)
- → **Status automatiquement défini à `translated`** + **`translated_text = source_text`** lors de l'extraction

### ❌ Garde status extracted
- Texte extrait : `"ゲーム [CSELF_1] !"`
- Configuration : Source `ja` (CJK) → Cible `fr` (non-CJK)
- Après nettoyage : `"ゲーム !"` (contient CJK)
- → **Status reste `extracted`** (nécessite traduction)

### ⚠️ Configuration non supportée (pas de détection)
- Configuration : Source `en` (non-CJK) → Cible `fr` (non-CJK)
- → **Aucune détection automatique** (logique CJK → non-CJK uniquement)

## Déploiement

1. **Vérifier les dépendances** : `cjk-regex` déjà installé (présent dans package.json)
2. **Créer le composable** `useAutoTranslationDetection` dans `app/composables/translation/useAutoTranslationDetection.ts`
3. **Modifier `updateProjectTexts()`** dans `app/stores/projects.ts` pour utiliser le composable
4. **Tester** l'extraction automatique sur un projet existant avec textes déjà traduits

## Dépendances

- **cjk-regex** : Détection de caractères CJK ([cjk-regex](https://github.com/ikatyang-collab/cjk-regex))
- **useSettingsStore** : Récupération des langues depuis les paramètres persistants

---

**Version mise à jour** : Décembre 2025
**Dépendances** : cjk-regex
**Statut** : Prêt pour implémentation
