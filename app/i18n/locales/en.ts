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
    settings: 'Settings',
    help: 'Help',
    about: 'About'
  },
  projects: {
    title: 'Localization Projects',
    create: 'New Project',
    scan: 'Scan Games',
    empty: 'No projects found',
    emptyDescription: 'Create your first localization project to get started',
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
    language: 'Language',
    theme: 'Theme',
    ollama: 'Ollama Configuration',
    endpoint: 'Endpoint',
    port: 'Port',
    local: 'Local',
    online: 'Online',
    model: 'Model',
    test: 'Test Connection',
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
  donations: {
    title: 'Support LudoLingo',
    description: 'Help us improve LudoLingo with your support',
    donate: 'Make a donation'
  }
} as const
