// Glossary Extraction Operations
// Handles extracting terms from translations to glossary

import { createGlossaryEntry } from './create'
import type {
  CreateGlossaryEntry,
  GlossaryEntry,
  GlossaryOperationResult
} from './types'

/**
 * Extract a term from a translation entry directly to glossary
 * Creates a glossary entry with source_text and translated_text
 */
export async function extractToGlossary(
  source_term: string,
  translated_term: string,
  source_language?: string,
  target_language?: string,
  category?: CreateGlossaryEntry['category']
): Promise<GlossaryOperationResult<GlossaryEntry>> {
  // Validate inputs
  if (!source_term || !translated_term) {
    return {
      success: false,
      error: 'Source term and translated term are required'
    }
  }

  // Create glossary entry
  const entry: CreateGlossaryEntry = {
    source_term: source_term.trim(),
    translated_term: translated_term.trim(),
    source_language,
    target_language,
    category: category || 'general'
  }

  return createGlossaryEntry(entry)
}

