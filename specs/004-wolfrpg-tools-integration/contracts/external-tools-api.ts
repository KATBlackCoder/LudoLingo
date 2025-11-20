// External Tools API Contract
// Documents the expected behavior and APIs of UberWolf and WolfTL tools

// =============================================================================
// UBERWOLF CLI API SPECIFICATION
// =============================================================================

export interface UberWolfCliOptions {
  /** Project path containing Game.exe */
  'project-path': string
  /** Output path for extracted JSON files */
  'output-path': string
  /** Extract all available data */
  'extract-all': boolean
  /** Verbose output */
  'verbose'?: boolean
  /** Force overwrite existing files */
  'force'?: boolean
}

export interface UberWolfCliResult {
  /** Exit code (0 = success) */
  exitCode: number
  /** Standard output */
  stdout: string
  /** Error output */
  stderr: string
  /** Whether extraction succeeded */
  success: boolean
  /** List of extracted files */
  extractedFiles?: string[]
  /** Extraction statistics */
  stats?: {
    totalFiles: number
    extractedFiles: number
    skippedFiles: number
    errors: number
  }
}

// Expected UberWolf CLI behavior:
// - Input: Drag & drop Game.exe or folder containing encrypted files
// - Output: Decrypted .dat/.mps files in original location
// - Exit codes: 0 = success, non-zero = error
// - Error handling: Detailed error messages in stderr
// - Note: UberWolf decrypts files in-place, WolfTL is then used to extract JSON

// =============================================================================
// WOLFTL API SPECIFICATION
// =============================================================================

export interface WolfTlOptions {
  /** Project path containing Game.exe */
  'project-path': string
  /** Input path containing modified JSON files */
  'input-path': string
  /** Inject all available data */
  'inject-all': boolean
  /** Create backup before injection */
  'backup'?: boolean
  /** Verbose output */
  'verbose'?: boolean
  /** Dry run (no actual file modification) */
  'dry-run'?: boolean
}

export interface WolfTlResult {
  /** Exit code (0 = success) */
  exitCode: number
  /** Standard output */
  stdout: string
  /** Error output */
  stderr: string
  /** Whether injection succeeded */
  success: boolean
  /** List of modified files */
  modifiedFiles?: string[]
  /** Backup information */
  backupInfo?: {
    backupCreated: boolean
    backupPath?: string
  }
  /** Injection statistics */
  stats?: {
    totalFiles: number
    modifiedFiles: number
    skippedFiles: number
    errors: number
  }
}

// Expected WolfTL behavior:
// - Input: Folder containing .dat/.mps files (decrypted or native)
// - Output: JSON files in dump/ subdirectories (db/, common/, mps/)
// - Modes: create (extraction), patch (injection), patch_ip (injection in-place)
// - Exit codes: 0 = success, non-zero = error
// - Note: WolfTL can work directly with native .dat/.mps or after UberWolf decryption

// =============================================================================
// FILE FORMAT SPECIFICATIONS
// =============================================================================

export interface WolfRpgDumpStructure {
  /** Database files (extracted from DataBase.dat, etc.) */
  db: {
    /** Main database file */
    'DataBase.json': WolfRpgDatabase
    /** Custom database file */
    'CDataBase.json'?: WolfRpgDatabase
    /** System database file */
    'SysDatabase.json'?: WolfRpgDatabase
  }
  /** Common event files */
  common?: {
    /** Common event files indexed by ID */
    [key: string]: WolfRpgCommonEvent
  }
  /** Map files */
  mps?: {
    /** Map files indexed by coordinates or ID */
    [key: string]: WolfRpgMap
  }
}

// Simplified interfaces for WolfRPG data structures
export interface WolfRpgDatabase {
  // Database structure as extracted by UberWolf
  [key: string]: any
}

export interface WolfRpgCommonEvent {
  // Common event structure
  id: number
  name: string
  // ... other properties
  [key: string]: any
}

export interface WolfRpgMap {
  // Map structure
  id: string
  width: number
  height: number
  // ... other properties
  [key: string]: any
}

// =============================================================================
// COMPATIBILITY MATRIX
// =============================================================================

export interface ToolCompatibility {
  /** Supported WolfRPG versions */
  supportedVersions: string[]
  /** Supported file extensions */
  supportedExtensions: string[]
  /** Platform requirements */
  platformRequirements: {
    windows: {
      minVersion: string
      architectures: string[]
    }
    linux: {
      requiresWine: boolean
      minWineVersion?: string
    }
  }
  /** Known limitations */
  limitations: string[]
}

// UberWolf compatibility
export const UBERWOLF_COMPATIBILITY: ToolCompatibility = {
  supportedVersions: ['Wolf RPG Editor', 'Wolf RPG Editor Pro'],
  supportedExtensions: ['.wolf', '.data', '.pak', '.bin', '.assets', '.content', '.res', '.resource'],
  platformRequirements: {
    windows: {
      minVersion: '7',
      architectures: ['x86', 'x64']
    },
    linux: {
      requiresWine: true,
      minWineVersion: '7.0'
    }
  },
  limitations: [
    'Requires .NET Framework on Windows',
    'Some encrypted files may not be supported',
    'Large projects may take significant time'
  ]
}

// WolfTL compatibility
export const WOLFTL_COMPATIBILITY: ToolCompatibility = {
  supportedVersions: ['Wolf RPG Editor', 'Wolf RPG Editor Pro'],
  supportedExtensions: ['.wolf', '.data', '.pak', '.bin', '.assets', '.content', '.res', '.resource'],
  platformRequirements: {
    windows: {
      minVersion: '7',
      architectures: ['x86', 'x64']
    },
    linux: {
      requiresWine: true,
      minWineVersion: '7.0'
    }
  },
  limitations: [
    'Requires .NET Framework on Windows',
    'Modifications are permanent (use backup)',
    'Validation limited to file structure'
  ]
}

// =============================================================================
// ERROR HANDLING SPECIFICATION
// =============================================================================

export interface ToolError {
  /** Error code */
  code: string
  /** Human-readable message */
  message: string
  /** Suggested solution */
  solution?: string
  /** Whether the error is recoverable */
  recoverable: boolean
}

// =============================================================================
// COMBINED WORKFLOW SPECIFICATION
// =============================================================================

export interface CombinedWorkflow {
  /** Steps to execute in order */
  steps: WorkflowStep[]
  /** Expected total duration in seconds */
  estimatedDuration: number
  /** Whether the workflow is destructive (modifies original files) */
  isDestructive: boolean
}

export interface WorkflowStep {
  /** Step identifier */
  id: string
  /** Human-readable description */
  description: string
  /** Tool to use (ubwolf or wolftl) */
  tool: 'ubwolf' | 'wolftl'
  /** Command arguments */
  args: string[]
  /** Whether this step is conditional */
  conditional: boolean
  /** Condition to check before executing */
  condition?: string
  /** Expected duration in seconds */
  estimatedDuration: number
}

// Extraction workflow for encrypted projects
export const ENCRYPTED_EXTRACTION_WORKFLOW: CombinedWorkflow = {
  steps: [
    {
      id: 'decrypt',
      description: 'Déchiffrer les fichiers .wolf/.data/etc. avec UberWolf',
      tool: 'ubwolf',
      args: ['--project-path', '{game_path}', '--output-path', '{output_path}', '--extract-all'],
      conditional: true,
      condition: 'encrypted_files_present',
      estimatedDuration: 45
    },
    {
      id: 'extract',
      description: 'Extraire les données JSON avec WolfTL',
      tool: 'wolftl',
      args: ['{data_folder}', '{output_folder}', 'create'],
      conditional: false,
      estimatedDuration: 30
    }
  ],
  estimatedDuration: 75,
  isDestructive: false
}

// Extraction workflow for native projects
export const NATIVE_EXTRACTION_WORKFLOW: CombinedWorkflow = {
  steps: [
    {
      id: 'extract',
      description: 'Extraire les données JSON directement avec WolfTL',
      tool: 'wolftl',
      args: ['{data_folder}', '{output_folder}', 'create'],
      conditional: false,
      estimatedDuration: 30
    }
  ],
  estimatedDuration: 30,
  isDestructive: false
}

// Injection workflow
export const INJECTION_WORKFLOW: CombinedWorkflow = {
  steps: [
    {
      id: 'inject',
      description: 'Injecter les traductions avec WolfTL',
      tool: 'wolftl',
      args: ['{data_folder}', '{input_folder}', 'patch_ip'],
      conditional: false,
      estimatedDuration: 25
    }
  ],
  estimatedDuration: 25,
  isDestructive: true
}

export const COMMON_TOOL_ERRORS: Record<string, ToolError> = {
  TOOL_NOT_FOUND: {
    code: 'TOOL_NOT_FOUND',
    message: 'Outil non trouvé dans le répertoire spécifié',
    solution: 'Vérifiez que l\'outil est installé dans src-tauri/Tools/wolfrpg/',
    recoverable: true
  },
  WINE_NOT_FOUND: {
    code: 'WINE_NOT_FOUND',
    message: 'Wine n\'est pas installé sur Linux',
    solution: 'Installez Wine avec: sudo apt install wine',
    recoverable: true
  },
  INVALID_PROJECT: {
    code: 'INVALID_PROJECT',
    message: 'Le projet WolfRPG n\'est pas valide',
    solution: 'Vérifiez que Game.exe existe et que la structure est correcte',
    recoverable: true
  },
  EXTRACTION_FAILED: {
    code: 'EXTRACTION_FAILED',
    message: 'Échec de l\'extraction des données',
    solution: 'Vérifiez les permissions et la validité des fichiers',
    recoverable: false
  },
  INJECTION_FAILED: {
    code: 'INJECTION_FAILED',
    message: 'Échec de l\'injection des données',
    solution: 'Vérifiez que les fichiers JSON sont valides et restaurer depuis backup',
    recoverable: false
  }
}
