# Feature Specification: Auto-Translation Detection

**Feature Branch**: `010-auto-translation-detection`
**Created**: 2025-12-XX
**Status**: Ready for Implementation
**Input**: Automatic detection of already translated texts during extraction process

**🎯 Version 1.0 Scope**: Implement transparent auto-detection that marks extracted texts as `translated` when:
1. Source language is CJK (`ja`, `zh`, `ko`)
2. Target language is non-CJK (`fr`, `en`, `es`, etc.)
3. Cleaned text contains NO CJK characters

**Note**: Detection happens automatically during each extraction. No manual button required.

## User Scenarios & Testing

### User Story 1 - Transparent Auto-Detection During Extraction (Priority: P1)

Localisateur extrait des textes d'un jeu et voit automatiquement certains textes marqués comme déjà traduits sans aucune action manuelle.

**Why this priority**: This is the core functionality - automatic detection should work transparently during the extraction process without requiring manual intervention.

**Independent Test**: Can be tested by extracting texts from a game with mixed translated/untranslated content and verifying that appropriate texts are auto-marked as translated.

**Acceptance Scenarios**:

1. **Given** a game with texts already translated from Japanese to French, **When** extraction occurs, **Then** texts without CJK characters are automatically marked as `translated`
2. **Given** a text containing Japanese characters, **When** extraction occurs, **Then** the text remains marked as `extracted` (needs translation)
3. **Given** a text with placeholders like `[CSELF_1]`, **When** extraction occurs, **Then** placeholders are ignored during CJK detection
4. **Given** a very short text (< 10 characters), **When** extraction occurs, **Then** the text is not auto-detected regardless of CJK content

---

### User Story 2 - Language Configuration Awareness (Priority: P1)

The auto-detection only activates for appropriate language pairs (CJK → non-CJK) and respects user language settings.

**Why this priority**: Language configuration is critical - detection should only work when it makes sense and respect user preferences.

**Independent Test**: Can be tested by changing language settings and verifying that auto-detection activates/deactivates appropriately.

**Acceptance Scenarios**:

1. **Given** source language is Japanese and target is French, **When** extraction occurs, **Then** auto-detection is active
2. **Given** source language is English and target is French, **When** extraction occurs, **Then** auto-detection is inactive
3. **Given** source language is Japanese and target is Chinese, **When** extraction occurs, **Then** auto-detection is inactive
4. **Given** user changes language settings, **When** extraction occurs, **Then** the new settings are immediately respected

---

### User Story 3 - Seamless Integration with Existing Workflow (Priority: P1)

The auto-detection enriches the existing extraction workflow without disrupting any existing functionality.

**Why this priority**: This feature must integrate seamlessly without breaking existing workflows or requiring changes to other parts of the system.

**Independent Test**: Can be tested by running the complete extraction → DB injection → UI display workflow and verifying that auto-detection happens transparently.

**Acceptance Scenarios**:

1. **Given** existing extraction workflow, **When** auto-detection is added, **Then** all existing functionality continues to work unchanged
2. **Given** texts that don't qualify for auto-detection, **When** extraction occurs, **Then** they follow the normal workflow (marked as `extracted`)
3. **Given** auto-detected texts, **When** they are saved to DB, **Then** they have correct status and translation text
4. **Given** rollback scenarios, **When** DB injection fails, **Then** auto-detected statuses are properly preserved in rollback

## Technical Implementation

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
