# Démarrage rapide : Détection automatique des textes

## Prérequis
- ✅ `cjk-regex` déjà installé dans le projet
- ✅ Configuration langues CJK → non-CJK dans les paramètres

## Utilisation
1. **Configurer les langues** : Source CJK (ja, zh, ko) → Cible non-CJK (fr, en, es, etc.)
2. **Extraire les textes** d'un projet de jeu comme d'habitude
3. **L'enrichissement automatique** traite les textes avant sauvegarde DB
4. **Vérifier les résultats** : Certains textes sont automatiquement marqués comme "traduits"

## Vérification
La détection automatique s'effectue de manière transparente :

- **Textes sans CJK** : Automatiquement marqués comme "traduits" ✅
- **Textes avec CJK** : Restent "à traduire" ❌
- **Configuration non-CJK** : Pas de détection automatique ⚠️

## Logique d'enrichissement
- ✅ **Enrichi automatiquement** : Textes extraits sans CJK → marqués 'translated' avant DB
- ❌ **Préservé** : Textes extraits avec CJK → restent 'extracted' (nécessitent traduction)
- ⚠️ **Workflow préservé** : L'extraction fonctionne normalement, enrichie d'intelligence

---

**Guide mis à jour** : Décembre 2025
**Dépendances** : cjk-regex
