# Tâches : Détection automatique des textes déjà traduits

## Tâches essentielles

### Tâche 1 : Vérification des dépendances
**Estimation** : 15 min
- [x] `cjk-regex` déjà installé (vérifier présence dans package.json)
- [x] Vérifier compatibilité Tauri (tests Node.js réussis)
- [x] Tester imports de base et compatibilité avec code existant (tests TypeScript réussis)

### Tâche 2 : Composable useAutoTranslationDetection
**Estimation** : 1h
- [x] Créer `app/composables/translation/useAutoTranslationDetection.ts`
- [x] Implémenter `useAutoTranslationDetection()` composable
- [x] Ajouter fonction `applyAutoTranslationDetection(texts: TextEntry[]): Promise<TextEntry[]>`
- [x] Intégrer accès aux settings pour récupérer les langues
- [x] Appliquer logique CJK → non-CJK avec détection cjk-regex
- [x] Ajouter logs informatifs et métriques
- [x] Tests unitaires : logique CJK avec différents paramètres (5/5 tests réussis)

### Tâche 3 : Enrichissement de updateProjectTexts
**Estimation** : 45 min
- [x] Modifier `app/stores/projects.ts` → `updateProjectTexts()`
- [x] Importer le composable `useAutoTranslationDetection`
- [x] Insérer `applyAutoTranslationDetection()` AVANT `createBulkTextEntries()`
- [x] Préserver le workflow d'injection DB existant (pas de remplacement)
- [x] Maintenir logique de rollback avec textes enrichis
- [x] Logs informatifs pour le développement (intégrés dans composable)

### Tâche 4 : Tests fonctionnels
**Estimation** : 1h
- [x] Tester extraction avec textes sans CJK (status: NotTranslated → Translated)
- [x] Tester extraction avec textes contenant CJK (rester NotTranslated)
- [x] Tester langues non-CJK → CJK (pas de détection automatique)
- [x] Tester langues non-CJK → non-CJK (pas de détection automatique)
- [x] Vérifier sauvegarde correcte en DB avec statuts mis à jour
- [x] Tester rollback en cas d'erreur DB (fonctionne correctement)

### Tâche 5 : Optimisations et métriques
**Estimation** : 30 min
- [ ] Ajouter métriques de performance (< 1ms par texte)
- [ ] Logs détaillés pour débogage
- [ ] Statistiques d'efficacité (pourcentage de textes auto-marqués)

---

**Version finale** : 3h 30min total
**Tâches** : 5 essentielles
**Approche** : Composable dédié + Enrichissement intelligent du workflow d'injection DB

---

**Mis à jour le** : Décembre 2025
