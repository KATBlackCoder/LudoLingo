// Glossary Commands Contract
// Tauri commands for glossary management

export interface GlossaryEntry {
  id: number;
  source_term: string;
  translated_term: string;
  context?: string;
  category: 'general' | 'character' | 'item' | 'location' | 'system' | 'skill';
  frequency: number;
  created_at: string;
  updated_at: string;
}

export interface CreateGlossaryEntryRequest {
  source_term: string;
  translated_term: string;
  context?: string;
  category?: string;
}

export interface UpdateGlossaryEntryRequest {
  id: number;
  source_term?: string;
  translated_term?: string;
  context?: string;
  category?: string;
}

// Tauri Commands
export interface GlossaryCommands {
  // Create glossary entry
  create_glossary_entry: (request: CreateGlossaryEntryRequest) => Promise<GlossaryEntry>;

  // Get all glossary entries
  list_glossary_entries: (filters?: {
    category?: string;
    search?: string;
    limit?: number;
    offset?: number;
  }) => Promise<{
    entries: GlossaryEntry[];
    total_count: number;
  }>;

  // Get glossary entry by ID
  get_glossary_entry: (id: number) => Promise<GlossaryEntry>;

  // Update glossary entry
  update_glossary_entry: (request: UpdateGlossaryEntryRequest) => Promise<GlossaryEntry>;

  // Delete glossary entry
  delete_glossary_entry: (id: number) => Promise<void>;

  // Extract term from translation to glossary
  extract_to_glossary: (translation_entry_id: number, category?: string) => Promise<GlossaryEntry>;

  // Apply glossary entry to multiple translations
  apply_glossary_to_translations: (glossary_id: number, translation_entry_ids: number[]) => Promise<{
    applied_count: number;
    skipped_count: number;
  }>;

  // Find similar terms in glossary
  find_similar_glossary_terms: (source_term: string, limit?: number) => Promise<Array<{
    entry: GlossaryEntry;
    similarity_score: number;
  }>>;

  // Get glossary statistics
  get_glossary_stats: () => Promise<{
    total_entries: number;
    categories_breakdown: Record<string, number>;
    most_used_terms: Array<{
      term: string;
      frequency: number;
    }>;
  }>;

  // Import glossary from CSV/JSON
  import_glossary: (file_path: string, format: 'csv' | 'json') => Promise<{
    imported_count: number;
    errors: string[];
  }>;

  // Export glossary to CSV/JSON
  export_glossary: (file_path: string, format: 'csv' | 'json', categories?: string[]) => Promise<{
    exported_count: number;
    file_path: string;
  }>;
}
