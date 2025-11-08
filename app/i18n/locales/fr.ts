// Messages en français pour LudoLingo

export default {
  app: {
    name: 'LudoLingo',
    description: 'Localisation de jeux vidéo professionnelle',
    version: 'Version 1.0'
  },
  nav: {
    home: 'Accueil',
    projects: 'Projets',
    donation: 'Dons',
    settings: 'Paramètres',
    help: 'Aide',
    about: 'À propos'
  },
  projects: {
    title: 'Projets de Localisation',
    open: 'Ouvrir un projet',
    empty: 'Aucun projet trouvé',
    emptyDescription: 'Ouvrez votre premier projet de localisation pour commencer',
    loading: 'Chargement des projets...',
    error: 'Erreur lors du chargement des projets'
  },
  games: {
    supported: 'Jeux Supportés',
    scanning: 'Recherche de jeux...',
    rpgMaker: 'RPG Maker MV/MZ',
    wolfRPG: 'WolfRPG Editor',
    baki: 'Baki Engine',
    comingSoon: 'Bientôt disponible'
  },
  translation: {
    title: 'Traduction',
    batch: 'Traduction par lots',
    single: 'Traduction simple',
    progress: 'Progression',
    status: 'Statut',
    pending: 'En attente',
    processing: 'En cours',
    completed: 'Terminé',
    failed: 'Échec',
    save: 'Sauvegarder',
    export: 'Exporter',
    import: 'Importer'
  },
  settings: {
    title: 'Paramètres',
    subtitle: 'Configurez LudoLingo selon vos préférences',
    ollama: 'Configuration Ollama',
    mode: 'Mode de connexion',
    endpoint: 'Point de terminaison',
    endpointPlaceholder: 'http://localhost',
    port: 'Port',
    portPlaceholder: '11434',
    onlineEndpointPlaceholder: 'https://votre-service-ollama.com',
    local: 'Local',
    online: 'En ligne',
    model: 'Modèle',
    selectModel: 'Sélectionner un modèle',
    refreshModels: 'Actualiser',
    testConnectionFirst: 'Testez d\'abord la connexion pour charger les modèles',
    test: 'Tester la connexion',
    reset: 'Réinitialiser',
    save: 'Enregistrer'
  },
  common: {
    save: 'Enregistrer',
    cancel: 'Annuler',
    delete: 'Supprimer',
    edit: 'Modifier',
    add: 'Ajouter',
    remove: 'Retirer',
    loading: 'Chargement...',
    error: 'Erreur',
    success: 'Succès',
    warning: 'Avertissement',
    info: 'Information',
    confirm: 'Confirmer',
    close: 'Fermer'
  },
  validation: {
    required: 'Ce champ est requis',
    minLength: 'Minimum {count} caractères requis',
    maxLength: 'Maximum {count} caractères autorisés',
    invalidPath: 'Chemin invalide',
    invalidUrl: 'URL invalide',
    invalidEmail: 'Adresse email invalide'
  },
  welcome: {
    title: 'Bienvenue sur LudoLingo',
    subtitle: 'Localisez vos jeux vidéo préférés avec l\'intelligence artificielle. Extraction automatique des textes, traduction par lots avec Ollama, et réinjection transparente.'
  },
  donations: {
    title: 'Soutenir LudoLingo',
    subtitle: 'Votre soutien nous aide à continuer le développement et à maintenir LudoLingo gratuit',
    description: 'Aidez-nous à améliorer LudoLingo avec votre soutien',
    donate: 'Faire un don',
    whyDonate: 'Pourquoi faire un don ?',
    featureDev: 'Développement de fonctionnalités',
    featureDevDesc: 'Ajout de nouveaux moteurs de jeu et améliorations continues',
    infrastructure: 'Infrastructure',
    infrastructureDesc: 'Serveurs, stockage et maintenance technique',
    localization: 'Localisation',
    localizationDesc: 'Support de nouvelles langues et amélioration des traductions',
    community: 'Communauté',
    communityDesc: 'Support utilisateur et documentation',
    chooseAmount: 'Choisir un montant',
    customAmount: 'Montant personnalisé',
    enterAmount: 'Saisir un montant (€)',
    donateAmount: 'Faire un don de {amount}€',
    securePayment: 'Paiement sécurisé via Stripe'
  },
  scanning: {
    title: 'Scanner un jeu',
    browse_button: 'Sélectionner un dossier',
    progress_title: 'Progression du scan',
    files_processed: 'Fichiers traités',
    entries_found: 'Entrées trouvées',
    current_file: 'Fichier actuel',
    errors_title: 'Erreurs rencontrées',
    help_title: 'Comment scanner ?',
    help_text: 'Sélectionnez un dossier contenant un jeu RPG Maker MV ou MZ. LudoLingo détectera automatiquement le type de jeu et extraira tous les textes traduisibles.',
    cancel_scan: 'Annuler le scan'
  }
} as const
