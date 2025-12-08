# Spécification Fonctionnelle: Contrôles de Pause Configurables

## 1. Contexte

### 1.1 Situation actuelle
Les sessions de traduction séquentielle Ollama utilisent actuellement des pauses automatiques hardcodées :
- Pause toutes les 500 traductions
- Durée de pause fixe de 12 minutes
- Aucune possibilité de configuration utilisateur

### 1.2 Problème identifié
Cette approche rigide ne s'adapte pas aux différents cas d'usage :
- Matériel performant nécessitant moins de pauses
- Sessions courtes ne nécessitant pas de pauses
- Utilisateurs souhaitant des pauses plus courtes/longer

### 1.3 Objectif
Permettre aux utilisateurs de configurer finement le comportement des pauses automatiques selon leurs besoins tout en maintenant la robustesse de la protection matérielle.

## 2. Architecture décidée

### 2.1 Séparation des responsabilités

#### Backend (Rust/Tauri) - Logique de sécurité
- **Gère les vraies pauses matérielles** : `tokio::time::sleep()` pour protection contre surchauffe
- **Configuration des paramètres** : Taille des batches et durée des pauses via `PauseSettings`
- **Intégration avec code existant** : Utilise `batch_counter` existant dans `SequentialSession`
- **Robustesse garantie** : Les pauses s'exécutent même si le frontend crash
- **État de session** : Tracking précis du statut et timing des pauses via `pause_end_time`

#### Frontend (Vue/TypeScript) - Interface utilisateur
- **Configuration utilisateur** : Interface pour paramétrer les pauses
- **Affichage du compteur** : Visibilité temps réel pendant les pauses
- **Contrôle des sessions** : Démarrage/arrêt selon configuration
- **Validation des paramètres** : Vérifications côté utilisateur

### 2.2 Avantages de cette architecture
- **Sécurité matérielle** : Protection garantie contre la surchauffe
- **Expérience utilisateur** : Compteur visible et contrôles intuitifs
- **Intégration propre** : Utilise le `batch_counter` existant plutôt que de le remplacer
- **Robustesse** : Fonctionnement même en cas de crash frontend
- **Maintenabilité** : Séparation claire des responsabilités

## 3. Fonctionnalités requises

### 3.0 Note d'intégration
**Le code existant utilise déjà un `batch_counter` dans `OllamaSequentialSession`.**
La nouvelle implémentation l'intègre proprement en l'étendant à `SequentialSession` commune,
plutôt que de le remplacer, assurant la compatibilité ascendante.

### 2.1 Configuration des pauses

#### 2.1.1 Activation/désactivation
- **Type** : Case à cocher binaire
- **Étiquette** : "Activer les pauses automatiques"
- **Valeur par défaut** : Activé (true)
- **Comportement** :
  - Si désactivé : Aucune pause automatique
  - Si activé : Application des paramètres configurés

#### 2.1.2 Nombre de traductions avant pause
- **Type** : Champ numérique
- **Étiquette** : "Traductions avant pause"
- **Valeurs** :
  - Minimum : 1
  - Maximum : 1000
  - Défaut : 150
  - Étape : 10
- **Unité** : nombre de traductions

#### 2.1.3 Durée de pause
- **Type** : Champ numérique
- **Étiquette** : "Durée de pause (minutes)"
- **Valeurs** :
  - Minimum : 1
  - Maximum : 60
  - Défaut : 5
  - Étape : 1
- **Unité** : minutes

### 2.2 Interface utilisateur

#### 2.2.1 Composant dédié : `PauseControls.vue`
- **Localisation** : `app/components/settings/PauseControls.vue`
- **Pattern** : Composant de settings réutilisable
- **Intégration** : Page settings principale
- **Style** : Cohérent avec les autres composants settings (UCard, UFormField)

#### 2.2.2 Pattern de communication
- **Props** : `settings` (contient `translation.pause`)
- **Emits** : `update:enabled`, `update:batchSize`, `update:pauseDurationMinutes`
- **Persistance** : Gérée par le parent (page settings)

#### 2.2.3 États des contrôles
- **Configuration** : Contrôles toujours actifs (pas de verrouillage pendant traduction)
- **Validation** : Vérifications en temps réel des valeurs saisies
- **Feedback** : Messages d'erreur et correction automatique

#### 2.2.3 Validation en temps réel
- Messages d'erreur pour valeurs hors limites
- Correction automatique des valeurs invalides
- Feedback visuel immédiat

### 3.3 Compteur de pause dans la page translation

#### 3.3.1 Affichage conditionnel
- **Visible uniquement** pendant les pauses automatiques actives
- **Masqué** en dehors des pauses ou si pauses désactivées
- **Format** : "Pause en cours: MM:SS"
- **Source** : Données du backend via le système de polling existant

#### 3.3.2 Fonction d'affichage uniquement
- **Pas de contrôle** : Le compteur est purement informatif
- **Configuration séparée** : Paramètres configurés dans le composant settings dédié
- **Mise à jour temps réel** : Via l'état des sessions de traduction
- **Précision** : À la seconde près depuis les données backend

#### 3.3.3 Style visuel
- **Icône** : Pause circle (⏸️)
- **Couleur** : Bleu informatif
- **Position** : Prominente dans la page translation (en-tête ou section dédiée)
- **Design** : Très visible pour informer l'utilisateur du statut de pause

### 2.4 Persistance des paramètres

#### 2.4.1 Stockage
- **Système** : Tauri Store (settings.json)
- **Clé** : `translation.pause`
- **Structure** :
```typescript
{
  enabled: boolean,
  batchSize: number,
  pauseDurationMinutes: number
}
```

#### 2.4.2 Sauvegarde automatique
- **Déclencheur** : Modification de n'importe quel contrôle
- **Moment** : Immédiat (sans attendre la validation)
- **Gestion d'erreur** : Notification en cas d'échec

#### 2.4.3 Chargement
- **Moment** : Au montage du composant
- **Fallback** : Valeurs par défaut si chargement échoue

## 3. Contraintes techniques

### 3.1 Compatibilité
- **Providers** : Ollama et RunPod (même interface)
- **Types de traduction** : Simple et retraduction
- **Sessions multiples** : Paramètres partagés entre sessions

### 3.2 Performance
- **Impact minimal** sur les performances de traduction
- **Pas de polling supplémentaire** pour le compteur
- **Lazy loading** des paramètres

### 3.3 Sécurité
- **Validation côté frontend** : Limites min/max
- **Validation côté backend** : Protection contre valeurs invalides
- **Pas d'impact** sur la sécurité des traductions

## 4. Interfaces et APIs

### 4.1 Types TypeScript

```typescript
// Extension de AppSettings
interface AppSettings {
  translation: {
    pause: {
      enabled: boolean
      batchSize: number
      pauseDurationMinutes: number
    }
  }
}

// Requête de traduction étendue
interface SequentialTranslationRequest {
  pauseSettings?: {
    enabled: boolean
    batchSize: number
    pauseDurationMinutes: number
  }
}

// Progrès étendu
interface SequentialProgress {
  pauseTimeRemaining?: number // secondes
}
```

### 4.2 APIs Backend

#### 4.2.1 Commandes Tauri
- **startSequentialTranslation** : Accepte les paramètres de pause
- **getTranslationProgress** : Retourne le temps de pause restant

#### 4.2.2 Structures Rust
```rust
#[derive(Serialize, Deserialize)]
pub struct PauseSettings {
    pub enabled: bool,
    pub batch_size: u32,
    pub pause_duration_minutes: u32,
}

// Intégration dans SequentialSession commune
pub struct SequentialSession {
    // ... autres champs existants ...
    pub pause_settings: PauseSettings,  // Configuration utilisateur
    pub batch_counter: usize,           // Compteur interne (déplacé depuis OllamaSequentialSession)
}

// OllamaSequentialSession simplifiée
pub struct OllamaSequentialSession {
    pub common: SequentialSession,      // Maintenant inclut pause_settings et batch_counter
    pub app_handle: AppHandle,
    pub pause_end_time: Option<std::time::Instant>, // Nouveau pour tracking du temps restant
}
```

## 5. Scénarios d'usage

### 5.1 Scénario principal : Configuration personnalisée
1. Utilisateur ouvre l'interface de traduction
2. Configure : 200 traductions, 3 minutes de pause
3. Lance la traduction
4. Observe les pauses selon ses paramètres
5. Voit le compteur pendant chaque pause

### 5.2 Scénario alternatif : Désactivation complète
1. Utilisateur décoche "Activer les pauses"
2. Lance une traduction longue
3. Traduction s'exécute sans interruption
4. Aucun compteur n'apparaît

### 5.3 Scénario d'erreur : Valeurs invalides
1. Utilisateur saisit 0 comme nombre de traductions
2. Interface affiche message d'erreur
3. Valeur corrigée automatiquement à la minimum (1)

## 6. Critères d'acceptation

### 6.1 Fonctionnels
- [ ] Configuration accessible et intuitive
- [ ] Paramètres sauvegardés automatiquement
- [ ] Compteur précis et visible uniquement pendant pauses
- [ ] Contrôles verrouillés pendant traduction
- [ ] Valeurs par défaut appropriées

### 6.2 Techniques
- [ ] Code TypeScript typé strictement
- [ ] Tests unitaires pour la logique de pause
- [ ] Validation côté frontend et backend
- [ ] Performance non dégradée
- [ ] Compatibilité ascendante

### 6.3 Qualité
- [ ] Interface responsive et accessible
- [ ] Messages d'erreur clairs
- [ ] Documentation à jour
- [ ] Code review validé
