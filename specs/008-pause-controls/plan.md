# Plan d'implémentation: Contrôles de Pause Configurables

## Vue d'ensemble

Ce plan décrit l'implémentation des contrôles de pause configurables pour les traductions Ollama. L'approche suit une stratégie incrémentale avec séparation claire frontend/backend.

## Architecture cible

### Backend (Rust/Tauri) - Logique de sécurité matérielle
```
src-tauri/src/translation/common/types.rs
├── PauseSettings (struct)
├── SequentialTranslationRequest (étendu)
└── SequentialProgress (étendu avec pause_time_remaining)

src-tauri/src/translation/ollama/sequential.rs
├── OllamaSequentialSession (étendu avec pause_end_time)
├── SequentialTranslationManager (modifié)
└── Logique de pause matérielle configurable (tokio::sleep)
```

### Frontend (Vue/TypeScript) - Interface utilisateur
```
app/composables/useTauriSetting.ts
└── AppSettings étendu avec section pause

app/components/settings/PauseControls.vue (NOUVEAU)
├── Composant de configuration des pauses
├── Validation temps réel des paramètres
└── Pattern props/emits standard settings

app/pages/translation.vue
└── Compteur d'affichage temps réel (lecture seule)
```

## Phases d'implémentation

### Phase 1: Types et structures de données

#### 1.1 Extension des types communs (Backend)
**Objectif** : Ajouter les structures de données pour la configuration des pauses
**Tâches** :
- Créer structure `PauseSettings` avec `enabled`, `batch_size`, `pause_duration_minutes`
- Étendre `SequentialSession` avec `pause_settings` et `batch_counter`
- Étendre `SequentialTranslationRequest` avec `pause_settings: Option<PauseSettings>`
- Étendre `SequentialProgress` avec `pause_time_remaining`

#### 1.2 Extension des settings utilisateur (Frontend)
**Objectif** : Étendre l'interface des paramètres utilisateur
**Tâches** :
- Ajouter section `pause` dans `AppSettings`
- Définir valeurs par défaut appropriées
- Étendre fonctions de sauvegarde/chargement

### Phase 2: Logique backend

#### 2.1 Modification de la session Ollama
**Objectif** : Intégrer les paramètres de pause dans la session
**Tâches** :
- Étendre `SequentialSession` avec `pause_settings: PauseSettings` et `batch_counter: usize`
- Ajouter `pause_end_time` à `OllamaSequentialSession` pour tracking du temps restant
- Modifier constructeur pour initialiser `SequentialSession` avec `pause_settings`
- Mettre à jour logique de pause pour utiliser `pause_settings.batch_size` au lieu de valeur hardcodée

#### 2.2 Mise à jour du gestionnaire de sessions
**Objectif** : Traiter les nouveaux paramètres de pause
**Tâches** :
- Modifier `start_session` pour traiter `pause_settings` de la requête avec valeurs par défaut
- Mettre à jour `get_progress` pour calculer et retourner `pause_time_remaining` depuis `pause_end_time`
- Remplacer logique de pause hardcodée (500 traductions) par logique utilisant `pause_settings.batch_size`
- Utiliser `pause_settings.pause_duration_minutes` pour calculer la durée de pause

### Phase 3: Composant Settings dédié

#### 3.1 Créer `PauseControls.vue` dans settings
**Objectif** : Développer le composant de configuration des pauses
**Tâches** :
- Créer `app/components/settings/PauseControls.vue`
- Implémenter template avec UCard, UFormField, UCheckbox, UInput
- Ajouter props `settings` et emits `update:*` selon le pattern settings
- Intégrer validation temps réel et correction automatique des valeurs
- Appliquer style cohérent avec les autres composants settings

#### 3.2 Logique de validation et feedback
**Objectif** : Implémenter la logique de validation côté composant
**Tâches** :
- Créer validateurs pour batchSize (1-1000) et pauseDurationMinutes (1-60)
- Implémenter correction automatique des valeurs hors limites
- Ajouter messages d'erreur contextuels en français
- Gérer feedback visuel pour l'utilisateur

#### 3.3 Intégration dans la page settings
**Objectif** : Intégrer le composant dans l'interface settings
**Tâches** :
- Ajouter export dans `app/components/settings/index.ts`
- Intégrer dans la page settings avec v-model pattern
- Tester la persistance automatique des paramètres
- Vérifier la cohérence avec les autres composants settings

### Phase 4: Intégration et tests

#### 4.1 Affichage du compteur dans la page translation
**Objectif** : Ajouter l'affichage du compteur de pause dans la page principale de traduction
**Tâches** :
- Modifier `app/pages/translation.vue` pour afficher le compteur
- Connecter aux données de progression des sessions
- Implémenter formatage "Pause en cours: MM:SS"
- Gérer affichage conditionnel selon l'état des sessions

#### 4.2 Logique de récupération des données
**Objectif** : Connecter le compteur aux données temps réel du backend
**Tâches** :
- Utiliser `sessionProgress` pour récupérer `pause_time_remaining`
- Implémenter mise à jour temps réel via polling existant
- Gérer cas où pauses sont désactivées ou session non en pause
- Optimiser les performances (pas de polling supplémentaire)

#### 4.3 Intégration frontend/backend
**Objectif** : Connecter les paramètres utilisateur à la logique backend
**Tâches** :
- Étendre `startAllTranslations` pour passer `pauseSettings` au backend
- Étendre `handleRetranslateSelected` pour passer `pauseSettings` au backend
- Vérifier compatibilité avec providers multiples (Ollama et RunPod)
- Gérer fallback vers valeurs par défaut si paramètres non disponibles

#### 4.4 Tests et validation
**Objectif** : Assurer la qualité de l'implémentation
**Tâches** :
- Tests unitaires pour logique de pause
- Tests d'intégration frontend/backend
- Validation des cas limites
- Tests de performance

## Dépendances et prérequis

### Dépendances techniques
- ✅ Architecture Tauri + Nuxt existante
- ✅ Système de paramètres utilisateur (Tauri Store)
- ✅ Logique de traduction séquentielle existante
- ✅ Page translation.vue existante

### Prérequis fonctionnels
- ✅ Sessions de traduction Ollama opérationnelles
- ✅ Interface de paramètres utilisateur fonctionnelle
- ✅ Système de notifications existant

## Risques et mitigation

### Risques identifiés
1. **Régression fonctionnelle** : Pause hardcodée supprimée accidentellement
2. **Impact performance** : Polling supplémentaire pour le compteur
3. **Inconsistance UI** : Contrôles non verrouillés correctement
4. **Perte de données** : Paramètres non sauvegardés

### Stratégies de mitigation
1. **Tests automatisés** : Couverture complète de la logique de pause
2. **Monitoring performance** : Mesure de l'impact sur les métriques
3. **Code review** : Validation de l'implémentation
4. **Sauvegarde défensive** : Validation avant sauvegarde

## Métriques de succès

### Indicateurs techniques
- **Performance** : < 5% d'impact sur le temps de traduction
- **Fiabilité** : Taux d'erreur < 0.1%
- **Compatibilité** : Tests passent sur tous les scénarios existants

### Indicateurs fonctionnels
- **UX** : Temps de configuration < 30 secondes (via onglet Settings)
- **Satisfaction** : Feedback utilisateur positif sur séparation settings/interface
- **Adoption** : 80% des utilisateurs configurent les pauses
- **Maintenabilité** : Composant réutilisable et testable indépendamment

## Livrables

### Code
- Extensions des types Rust
- Modifications du gestionnaire de sessions
- Composant Vue étendu
- Tests automatisés

### Documentation
- Spécification fonctionnelle mise à jour
- Guide d'utilisation pour les utilisateurs
- Documentation technique pour développeurs

## Planning prévisionnel

### Durée estimée : 3-4 jours
- **Jour 1** : Types et structures de données (backend)
- **Jour 2** : Logique backend et composant PauseControls
- **Jour 3** : Compteur dans page translation + intégration
- **Jour 4** : Tests et validation complète

### Jalons
1. **Types étendus** : Structures de données opérationnelles
2. **Backend fonctionnel** : Logique de pause configurable
3. **Composant Settings** : PauseControls intégré et fonctionnel
4. **Compteur opérationnel** : Affichage temps réel dans page translation
5. **Intégration validée** : Fonctionnalité complète et testée
