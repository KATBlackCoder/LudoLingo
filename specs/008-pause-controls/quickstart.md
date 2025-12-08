# Guide de démarrage rapide: Contrôles de Pause Configurables

## Vue d'ensemble

Les contrôles de pause configurables permettent de personnaliser le comportement des pauses automatiques pendant les longues sessions de traduction Ollama.

## Configuration rapide

### 1. Accès aux paramètres
1. Allez dans **Paramètres** de l'application
2. Localisez la section **"Paramètres de pause automatique"**
3. Configurez selon vos besoins

### 2. Paramètres recommandés

#### Pour matériel performant
```
□ Activer les pauses: ✅ Coché
Traductions avant pause: 200
Durée de pause: 3 minutes
```

#### Pour sessions courtes
```
□ Activer les pauses: ❌ Décoché
```

#### Pour matériel limité
```
□ Activer les pauses: ✅ Coché
Traductions avant pause: 100
Durée de pause: 10 minutes
```

## Utilisation

### Démarrage d'une traduction
1. Configurez les paramètres de pause
2. Cliquez sur **"Commencer la traduction"**
3. Les paramètres sont automatiquement appliqués

### Pendant la traduction
- **Paramètres** : Accessibles à tout moment dans l'onglet Settings
- **Compteur visible** : Apparaît dans la page de traduction pendant les pauses
- **Format** : "Pause en cours: MM:SS" (affichage en temps réel)
- **Sécurité** : Les pauses sont gérées par le backend même si l'interface crash

### Modification des paramètres
- **À tout moment** : Paramètres modifiables dans Settings, même pendant traduction
- **Sauvegarde automatique** : Modifications sauvegardées immédiatement
- **Validation** : Valeurs corrigées automatiquement si invalides

## Cas d'usage courants

### Traduction de gros projets
```
Paramètres: 150 traductions, 5 minutes
Résultat: Pause toutes les 2-3 heures selon la vitesse
```

### Sessions de test
```
Paramètres: Désactiver les pauses
Résultat: Traduction continue sans interruption
```

### Matériel sensible à la chaleur
```
Paramètres: 100 traductions, 8 minutes
Résultat: Pauses plus fréquentes et longues
```

## Dépannage

### Le compteur n'apparaît pas
- Vérifiez que les pauses sont activées
- Vérifiez qu'une traduction est en cours
- Attendez le déclenchement automatique de la pause

### Paramètres non sauvegardés
- Vérifiez les permissions d'écriture
- Redémarrez l'application si nécessaire
- Vérifiez l'espace disque disponible

### Traduction sans pause
- Vérifiez que "Activer les pauses" est coché
- Vérifiez que le nombre de traductions est > 0
- Vérifiez les logs pour erreurs de configuration

## Limites et contraintes

- **Minimum** : 1 traduction avant pause
- **Maximum** : 1000 traductions avant pause
- **Durée** : 1 à 60 minutes par pause
- **Sauvegarde** : Paramètres utilisateur uniquement
- **Scope** : Appliqué à toutes les traductions Ollama

## Support

Pour tout problème :
1. Vérifiez cette documentation
2. Consultez les logs de l'application
3. Ouvrez un ticket avec les détails de configuration
