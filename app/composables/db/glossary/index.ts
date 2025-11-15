// Glossary Database Composables
// Exports for glossary storage and retrieval operations

// Create operations
export {
  createGlossaryEntry,
  createBulkGlossaryEntries
} from './create'

// Extract operations
export {
  extractToGlossary
} from './extract'

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

// Import/Export operations
export {
  exportGlossaryToJSON,
  exportGlossaryToCSV,
  importGlossaryFromJSON,
  importGlossaryFromCSV
} from './importExport'

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

