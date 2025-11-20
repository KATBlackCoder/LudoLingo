// WolfRPG Tools Frontend Types Contract
// Defines the TypeScript types and interfaces for frontend components

export type WolfRpgProjectType = 'needs_ubwolf' | 'dump_exists' | 'invalid' | 'unknown'

export interface WolfRpgProjectInfo {
  /** Type of WolfRPG project */
  type: WolfRpgProjectType
  /** Human-readable description */
  description: string
  /** Whether the project is ready for translation */
  readyForTranslation: boolean
  /** Whether extraction is needed */
  needsExtraction: boolean
  /** Path to the project */
  path: string
}

export interface WolfRpgToolsState {
  /** Whether tools have been validated */
  toolsValidated: boolean
  /** Whether Wine is installed (Linux only) */
  wineInstalled: boolean
  /** Current project type */
  projectType: WolfRpgProjectType
  /** Whether extraction is in progress */
  extractionInProgress: boolean
  /** Whether injection is in progress */
  injectionInProgress: boolean
  /** Extraction progress (0-100) */
  extractionProgress: number
  /** Injection progress (0-100) */
  injectionProgress: number
  /** Current operation logs */
  currentLogs: string[]
  /** Last operation result */
  lastResult?: WolfRpgOperationResult
}

export interface WolfRpgOperationResult {
  /** Whether the operation succeeded */
  success: boolean
  /** Operation output/logs */
  output?: string
  /** Error message if failed */
  error?: string
  /** Duration in milliseconds */
  duration: number
  /** Additional metadata */
  metadata?: Record<string, any>
}

export interface WolfRpgToolsConfig {
  /** Path to the tools directory */
  toolsDirectory: string
  /** Whether to auto-install Wine on Linux */
  autoInstallWine: boolean
  /** Whether to show detailed logs */
  showDetailedLogs: boolean
  /** Timeout for operations in seconds */
  operationTimeout: number
}

// Component Props Interfaces

export interface WolfRpgToolsPanelProps {
  /** Current project information */
  project: WolfRpgProjectInfo | null
  /** Tools state */
  toolsState: WolfRpgToolsState
  /** Configuration */
  config: WolfRpgToolsConfig
  /** Callback when extraction is requested */
  onExtract?: () => Promise<void>
  /** Callback when injection is requested */
  onInject?: () => Promise<void>
  /** Callback when tools validation is requested */
  onValidateTools?: () => Promise<void>
  /** Callback when Wine setup is requested */
  onSetupWine?: () => Promise<void>
}

export interface WolfRpgStatusBadgeProps {
  /** Project type to display */
  projectType: WolfRpgProjectType
  /** Whether tools are validated */
  toolsValidated: boolean
  /** Custom CSS class */
  class?: string
}

export interface WolfRpgExtractionDialogProps {
  /** Whether the dialog is open */
  open: boolean
  /** Current progress (0-100) */
  progress: number
  /** Current logs */
  logs: string[]
  /** Whether the operation can be cancelled */
  canCancel: boolean
  /** Callback when cancel is requested */
  onCancel?: () => void
  /** Callback when dialog should close */
  onClose?: () => void
}

export interface WolfRpgInjectionDialogProps extends WolfRpgExtractionDialogProps {
  /** Number of files that will be modified */
  filesToModify: number
  /** Whether a backup will be created */
  backupWillBeCreated: boolean
}

export interface WolfRpgToolsConfigProps {
  /** Current configuration */
  config: WolfRpgToolsConfig
  /** Callback when config changes */
  onConfigChange: (config: WolfRpgToolsConfig) => void
  /** Whether the component is in read-only mode */
  readonly?: boolean
}

// Store Actions Interface

export interface WolfRpgStoreActions {
  /** Detect project type */
  detectProjectType: (path: string) => Promise<WolfRpgProjectInfo>
  /** Validate tools availability */
  validateTools: (toolsDir: string) => Promise<boolean>
  /** Setup Wine environment */
  setupWine: () => Promise<boolean>
  /** Extract data with UberWolf */
  extractData: (gamePath: string, toolsDir: string) => Promise<WolfRpgOperationResult>
  /** Inject data with WolfTL */
  injectData: (gamePath: string, toolsDir: string) => Promise<WolfRpgOperationResult>
  /** Process complete project workflow */
  processProject: (gamePath: string, toolsDir: string) => Promise<WolfRpgOperationResult>
  /** Update tools state */
  updateState: (updates: Partial<WolfRpgToolsState>) => void
  /** Reset state */
  resetState: () => void
}

// Composable Interface

export interface UseWolfRpgToolsReturn {
  /** Current state */
  state: Readonly<WolfRpgToolsState>
  /** Available actions */
  actions: WolfRpgStoreActions
  /** Computed properties */
  computed: {
    /** Whether extraction is possible */
    canExtract: Readonly<boolean>
    /** Whether injection is possible */
    canInject: Readonly<boolean>
    /** Whether the project is fully ready */
    isReady: Readonly<boolean>
    /** Current status message */
    statusMessage: Readonly<string>
  }
}
