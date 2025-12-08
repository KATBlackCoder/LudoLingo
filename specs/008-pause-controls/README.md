# Spécification 008: Contrôles de Pause Configurables

## Vue d'ensemble

Cette spécification décrit l'implémentation de contrôles de pause configurables dans l'interface frontend pour les sessions de traduction Ollama. Cette fonctionnalité permet aux utilisateurs de personnaliser le comportement des pauses automatiques pendant les longues sessions de traduction.

## Problème

Actuellement, les pauses automatiques dans les traductions Ollama sont hardcodées avec :
- Une pause toutes les 500 traductions
- Une durée de pause fixe de 12 minutes

Cette approche rigide ne permet pas aux utilisateurs d'adapter le comportement selon leurs besoins et contraintes matérielles.

## Solution proposée

Implémenter une architecture hybride avec séparation claire des responsabilités :

### Backend (Rust/Tauri) - Sécurité matérielle
- **Gère les vraies pauses** : `tokio::time::sleep()` pour protection contre surchauffe
- **Configuration des paramètres** : Taille des batches et durée des pauses via `PauseSettings`
- **Intégration propre** : Utilise le `batch_counter` existant dans `SequentialSession`
- **Robustesse garantie** : Fonctionnement même si frontend crash

### Frontend (Vue/TypeScript) - Interface utilisateur
- **Composant Settings** : `PauseControls.vue` pour configuration persistante
- **Compteur temps réel** : Affichage dans `app/pages/translation.vue` pendant les pauses
- **Validation des paramètres** : Vérifications côté composant settings

## Fonctionnalités utilisateur

1. **Activer/désactiver les pauses automatiques**
2. **Configurer le nombre de traductions avant pause** (défaut: 150)
3. **Configurer la durée de pause** (défaut: 5 minutes)
4. **Visualiser un compteur temps réel** pendant les pauses

## Avantages

- **Personnalisation** : Adaptation aux capacités matérielles de l'utilisateur
- **Transparence** : Visibilité du temps restant pendant les pauses
- **Contrôle** : Possibilité de désactiver complètement les pauses
- **Performance** : Réduction des interruptions inutiles

## Structure de l'implémentation

```
008-pause-controls/
├── README.md                    # Cette description
├── spec.md                     # Spécification détaillée
├── plan.md                     # Plan d'implémentation
├── tasks.md                    # Décomposition des tâches
├── quickstart.md              # Guide d'utilisation rapide
├── research.md                # Recherche et benchmarks
├── contracts/                 # Interfaces TypeScript
│   └── pause-controls.ts      # Contrats et validateurs
└── [app/components/settings/] # Composant à créer
    └── PauseControls.vue      # Configuration des pauses

[app/pages/]                   # Modification nécessaire
└── translation.vue            # Ajout du compteur de pause
```

## Technologies concernées

- **Frontend** : Vue 3 + TypeScript + Nuxt UI
- **Backend** : Rust + Tauri
- **Stockage** : Tauri Store (paramètres utilisateur)

## Critères d'acceptation

### Backend - Structures de données
- [ ] `PauseSettings` créée dans `common/types.rs`
- [ ] `SequentialSession` étendue avec `pause_settings` et `batch_counter`
- [ ] `SequentialTranslationRequest` étendue avec `pause_settings`
- [ ] `SequentialProgress` étendue avec `pause_time_remaining`
- [ ] `OllamaSequentialSession` mise à jour pour utiliser `SequentialSession` étendu

### Composant Settings (PauseControls.vue)
- [ ] Interface de configuration accessible dans settings
- [ ] Contrôles toujours actifs (pas de verrouillage pendant traduction)
- [ ] Paramètres sauvegardés automatiquement via Tauri Store
- [ ] Valeurs par défaut appropriées (150 traductions, 5 minutes)
- [ ] Validation temps réel des valeurs saisies (min/max)
- [ ] Correction automatique des valeurs invalides

### Compteur dans la page translation
- [ ] Compteur visible uniquement pendant les pauses actives
- [ ] Format "Pause en cours: MM:SS" correct
- [ ] Mise à jour temps réel via données backend
- [ ] Masqué si pauses désactivées ou session non en pause

### Intégration globale
- [ ] Paramètres settings utilisés par les traductions
- [ ] Compatibilité avec traductions simples et retraductions
- [ ] Architecture backend/frontend respectée

## Risques et considérations

- **Performance** : Impact minimal sur les performances de traduction
- **UX** : Interface intuitive et non intrusive
- **Persistance** : Sauvegarde fiable des préférences utilisateur
- **Compatibilité** : Fonctionnement avec tous les providers (Ollama, RunPod)

## Métriques de succès

- Temps de configuration < 30 secondes
- Réduction des interruptions manuelles de 80%
- Satisfaction utilisateur > 90%
- Zéro régression sur fonctionnalités existantes
