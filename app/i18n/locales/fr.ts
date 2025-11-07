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
    settings: 'Paramètres',
    help: 'Aide',
    about: 'À propos'
  },
  projects: {
    title: 'Projets de Localisation',
    create: 'Nouveau Projet',
    scan: 'Scanner des Jeux',
    empty: 'Aucun projet trouvé',
    emptyDescription: 'Créez votre premier projet de localisation pour commencer',
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
    language: 'Langue',
    theme: 'Thème',
    ollama: 'Configuration Ollama',
    endpoint: 'Point de terminaison',
    port: 'Port',
    local: 'Local',
    online: 'En ligne',
    model: 'Modèle',
    test: 'Tester la connexion',
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
  donations: {
    title: 'Soutenir LudoLingo',
    description: 'Aidez-nous à améliorer LudoLingo',
    donate: 'Faire un don'
  }
} as const
