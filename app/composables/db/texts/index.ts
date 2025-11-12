// Text Database Composables
// Exports for text storage and retrieval operations

// Create operations
export {
  createTextEntry,
  createBulkTextEntries,
  createOrGetGameFile
} from './create'

// Read operations
export {
  getProjectTexts,
  getProjectTextsWithCount,
  getTextEntry,
  getProjectFiles,
  getProjectTextStats
} from './read'

// Update operations
export {
  updateTextEntry,
  updateTextTranslation,
  updateTextStatus,
  bulkUpdateTextTranslations,
  bulkUpdateTextStatuses,
  updateGameFileStatus
} from './update'

// Delete operations
export {
  deleteTextEntry,
  deleteProjectTexts,
  deleteGameFileTexts,
  deleteGameFile,
  deleteProjectGameFiles,
  cleanupOrphanedTexts
} from './delete'

// Translation operations
export {
  startSequentialTranslation,
  getTranslationProgress,
  pauseTranslationSession,
  resumeTranslationSession,
  stopTranslationSession,
  getProjectTranslationSessions,
  getTranslationSuggestions,
  updateTextWithTranslation,
  bulkUpdateTranslations
} from './translation'

// Types
export type {
  DBTextEntry,
  CreateTextEntry,
  UpdateTextEntry,
  TextFilters,
  TextQueryResult,
  DBGameFile,
  CreateGameFile,
  UpdateGameFile,
  TextOperationResult,
  BulkTextOperationResult
} from './types'

export type {
  TranslationSession,
  TranslationProgress,
  TranslationSuggestion,
  StartTranslationRequest
} from './translation'
