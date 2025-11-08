// Messages in English for LudoLingo

export default {
  app: {
    name: 'LudoLingo',
    description: 'Professional video game localization',
    version: 'Version 1.0'
  },
  nav: {
    home: 'Home',
    projects: 'Projects',
    donation: 'Donate',
    settings: 'Settings',
    help: 'Help',
    about: 'About'
  },
  projects: {
    title: 'Localization Projects',
    open: 'Open a project',
    empty: 'No projects found',
    emptyDescription: 'Open your first localization project to get started',
    loading: 'Loading projects...',
    error: 'Error loading projects'
  },
  games: {
    supported: 'Supported Games',
    scanning: 'Scanning for games...',
    rpgMaker: 'RPG Maker MV/MZ',
    wolfRPG: 'WolfRPG Editor',
    baki: 'Baki Engine',
    comingSoon: 'Coming soon'
  },
  translation: {
    title: 'Translation',
    batch: 'Batch Translation',
    single: 'Single Translation',
    progress: 'Progress',
    status: 'Status',
    pending: 'Pending',
    processing: 'Processing',
    completed: 'Completed',
    failed: 'Failed',
    save: 'Save',
    export: 'Export',
    import: 'Import'
  },
  settings: {
    title: 'Settings',
    subtitle: 'Configure LudoLingo to your preferences',
    ollama: 'Ollama Configuration',
    mode: 'Connection Mode',
    endpoint: 'Endpoint',
    endpointPlaceholder: 'http://localhost',
    port: 'Port',
    portPlaceholder: '11434',
    onlineEndpointPlaceholder: 'https://your-ollama-service.com',
    local: 'Local',
    online: 'Online',
    model: 'Model',
    selectModel: 'Select a model',
    refreshModels: 'Refresh',
    testConnectionFirst: 'Test connection first to load available models',
    test: 'Test Connection',
    reset: 'Reset',
    save: 'Save'
  },
  common: {
    save: 'Save',
    cancel: 'Cancel',
    delete: 'Delete',
    edit: 'Edit',
    add: 'Add',
    remove: 'Remove',
    loading: 'Loading...',
    error: 'Error',
    success: 'Success',
    warning: 'Warning',
    info: 'Information',
    confirm: 'Confirm',
    close: 'Close'
  },
  validation: {
    required: 'This field is required',
    minLength: 'Minimum {count} characters required',
    maxLength: 'Maximum {count} characters allowed',
    invalidPath: 'Invalid path',
    invalidUrl: 'Invalid URL',
    invalidEmail: 'Invalid email address'
  },
  welcome: {
    title: 'Welcome to LudoLingo',
    subtitle: 'Localize your favorite video games with artificial intelligence. Automatic text extraction, batch translation with Ollama, and seamless reinjection.'
  },
  donations: {
    title: 'Support LudoLingo',
    subtitle: 'Your support helps us continue development and keep LudoLingo free',
    description: 'Help us improve LudoLingo with your support',
    donate: 'Make a donation',
    whyDonate: 'Why donate?',
    featureDev: 'Feature development',
    featureDevDesc: 'Adding new game engines and continuous improvements',
    infrastructure: 'Infrastructure',
    infrastructureDesc: 'Servers, storage and technical maintenance',
    localization: 'Localization',
    localizationDesc: 'Support for new languages and improved translations',
    community: 'Community',
    communityDesc: 'User support and documentation',
    chooseAmount: 'Choose an amount',
    customAmount: 'Custom amount',
    enterAmount: 'Enter amount (€)',
    donateAmount: 'Donate {amount}€',
    securePayment: 'Secure payment via Stripe'
  },
  scanning: {
    title: 'Scan a game',
    browse_button: 'Select folder',
    progress_title: 'Scan progress',
    files_processed: 'Files processed',
    entries_found: 'Entries found',
    current_file: 'Current file',
    errors_title: 'Errors encountered',
    help_title: 'How to scan?',
    help_text: 'Select a folder containing an RPG Maker MV or MZ game. LudoLingo will automatically detect the game type and extract all translatable texts.',
    cancel_scan: 'Cancel scan'
  }
} as const
