# Research: LudoLingo Game Localization Core

**Date**: 2025-11-06
**Plan**: [specs/001-game-localization/plan.md](specs/001-game-localization/plan.md)

## Research Questions & Findings

### Parser Game Formats (RPG Maker, WolfRPG, Baki)

**Question**: Comment parser les formats de fichiers des différents moteurs de jeu pour extraire les textes ?

**Findings**:
- **RPG Maker**: Format JSON/MapXXX.json pour les cartes, System.json pour les termes système. Textes dans les events et les messages.
- **WolfRPG**: Format binaire propriétaire, nécessite reverse engineering ou outils existants comme WolfTrans.
- **Baki**: Format similaire à RPG Maker VX Ace, basé sur Ruby.

**Decision**: Implémenter des parsers spécialisés par format avec fallback générique pour formats inconnus.

**Alternatives considered**: Utiliser des outils externes (non retenu pour rester offline), bibliothèque unique (non viable car formats trop différents).

---

### Ollama Integration Patterns

**Question**: Quelle est la meilleure approche pour intégrer Ollama dans une application Tauri ?

**Findings**:
- Ollama peut tourner localement sur le même appareil
- API REST locale sur port 11434 par défaut
- Client Rust `ollama-rs` fournit une intégration native
- Gestion des timeouts et reconnexions nécessaires

**Decision**: Utiliser `ollama-rs` avec configuration automatique de l'endpoint local.

**Alternatives considered**: Wrapper HTTP direct (plus complexe), embedding Ollama (trop lourd).

---

### SQLite Schema Design for Localization

**Question**: Comment structurer les tables translation et glossary pour optimiser les performances ?

**Findings**:
- Table translation: id, project_id, file_path, source_text, translated_text, status, context
- Table glossary: id, source_term, translated_term, context, frequency
- Indexes sur project_id, status, et termes fréquents
- Triggers pour mise à jour automatique des fréquences

**Decision**: Schéma normalisé avec indexes optimisés et contraintes d'intégrité.

**Alternatives considered**: Schéma dénormalisé (plus rapide mais moins maintenable), JSON storage (moins performant pour les recherches).

---

### Batch Processing Optimization

**Question**: Comment optimiser le traitement par lots de 1-100 éléments simultanément ?

**Findings**:
- Utiliser async/await avec tokio pour parallélisation
- Limiter la concurrence à 10-20 requêtes simultanées vers Ollama
- File d'attente avec priorité (erreurs d'abord, puis nouveaux textes)
- Cache local des traductions récentes

**Decision**: Pool de threads limité avec file d'attente priorisée et cache LRU.

**Alternatives considered**: Traitement séquentiel (trop lent), parallélisation complète (risque de surcharge).

---

### UI State Management for Large Datasets

**Question**: Comment gérer l'état UI avec potentiellement milliers de textes à traduire ?

**Findings**:
- Pinia stores avec pagination virtuelle
- Lazy loading des textes non visibles
- Filtrage côté client avec indexation
- Mise à jour optimiste pour les actions utilisateur

**Decision**: Pinia avec virtual scrolling et filtrage client optimisé.

**Alternatives considered**: Chargement complet (mémoire intensive), pagination serveur (complexité inutile pour données locales).

---

### Error Recovery Strategies

**Question**: Quelles stratégies de récupération d'erreur pour l'extraction et l'injection de fichiers ?

**Findings**:
- Backup automatique avant modification
- Points de contrôle pour reprise partielle
- Validation des fichiers modifiés
- Rollback automatique en cas d'erreur

**Decision**: Système de backup avec validation et rollback automatique.

**Alternatives considered**: Pas de backup (risqué), backup manuel (oubliable).

---

### Testing Strategy for Game Parsers

**Question**: Comment tester les parsers de formats de jeu propriétaires ?

**Findings**:
- Tests avec fichiers de jeu d'exemple (anonymisés)
- Mocks pour formats complexes
- Tests de régression avec captures d'écran
- Validation croisée avec outils externes

**Decision**: Suite de tests avec fichiers de test et validation par outils externes.

**Alternatives considered**: Tests manuels uniquement (non maintenable), tests sans données réelles (insuffisant).

---

### Memory Management for Large Game Files

**Question**: Comment gérer la mémoire pour les gros fichiers de jeu (jusqu'à 100MB+) ?

**Findings**:
- Streaming parsing au lieu du chargement complet
- Traitement par chunks avec libération mémoire
- Cache intelligent des données fréquemment utilisées
- Monitoring de l'usage mémoire

**Decision**: Parsing streaming avec libération progressive de la mémoire.

**Alternatives considered**: Chargement complet (limite la taille des fichiers), swap disque (performance dégradée).

## Technical Decisions Summary

1. **Architecture**: Tauri + Nuxt avec séparation claire frontend/backend
2. **Base de données**: SQLite avec schéma optimisé pour la localisation
3. **Traduction**: Ollama local avec batch processing optimisé
4. **Parsers**: Spécialisés par format avec fallback générique
5. **UI**: Virtual scrolling avec filtrage optimisé
6. **Tests**: TDD avec couverture >80%, tests spécialisés pour parsers
7. **Performance**: Streaming parsing, concurrence limitée, cache intelligent

## Next Steps

Toutes les décisions techniques sont prises. Prêt pour la Phase 1 (Design & Contracts).
