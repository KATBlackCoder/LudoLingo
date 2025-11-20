# Guide de Démarrage Rapide: Intégration Automatique WolfRPG

## Vue d'Ensemble

L'intégration WolfRPG dans LudoLingo est **entièrement automatique**. Vous n'avez pas besoin de connaître les outils techniques - sélectionnez simplement votre projet WolfRPG et commencez à traduire !

## Prérequis Automatiques

### Système
- **Windows**: Fonctionne nativement (rien à installer)
- **Linux**: Wine sera installé automatiquement si nécessaire

### Outils Externes
**Tout est automatique !** LudoLingo télécharge et configure UberWolf et WolfTL automatiquement lors du premier usage.

### Configuration Initiale
1. Lancez LudoLingo
2. Lors du premier projet WolfRPG, LudoLingo installera automatiquement :
   - Les outils UberWolf et WolfTL
   - Wine sur Linux (si nécessaire)
3. C'est tout - vous êtes prêt !

## Workflow Automatique

### C'est Simple Comme Bonjour !

1. **Sélectionnez votre projet WolfRPG**
   ```
   Cliquez "Scanner un projet" → Choisissez le dossier avec Game.exe
   ```

2. **LudoLingo fait tout automatiquement**
   - Détecte le type de projet (chiffré ou natif)
   - Installe les outils si nécessaire
   - Traite les fichiers avec UberWolf/WolfTL
   - Présente les textes extraits

3. **Traduisez normalement**
   - Interface de traduction familière
   - Tous vos outils habituels
   - Aucun changement dans votre workflow

4. **Exportez le jeu final**
   ```
   Cliquez "Exporter vers WolfRPG" → Injection automatique → Jeu terminé !
   ```

### Types de Projets Supportés

**Tout type de projet WolfRPG fonctionne :**
- ✅ Projets avec fichiers chiffrés (.wolf, .data, .pak, etc.)
- ✅ Projets avec fichiers natifs (.dat, .mps)
- ✅ Projets déjà extraits (avec dump/)
- ✅ Projets Pro Editor avec protection

**LudoLingo détecte automatiquement et gère tout !**

## Comment Ça Marche

### Backend Automatique (Transparent)

**LudoLingo gère automatiquement :**
- Détection du type de projet WolfRPG
- Téléchargement et validation des outils externes
- Exécution conditionnelle d'UberWolf (si fichiers chiffrés)
- Exécution systématique de WolfTL (extraction)
- Gestion Wine sur Linux
- Injection finale des traductions

### Interface Utilisateur (Familière)

**Vous ne voyez que :**
- Sélection du projet (comme d'habitude)
- Interface de traduction (inchangée)
- Bouton d'export final (nouveau mais simple)

**Rien de complexe - tout est automatique !**

## Support et Questions

### Tout est Automatique !
**La plupart des problèmes se résolvent automatiquement :**
- Installation des outils manquants
- Configuration de Wine sur Linux
- Validation des fichiers
- Retry des opérations échouées

### Besoin d'Aide ?
**Si vous rencontrez un problème :**
1. **Vérifiez les messages d'erreur** - LudoLingo fournit des instructions détaillées
2. **Redémarrez LudoLingo** - Parfois une simple relance résout le problème
3. **Contactez le support** avec :
   - Version de LudoLingo
   - Système d'exploitation
   - Message d'erreur complet

### Ressources Utiles
- [GitHub Issues LudoLingo](https://github.com/KATBlackCoder/LudoLingo/issues) - Pour les bugs
- [UberWolf GitHub](https://github.com/Sinflower/UberWolf) - Outil de déchiffrement
- [WolfTL GitHub](https://github.com/Sinflower/WolfTL) - Outil d'extraction/injection

**L'intégration WolfRPG est conçue pour être invisible - si vous la voyez, c'est qu'elle fonctionne ! 🎉**
