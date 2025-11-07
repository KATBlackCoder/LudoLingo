// Injection Commands Contract
// Tauri commands for reinjecting translations into game files

export interface InjectionRequest {
  project_id: number;
  file_ids?: number[]; // specific files, or all if empty
  create_backup?: boolean; // default: true
}

export interface InjectionProgress {
  current_file: string;
  files_processed: number;
  total_files: number;
  entries_injected: number;
  errors: Array<{
    file_path: string;
    error_message: string;
  }>;
  backup_paths: string[];
}

export interface InjectionResult {
  injection_id: string;
  status: 'completed' | 'partial' | 'failed';
  files_processed: number;
  entries_injected: number;
  backup_created: boolean;
  backup_path?: string;
  errors: Array<{
    file_path: string;
    error_message: string;
  }>;
  completed_at: string;
}

// Tauri Commands
export interface InjectionCommands {
  // Start translation injection
  start_injection: (request: InjectionRequest) => Promise<{
    injection_id: string;
    total_files: number;
    estimated_duration: number; // in seconds
  }>;

  // Get injection progress
  get_injection_progress: (injection_id: string) => Promise<InjectionProgress>;

  // Cancel injection
  cancel_injection: (injection_id: string) => Promise<void>;

  // Get injection result
  get_injection_result: (injection_id: string) => Promise<InjectionResult>;

  // Validate injection (dry run)
  validate_injection: (project_id: number, file_ids?: number[]) => Promise<{
    valid: boolean;
    issues: Array<{
      file_path: string;
      severity: 'warning' | 'error';
      message: string;
    }>;
    summary: {
      files_to_process: number;
      entries_to_inject: number;
      untranslated_entries: number;
    };
  }>;

  // Restore from backup
  restore_from_backup: (backup_path: string, target_path?: string) => Promise<{
    restored: boolean;
    files_restored: number;
    errors: string[];
  }>;

  // List available backups
  list_backups: (project_id?: number) => Promise<Array<{
    backup_path: string;
    project_name?: string;
    created_at: string;
    size_bytes: number;
    injection_id?: string;
  }>>;

  // Clean old backups
  clean_old_backups: (older_than_days: number, project_id?: number) => Promise<{
    deleted_count: number;
    freed_space_bytes: number;
  }>;
}
