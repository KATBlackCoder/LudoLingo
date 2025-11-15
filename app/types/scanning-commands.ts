// File Scanning Commands Contract
// Tauri commands for game file scanning and text extraction

export interface GameFile {
  id: number;
  project_id: number;
  file_path: string;
  file_format: string;
  file_size: number;
  checksum: string;
  last_modified: string;
  scan_status: 'pending' | 'scanning' | 'completed' | 'error';
  created_at: string;
}

export interface ScanRequest {
  project_id: number;
  folder_path: string;
  recursive?: boolean;
}

export interface ScanResult {
  scan_id: string;
  total_files_found: number;
}

export interface ScanProgress {
  scan_id: string;
  current_file: string;
  files_processed: number;
  total_files: number;
  entries_extracted: number;
  errors: string[];
  status: 'InProgress' | 'Completed' | 'Failed';
}

// Extended interface for scan history with completion timestamp
export interface ScanHistoryEntry extends ScanProgress {
  completed_at: string;
}

export interface FileValidationResult {
  supported: boolean;
  detected_engine?: string;
  format_details?: string;
}

export interface TranslationEntry {
  id: number;
  project_id: number;
  game_file_id?: number;
  source_text: string;
  translated_text?: string;
  location: string;  // Structured identifier: "object_type:object_id:field"
  text_type: 'character' | 'dialogue' | 'system' | 'item' | 'skill' | 'general' | 'other';
  status: 'extracted' | 'translated' | 'reviewed';
  created_at: string;
  updated_at: string;
}

// Tauri Commands
export interface ScanningCommands {
  // Scan folder for game files
  scan_folder: (request: ScanRequest) => Promise<ScanResult>;

  // Get scan progress
  get_scan_progress: (scan_id: string) => Promise<ScanProgress>;

  // Cancel ongoing scan
  cancel_scan: (scan_id: string) => Promise<void>;

  // Get scanned files for project
  get_project_files: (project_id: number) => Promise<GameFile[]>;

  // Get extracted entries for project
  get_project_entries: (project_id: number, filters?: {
    status?: string[];
    text_type?: string[];
    limit?: number;
    offset?: number;
  }) => Promise<{
    entries: TranslationEntry[];
    total_count: number;
  }>;

  // Validate file format compatibility
  validate_file_format: (file_path: string) => Promise<FileValidationResult>;

  // Extract texts directly from folder
  extract_texts_from_folder: (folder_path: string) => Promise<TextEntry[]>;
}

export interface TextEntry {
  id: string;
  source_text: string;
  translated_text: string;
  field_type: string;
  status: 'NotTranslated' | 'Translated' | 'Ignored' | 'InProgress';
  prompt_type: 'Character' | 'Dialogue' | 'Item' | 'Skill' | 'System' | 'General' | 'Other';
  location: string;  // Structured identifier: "object_type:object_id:field"
  entry_type: string;
  file_path?: string;
}
