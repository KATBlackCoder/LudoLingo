# Décomposition des tâches: Contrôles de Pause Configurables

## Vue d'ensemble

Cette spécification implémente des contrôles de pause configurables pour les sessions de traduction Ollama. Les tâches sont organisées par domaine fonctionnel avec marquage des dépendances et possibilités d'exécution parallèle.

## Tâches Backend (Rust)

### 1. Extension des types communs
**ID**: `backend-types-001`
**Priorité**: Critical
**Estimation**: 30 min
**Dépendances**: Aucune
**Parallèle**: Non

**Description**:
Ajouter les structures de données pour la configuration des pauses dans `src-tauri/src/translation/common/types.rs`

**Sous-tâches**:
- Créer structure `PauseSettings` avec `enabled`, `batch_size`, `pause_duration_minutes`
- Étendre `SequentialSession` avec `pause_settings: PauseSettings` et `batch_counter: usize`
- Étendre `SequentialTranslationRequest` avec champ `pause_settings: Option<PauseSettings>`
- Étendre `SequentialProgress` avec champ `pause_time_remaining: Option<i64>`
- Mettre à jour `OllamaSequentialSession` pour utiliser `SequentialSession` étendu
- Ajouter sérialisation/désérialisation pour compatibilité Tauri

**Critères d'acceptation**:
- [x] Code compile sans erreur
- [x] Structures sérialisables en JSON
- [x] Tests de compilation passent
- [x] PauseSettings créée avec les bons champs
- [x] SequentialSession étendue avec pause_settings et batch_counter
- [x] SequentialTranslationRequest étendue avec pause_settings
- [x] SequentialProgress étendue avec pause_time_remaining

### 2. Modification de la session Ollama
**ID**: `backend-session-002`
**Priorité**: High
**Estimation**: 45 min
**Dépendances**: `backend-types-001`
**Parallèle**: Non

**Description**:
Intégrer les paramètres de pause dans `OllamaSequentialSession`

**Sous-tâches**:
- Étendre `SequentialSession` avec `pause_settings: PauseSettings` et `batch_counter: usize`
- Ajouter champ `pause_end_time: Option<std::time::Instant>` à `OllamaSequentialSession`
- Modifier constructeur pour initialiser `SequentialSession` avec `pause_settings`
- Mettre à jour logique de pause pour utiliser `pause_settings.batch_size` au lieu de valeur hardcodée
- Utiliser `batch_counter` de `SequentialSession` pour le comptage interne

**Critères d'acceptation**:
- [x] Session créée avec paramètres de pause
- [x] Champ `pause_end_time` correctement initialisé
- [x] Code compile et tests passent

### 3. Mise à jour du gestionnaire de sessions
**ID**: `backend-manager-003`
**Priorité**: High
**Estimation**: 60 min
**Dépendances**: `backend-types-001`, `backend-session-002`
**Parallèle**: Non

**Description**:
Modifier `SequentialTranslationManager` pour traiter les paramètres de pause

**Sous-tâches**:
- Modifier `start_session` pour traiter `pause_settings` de la requête avec valeurs par défaut
- Mettre à jour `get_progress` pour calculer et retourner `pause_time_remaining` depuis `pause_end_time`
- Remplacer logique de pause hardcodée (500 traductions) par logique utilisant `pause_settings.batch_size`
- Utiliser `pause_settings.pause_duration_minutes` pour calculer la durée de pause
- Gérer les transitions pause/reprise avec paramètres personnalisés

**Critères d'acceptation**:
- [x] Sessions démarrent avec paramètres de pause personnalisés
- [x] Progrès retourne correctement le temps restant
- [x] Pauses respectent les paramètres configurés
- [x] Reprise fonctionne correctement

## Tâches Frontend (TypeScript/Vue)

### 4. Extension des paramètres utilisateur
**ID**: `frontend-settings-004`
**Priorité**: High
**Estimation**: 20 min
**Dépendances**: Aucune
**Parallèle**: Oui (avec backend)

**Description**:
Étendre l'interface des paramètres utilisateur dans `useTauriSetting.ts`

**Sous-tâches**:
- Ajouter section `pause` dans `AppSettings` avec `enabled`, `batchSize`, `pauseDurationMinutes`
- Définir valeurs par défaut : `enabled: true`, `batchSize: 150`, `pauseDurationMinutes: 5`
- Vérifier compatibilité avec chargement/sauvegarde existant

**Critères d'acceptation**:
- [x] Interface TypeScript étendue sans erreur
- [x] Valeurs par défaut appropriées
- [x] Chargement/sauvegarde fonctionnel

### 5. Créer composant PauseControls.vue
**ID**: `frontend-pause-controls-005`
**Priorité**: High
**Estimation**: 60 min
**Dépendances**: `frontend-settings-004`
**Parallèle**: Non

**Description**:
Créer le composant `PauseControls.vue` dans le dossier settings

**Sous-tâches**:
- Créer `app/components/settings/PauseControls.vue`
- Implémenter template avec UCard et contrôles UI
- Ajouter props `settings` et emits `update:enabled`, `update:batchSize`, `update:pauseDurationMinutes`
- Implémenter validation temps réel des valeurs
- Appliquer style cohérent avec les autres composants settings

**Critères d'acceptation**:
- [x] Interface visible et accessible
- [x] Contrôles fonctionnels avec validation
- [x] Style cohérent avec le reste de l'application

### 6. Validation et feedback utilisateur
**ID**: `frontend-validation-006`
**Priorité**: Medium
**Estimation**: 30 min
**Dépendances**: `frontend-pause-controls-005`
**Parallèle**: Oui

**Description**:
Implémenter la logique de validation et feedback dans le composant PauseControls

**Sous-tâches**:
- Créer validateurs pour batchSize (1-1000) et pauseDurationMinutes (1-60)
- Implémenter correction automatique des valeurs invalides
- Ajouter messages d'erreur contextuels en français
- Gérer feedback visuel immédiat pour l'utilisateur

**Critères d'acceptation**:
- [x] Validation batchSize (1-1000) fonctionnelle
- [x] Validation pauseDurationMinutes (1-60) fonctionnelle
- [x] Correction automatique des valeurs invalides
- [x] Messages d'erreur en français affichés

### 7. Intégration settings et export
**ID**: `frontend-integration-007`
**Priorité**: Medium
**Estimation**: 20 min
**Dépendances**: `frontend-pause-controls-005`
**Parallèle**: Oui

**Description**:
Intégrer PauseControls dans le système settings

**Sous-tâches**:
- Ajouter export dans `app/components/settings/index.ts`
- Préparer pattern d'intégration dans page settings
- Tester cohérence avec autres composants settings
- Vérifier persistance automatique des paramètres

**Critères d'acceptation**:
- [x] Export ajouté dans index.ts
- [x] Pattern d'intégration préparé pour page settings
- [x] Cohérence avec autres composants settings
- [x] Persistance automatique fonctionnelle

### 8. Compteur dans la page translation
**ID**: `frontend-counter-008`
**Priorité**: Medium
**Estimation**: 45 min
**Dépendances**: `backend-manager-003`, `frontend-integration-007`
**Parallèle**: Non

**Description**:
Implémenter l'affichage du compteur de pause dans la page principale de traduction

**Sous-tâches**:
- Ajouter section compteur dans `app/pages/translation.vue`
- Connecter aux données `pause_time_remaining` du backend
- Implémenter formatage "Pause en cours: MM:SS"
- Gérer affichage conditionnel selon état des sessions

**Critères d'acceptation**:
- [x] Section compteur ajoutée dans app/pages/translation.vue
- [x] Connexion aux données pause_time_remaining fonctionnelle
- [x] Format "Pause en cours: MM:SS" correct
- [x] Affichage conditionnel selon état des sessions

**Critères d'acceptation**:
- [x] Compteur visible uniquement pendant pauses
- [x] Format MM:SS correct
- [x] Mise à jour temps réel
- [x] Style cohérent (bleu informatif)

## Tâches d'intégration

### 8. Connexion frontend/backend
**ID**: `integration-connect-008`
**Priorité**: Critical
**Estimation**: 45 min
**Dépendances**: `backend-manager-003`, `frontend-state-006`
**Parallèle**: Non

**Description**:
Connecter les paramètres frontend aux appels backend

**Sous-tâches**:
- Étendre `startAllTranslations` pour passer `pauseSettings`
- Étendre `handleRetranslateSelected` pour passer `pauseSettings`
- Vérifier compatibilité avec providers Ollama et RunPod
- Gérer fallback vers valeurs par défaut

**Critères d'acceptation**:
- [x] Paramètres passent correctement au backend
- [x] Traductions démarrent avec paramètres personnalisés
- [x] Compatibilité avec tous les types de traduction

### 9. Tests et validation
**ID**: `testing-validation-009`
**Priorité**: High
**Estimation**: 60 min
**Dépendances**: Toutes les tâches précédentes
**Parallèle**: Non

**Description**:
Valider l'implémentation complète

**Sous-tâches**:
- Tests unitaires pour logique de pause backend
- Tests d'intégration frontend/backend
- Validation des cas limites (valeurs extrêmes)
- Tests de performance (impact sur traduction)
- Tests de régression (fonctionnalités existantes)

**Critères d'acceptation**:
- [x] Tous les tests passent
- [x] Performance non dégradée
- [x] Interface fonctionne dans tous les scénarios
- [x] Aucune régression détectée

## Ordre d'exécution recommandé

```
1. backend-types-001
   ├── backend-session-002
      ├── backend-manager-003
         ├── integration-connect-008
2. frontend-settings-004
   ├── frontend-controls-005
      ├── frontend-state-006
         ├── frontend-counter-007
            ├── integration-connect-008
3. testing-validation-009
```

## Points d'attention

### Tests parallèles
Les tâches `frontend-settings-004` et `frontend-counter-007` peuvent être développées en parallèle avec le backend une fois les types définis.

### Risques
- **Dépendance critique** : `backend-types-001` doit être terminé avant tout développement
- **Interface bloquante** : `integration-connect-008` nécessite backend et frontend complets
- **Performance** : Tests de performance critiques pour valider l'impact

### Métriques de suivi
- **Progression** : Nombre de tâches terminées / total
- **Qualité** : Taux de succès des tests automatisés
- **Performance** : Impact mesuré sur les traductions
- **Fonctionnalité** : Scénarios d'usage validés
