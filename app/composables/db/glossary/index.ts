// Glossary Database Composables
// Exports for glossary storage and retrieval operations

// Create operations
export {
  createGlossaryEntry,
  createBulkGlossaryEntries
} from './create'

// Read operations
export {
  getGlossaryTermsForLanguages,
  getGlossaryEntries,
  getGlossaryEntry,
  searchGlossaryByTerm,
  getGlossaryStats
} from './read'

// Update operations
export {
  updateGlossaryEntry,
  bulkUpdateGlossaryEntries
} from './update'

// Delete operations
export {
  deleteGlossaryEntry,
  bulkDeleteGlossaryEntries,
  deleteGlossaryEntriesForLanguages,
  deleteGlossaryEntriesByCategory
} from './delete'

// Types
export type {
  GlossaryEntry,
  CreateGlossaryEntry,
  UpdateGlossaryEntry,
  GlossaryFilters,
  GlossaryQueryResult,
  GlossaryOperationResult,
  GlossaryLookupRequest,
  GlossaryLookupResponse
} from './types'

// Bridge
export { setupGlossaryBridge } from './glossaryBridge'

