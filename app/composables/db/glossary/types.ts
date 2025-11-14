// Glossary Database Operations Types
// Based on glossary_entries table structure

export interface GlossaryEntry {
  id: number
  source_term: string
  translated_term: string
  source_language: string  // ISO 639-1 code (ja, en, fr, etc.)
  target_language: string  // ISO 639-1 code
  category: 'general' | 'character' | 'item' | 'location' | 'system' | 'skill'
  created_at: string
  updated_at: string
}

export interface CreateGlossaryEntry {
  source_term: string
  translated_term: string
  source_language?: string  // Defaults to 'ja' in DB
  target_language?: string  // Defaults to 'fr' in DB
  category?: 'general' | 'character' | 'item' | 'location' | 'system' | 'skill'
}

export interface UpdateGlossaryEntry {
  id: number
  source_term?: string
  translated_term?: string
  source_language?: string
  target_language?: string
  category?: 'general' | 'character' | 'item' | 'location' | 'system' | 'skill'
}

export interface GlossaryFilters {
  category?: string[]
  source_language?: string
  target_language?: string
  search?: string  // Search in source_term or translated_term
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

