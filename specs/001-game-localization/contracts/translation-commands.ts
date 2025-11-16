// Translation Commands Contract
// Tauri commands for batch translation and manual editing

export interface TranslationBatch {
  id: number;
  project_id: number;
  batch_name?: string;
  status: 'pending' | 'processing' | 'completed' | 'error';
  total_entries: number;
  processed_entries: number;
  error_count: number;
  started_at?: string;
  completed_at?: string;
  created_at: string;
}

export interface BatchTranslationRequest {
  project_id: number;
  entry_ids: number[];
  batch_name?: string;
  use_glossary?: boolean;
}

export interface BatchProgress {
  batch_id: number;
  status: string;
  processed_entries: number;
  total_entries: number;
  current_entry?: string;
  errors: Array<{
    entry_id: number;
    error_message: string;
  }>;
}

export interface OllamaStatus {
  available: boolean;
  version?: string;
  models_available: string[];
  error?: string;
}

// Tauri Commands
export interface TranslationCommands {
  // Check Ollama availability
  check_ollama_status: () => Promise<OllamaStatus>;

  // Start batch translation
  start_batch_translation: (request: BatchTranslationRequest) => Promise<TranslationBatch>;

  // Get batch progress
  get_batch_progress: (batch_id: number) => Promise<BatchProgress>;

  // Cancel batch translation
  cancel_batch_translation: (batch_id: number) => Promise<void>;

  // Get active batches for project
  get_project_batches: (project_id: number) => Promise<TranslationBatch[]>;

  // Update single translation entry
  update_translation_entry: (entry_id: number, translated_text: string, source?: string) => Promise<void>;

  // Bulk update translation entries
  bulk_update_translations: (updates: Array<{
    entry_id: number;
    translated_text: string;
    source?: string;
  }>) => Promise<{
    updated_count: number;
    errors: Array<{
      entry_id: number;
      error: string;
    }>;
  }>;

  // Apply glossary translation to entry
  apply_glossary_to_entry: (entry_id: number, glossary_id: number) => Promise<void>;

  // Get translation suggestions for text
  get_translation_suggestions: (source_text: string, context?: string) => Promise<Array<{
    suggestion: string;
    confidence: number;
    source: 'ollama' | 'glossary' | 'similar';
  }>>;
}
