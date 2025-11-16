/**
 * Utility functions for detecting connection-related errors
 * Centralizes error detection logic to avoid duplication
 */

/**
 * Check if an error message indicates a connection error
 * @param errorMessage - The error message to check
 * @returns true if the error is connection-related
 */
export function isConnectionError(errorMessage: string | null | undefined): boolean {
  if (!errorMessage) return false
  
  const errorMsgLower = errorMessage.toLowerCase()
  
  return (
    errorMsgLower.includes('connection') ||
    errorMsgLower.includes('ollama') ||
    errorMsgLower.includes('network') ||
    errorMsgLower.includes('econnrefused') ||
    errorMsgLower.includes('timeout') ||
    errorMsgLower.includes('refused') ||
    errorMsgLower.includes('unreachable') ||
    errorMsgLower.includes('failed to connect') ||
    errorMsgLower.includes('no connection')
  )
}

/**
 * Check if an error object indicates a connection error
 * @param error - The error object (Error instance or any)
 * @returns true if the error is connection-related
 */
export function isConnectionErrorFromObject(error: unknown): boolean {
  if (error instanceof Error) {
    return isConnectionError(error.message)
  }
  
  if (typeof error === 'string') {
    return isConnectionError(error)
  }
  
  return false
}

