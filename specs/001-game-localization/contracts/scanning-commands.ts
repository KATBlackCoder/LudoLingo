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

export interface ScanProgress {
  current_file: string;
  files_processed: number;
  total_files: number;
  entries_extracted: number;
  errors: string[];
}

export interface TranslationEntry {
  id: number;
  project_id: number;
  game_file_id?: number;
  source_text: string;
  translated_text?: string;
  context?: string;
  text_type: 'dialogue' | 'system' | 'item' | 'skill' | 'other';
  status: 'extracted' | 'translated' | 'reviewed' | 'finalized';
  translation_source?: 'manual' | 'ollama' | 'glossary';
  created_at: string;
  updated_at: string;
}

// Tauri Commands
export interface ScanningCommands {
  // Scan folder for game files
  scan_folder: (request: ScanRequest) => Promise<{
    scan_id: string;
    total_files_found: number;
  }>;

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
  validate_file_format: (file_path: string) => Promise<{
    supported: boolean;
    detected_engine?: string;
    format_details?: string;
  }>;
}
