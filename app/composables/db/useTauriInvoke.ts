/**
 * Centralized Tauri invoke helper with consistent error handling
 * Replaces duplicate invokeTauri functions across composables
 */

import { invoke } from '@tauri-apps/api/core'
import type { TextOperationResult } from './texts/types'

/**
 * Generic helper for Tauri invoke operations with error handling
 * @param command - Tauri command name
 * @param args - Arguments to pass to the command
 * @returns Result with success flag and data or error
 */
export async function invokeTauri<T>(
  command: string,
  args: Record<string, unknown> = {}
): Promise<TextOperationResult<T>> {
  try {
    const result = await invoke(command, args)
    return { success: true, data: result as T }
  } catch (error) {
    // Tauri errors can be Error objects or strings
    let errorMessage: string
    if (error instanceof Error) {
      errorMessage = error.message
    } else if (typeof error === 'string') {
      errorMessage = error
    } else if (error && typeof error === 'object' && 'message' in error) {
      errorMessage = String((error as { message: unknown }).message)
    } else {
      errorMessage = String(error) || 'Unknown error'
    }
    
    return {
      success: false,
      error: `Failed to ${command.replace(/_/g, ' ')}: ${errorMessage}`
    }
  }
}

/**
 * Generic helper for Tauri invoke operations without return data
 * @param command - Tauri command name
 * @param args - Arguments to pass to the command
 * @returns Result with success flag or error
 */
export async function invokeTauriVoid(
  command: string,
  args: Record<string, unknown> = {}
): Promise<TextOperationResult> {
  try {
    await invoke(command, args)
    return { success: true }
  } catch (error) {
    // Tauri errors can be Error objects or strings
    let errorMessage: string
    if (error instanceof Error) {
      errorMessage = error.message
    } else if (typeof error === 'string') {
      errorMessage = error
    } else if (error && typeof error === 'object' && 'message' in error) {
      errorMessage = String((error as { message: unknown }).message)
    } else {
      errorMessage = String(error) || 'Unknown error'
    }
    
    return {
      success: false,
      error: `Failed to ${command.replace(/_/g, ' ')}: ${errorMessage}`
    }
  }
}

