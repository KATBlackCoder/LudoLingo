// WolfRPG Tools Commands API Contract
// Defines the Tauri commands for WolfRPG tools integration

export interface WolfRpgProjectType {
  /** Type of WolfRPG project detected */
  type: 'needs_ubwolf' | 'dump_exists' | 'invalid'
  /** Human-readable description */
  description: string
  /** Whether the project is ready for translation */
  ready_for_translation: boolean
  /** Whether extraction is needed */
  needs_extraction: boolean
}

export interface WolfRpgToolsValidation {
  /** Whether UberWolf is available and functional */
  uberwolf_available: boolean
  /** Whether WolfTL is available and functional */
  wolftl_available: boolean
  /** Whether Wine is available (Linux only) */
  wine_available?: boolean
  /** Overall validation status */
  is_valid: boolean
  /** Validation errors if any */
  errors: string[]
}

export interface WolfRpgOperationResult {
  /** Whether the operation succeeded */
  success: boolean
  /** Operation output/logs */
  output?: string
  /** Error message if failed */
  error?: string
  /** Duration in milliseconds */
  duration_ms: number
}

// Command: detect_wolfrpg_project_type
export interface DetectWolfRpgProjectTypeRequest {
  /** Absolute path to the game project directory */
  game_path: string
}

export interface DetectWolfRpgProjectTypeResponse extends WolfRpgProjectType {}

// Command: validate_wolfrpg_tools
export interface ValidateWolfRpgToolsRequest {
  /** Absolute path to the tools directory containing UberWolf and WolfTL */
  tools_dir: string
}

export interface ValidateWolfRpgToolsResponse extends WolfRpgToolsValidation {}

// Command: setup_wine_environment (Linux only)
export interface SetupWineEnvironmentRequest {
  // No parameters needed
}

export interface SetupWineEnvironmentResponse extends WolfRpgOperationResult {
  /** Whether Wine was installed or was already available */
  wine_installed: boolean
}

// Command: extract_wolfrpg_data
export interface ExtractWolfRpgDataRequest {
  /** Absolute path to the game project directory */
  game_path: string
  /** Absolute path to the tools directory */
  tools_dir: string
}

export interface ExtractWolfRpgDataResponse extends WolfRpgOperationResult {
  /** Path to the created dump directory */
  dump_path?: string
  /** Number of files extracted */
  files_extracted?: number
}

// Command: inject_wolfrpg_data
export interface InjectWolfRpgDataRequest {
  /** Absolute path to the game project directory */
  game_path: string
  /** Absolute path to the tools directory */
  tools_dir: string
}

export interface InjectWolfRpgDataResponse extends WolfRpgOperationResult {
  /** Number of files modified */
  files_modified?: number
  /** Backup created status */
  backup_created?: boolean
}

// Command: process_wolfrpg_project (unified workflow)
export interface ProcessWolfRpgProjectRequest {
  /** Absolute path to the game project directory */
  game_path: string
  /** Absolute path to the tools directory */
  tools_dir: string
  /** Force re-extraction even if dump exists */
  force_extraction?: boolean
}

export interface ProcessWolfRpgProjectResponse extends WolfRpgOperationResult {
  /** Final project type after processing */
  final_type: WolfRpgProjectType['type']
  /** Whether extraction was performed */
  extraction_performed?: boolean
  /** Whether the project is ready for translation */
  ready_for_translation: boolean
}
