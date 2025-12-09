# Checklist : Scan manuel des textes déjà traduits

## Fonctionnalités essentielles

### Installation
- [x] `cjk-regex` déjà installé (présent dans package.json)
- [x] Vérifier import possible dans le nouveau composable (tests réussis)

### Composable useAutoTranslationDetection
- [x] `useAutoTranslationDetection()` dans `app/composables/translation/useAutoTranslationDetection.ts`
- [x] Exposition de `applyAutoTranslationDetection(texts: TextEntry[]): Promise<TextEntry[]>`
- [x] Utilisation de `useSettings()` pour accéder aux langues source/cible
- [x] Logique CJK → non-CJK respectée uniquement pour cette configuration
- [x] Nettoyage des placeholders `[CODE_*]` fonctionnel
- [x] Acceptation de tous textes non-CJK (symboles "!!!", "??" inclus)
- [x] Détection cjk-regex pour caractères CJK
- [x] Copie automatique de source_text vers translated_text pour statuts 'Translated'
- [x] Logs informatifs et métriques de performance
- [x] Gestion d'erreurs et retours des textes avec statuts mis à jour

### Intégration dans updateProjectTexts
- [ ] Modification de `updateProjectTexts()` dans `app/stores/projects.ts`
- [ ] Appel à `applyAutoTranslationDetection()` avant sauvegarde DB
- [ ] Traitement automatique lors de chaque extraction de projet
- [ ] Maintien de la logique de rollback en cas d'erreur
- [ ] Logs informatifs pour débogage et métriques

### Tests fonctionnels
- [x] Test extraction avec textes sans CJK (status: NotTranslated → Translated automatiquement)
- [x] Test extraction avec textes contenant CJK (rester NotTranslated)
- [x] Test configuration non-CJK → CJK (pas de détection automatique)
- [x] Test configuration non-CJK → non-CJK (pas de détection automatique)
- [x] Vérification sauvegarde correcte en DB avec statuts mis à jour
- [x] Test rollback en cas d'erreur DB (statuts non modifiés)

### Nettoyage des textes
- [ ] Suppression de tous les placeholders `[CODE_*]`
- [ ] Gestion des espaces multiples et trimming
- [ ] Préservation du contenu linguistique
- [ ] Performance : < 1ms par texte nettoyé

### Logique de décision
- [ ] Application uniquement pour CJK → non-CJK
- [ ] Langues CJK supportées : ja, zh, ko
- [ ] Langues cibles supportées : fr, en, es, de, it, pt
- [ ] Rejet automatique des autres combinaisons

### Intégration système
- [x] Composable `useAutoTranslationDetection` enrichit `updateProjectTexts()`
- [x] `applyAutoTranslationDetection()` appelé AVANT `createBulkTextEntries()`
- [x] Workflow d'injection DB préservé (enrichi, pas remplacé)
- [x] Mise à jour DB directe via plugin SQL existant avec textes traités
- [x] Synchronisation automatique avec store Pinia lors de l'extraction
- [x] Gestion d'erreurs préservée (rollback avec textes enrichis)

## Qualité et performance

### Précision
- [ ] > 95% de décisions correctes sur jeux de test
- [ ] < 5% de faux positifs nécessitant correction
- [ ] Gestion correcte des cas limites

### Performance
- [ ] < 2 secondes pour 1000 textes
- [ ] < 50MB utilisation mémoire supplémentaire
- [ ] Impact CPU minimal (< 10% utilisation)
- [ ] Pas de blocage de l'interface utilisateur

### Fiabilité
- [ ] Continuation du traitement en cas d'échec partiel
- [ ] Logs détaillés pour débogage
- [ ] Récupération automatique des pannes

## Interface utilisateur

### Transparence
- [ ] Logs console informatifs pendant l'extraction automatique
- [ ] Métriques de détection automatique dans les logs
- [ ] Messages d'erreur explicites si échec de détection
- [ ] Indicateurs visuels dans l'interface montrant statuts auto-détectés

### Correction manuelle
- [ ] Possibilité de modifier les décisions automatiques via interface existante
- [ ] Interface de traduction permet override des statuts auto-détectés
- [ ] Historique des modifications préservé
- [ ] Pas de perte de données lors de corrections manuelles

## Sécurité et conformité

### Validation des entrées
- [ ] Sanitisation des textes avant traitement
- [ ] Protection contre les injections
- [ ] Limites de taille des textes traités
- [ ] Timeouts pour éviter les blocages

### Gestion des données
- [ ] Pas de fuite de données sensibles
- [ ] Conformité RGPD pour les analyses
- [ ] Anonymisation des données de debug
- [ ] Nettoyage automatique des caches

## Tests et validation

### Tests unitaires
- [ ] Couverture > 90% des fonctions utilitaires
- [ ] Tests pour tous les cas nominaux
- [ ] Tests pour tous les cas d'erreur
- [ ] Tests de performance automatisés

### Tests d'intégration
- [ ] Extraction complète avec marquage auto
- [ ] Validation cohérence DB vs Store
- [ ] Tests avec différents types de jeux
- [ ] Tests de charge (1000+ textes)

### Tests utilisateur
- [ ] Validation sur jeux réels représentatifs
- [ ] Feedback utilisateur recueilli
- [ ] Métriques d'acceptation > 80%
- [ ] Documentation utilisateur claire

## Maintenance et évolution

### Monitoring
- [ ] Métriques de performance collectées
- [ ] Alertes sur anomalies détectées
- [ ] Logs d'audit complets
- [ ] Tableau de bord de supervision

### Évolutivité
- [ ] Architecture modulaire pour extensions
- [ ] Configuration externe des langues
- [ ] API pour ajouts de langues futures
- [ ] Migration facile vers nouvelles versions

### Documentation
- [ ] Guide développeur complet
- [ ] Guide utilisateur accessible
- [ ] FAQ pour les cas courants
- [ ] Exemples de code documentés

## Critères d'acceptation globaux

### Fonctionnels
- [ ] Réduction de 20-50% de la charge de traduction grâce à l'enrichissement automatique
- [ ] Précision > 95% sur les décisions automatiques de marquage
- [ ] Transparence complète des actions (logs du composable et métriques)
- [ ] Enrichissement transparent du workflow d'extraction existant

### Techniques
- [ ] Performance acceptable (< 2s pour 1000 textes)
- [ ] Fiabilité > 99% (pas d'interruption de service)
- [ ] Maintenabilité : composable bien structuré et testé
- [ ] Sécurité : pas de vulnérabilités introduites

### Utilisateur
- [ ] Amélioration mesurable de l'expérience
- [ ] Formation minimale requise
- [ ] Support disponible pour questions
- [ ] Satisfaction utilisateur > 80%

---

**Checklist créée le** : Décembre 2025
**Révision** : À valider avant implémentation
**Responsable validation** : Équipe QA
