// Glossary Bridge - Frontend event listener for backend glossary lookup
// Listens to "glossary-lookup-request" events from Rust backend
// Responds with "glossary-lookup-response" containing glossary terms

import { listen, emit } from '@tauri-apps/api/event'
import { getGlossaryTermsForLanguages } from './read'
import type { GlossaryLookupRequest, GlossaryLookupResponse } from './types'

/**
 * Setup event listener for glossary lookup requests from backend
 * This bridge enables Rust backend to query glossary terms via Tauri events
 */
export async function setupGlossaryBridge(): Promise<() => void> {
  // Listen for glossary lookup requests from backend
  const unlisten = await listen<GlossaryLookupRequest>(
    'glossary-lookup-request',
    async (event) => {
      const { request_id, source_language, target_language, project_id, category } = event.payload

      console.log(
        `[GlossaryBridge] Received glossary-lookup-request: request_id=${request_id}, source_language=${source_language}, target_language=${target_language}, project_id=${project_id ?? 'null'}, category=${category ?? 'null'}`
      )

      try {
        // Fetch glossary terms from database
        // Behavior: ALWAYS retrieves global terms, IF project_id provided ALSO retrieves project-specific terms
        // Result: Combined global + project-specific terms (if project_id provided) or only global terms
        // IF category provided: FILTERS terms by category (only terms matching the category)
        // Convert project_id from number to number | null for TypeScript
        const projectId = project_id !== undefined ? (project_id === null ? null : Number(project_id)) : undefined
        const categoryFilter = category !== undefined ? (category === null ? null : String(category)) : undefined
        const result = await getGlossaryTermsForLanguages(
          source_language,
          target_language,
          projectId,
          categoryFilter
        )

        // Prepare response
        const response: GlossaryLookupResponse = {
          request_id,
          success: result.success,
          data: result.success ? result.data : undefined,
          error: result.success ? undefined : result.error
        }

        // Emit response back to backend
        await emit('glossary-lookup-response', response)

        console.log(
          `[GlossaryBridge] Emitted glossary-lookup-response: request_id=${request_id}, success=${result.success}, entries=${result.success ? result.data?.length || 0 : 0}`
        )
      } catch (error) {
        // Handle errors
        const errorMessage = error instanceof Error ? error.message : String(error)
        console.error(
          `[GlossaryBridge] Error processing glossary-lookup-request:`,
          error
        )

        // Emit error response
        const errorResponse: GlossaryLookupResponse = {
          request_id,
          success: false,
          data: undefined,
          error: errorMessage
        }

        await emit('glossary-lookup-response', errorResponse)
      }
    }
  )

  console.log('[GlossaryBridge] Event listener setup complete')

  // Return cleanup function
  return unlisten
}

