// Glossary Database Operations Types
// Based on glossary_entries table structure

export interface GlossaryEntry {
  id: number
  source_term: string
  translated_term: string
  source_language: string  // ISO 639-1 code (ja, en, fr, etc.)
  target_language: string  // ISO 639-1 code
  category: 'general' | 'character' | 'item' | 'location' | 'system' | 'skill'
  project_id: number | null  // NULL = global pour tous les projets, INTEGER = spécifique à un projet
  created_at: string
  updated_at: string
}

export interface CreateGlossaryEntry {
  source_term: string
  translated_term: string
  source_language?: string  // Defaults to 'ja' in DB
  target_language?: string  // Defaults to 'fr' in DB
  category?: 'general' | 'character' | 'item' | 'location' | 'system' | 'skill'
  project_id?: number | null  // NULL = global pour tous les projets, INTEGER = spécifique à un projet
}

export interface UpdateGlossaryEntry {
  id: number
  source_term?: string
  translated_term?: string
  source_language?: string
  target_language?: string
  category?: 'general' | 'character' | 'item' | 'location' | 'system' | 'skill'
  project_id?: number | null  // NULL = global pour tous les projets, INTEGER = spécifique à un projet
}

export interface GlossaryFilters {
  category?: string[]
  source_language?: string
  target_language?: string
  search?: string  // Search in source_term or translated_term
  project_id?: number | null | 'global' | 'current'  // null = global, number = project ID, 'global' = only global, 'current' = only current project
  limit?: number
  offset?: number
}

export interface GlossaryQueryResult {
  entries: GlossaryEntry[]
  total_count: number
}

// Result types for operations
export interface GlossaryOperationResult<T = void> {
  success: boolean
  data?: T
  error?: string
}

// Event payload types for glossary bridge communication
export interface GlossaryLookupRequest {
  request_id: string
  source_language: string
  target_language: string
  project_id?: number | null  // NULL = global uniquement, INTEGER = combine global + project-specific
  category?: string | null  // Filter glossary terms by category (null = all categories)
}

export interface GlossaryLookupResponse {
  request_id: string
  success: boolean
  data?: GlossaryEntry[]
  error?: string
}

