// Opérations de suppression des projets
// Implémentation des opérations CRUD avec le plugin SQL Tauri

import { useDatabase, executeQuery } from '../useDatabase'
import type { DBOperationResult } from './types'

/**
 * Supprime un projet par son ID
 * Note: SQLite gère automatiquement la suppression en cascade grâce aux FOREIGN KEY constraints
 */
export async function deleteProject(id: number): Promise<DBOperationResult> {
  try {
    const db = await useDatabase()

    // Vérifier que le projet existe avant suppression
    const existingProject = await executeQuery<{ id: number }>(
      'SELECT id FROM projects WHERE id = ?',
      [id]
    )

    if (existingProject.length === 0) {
      return {
        success: false,
        error: `Project with ID ${id} not found`
      }
    }

    // Supprimer le projet
    // SQLite supprimera automatiquement les enregistrements liés grâce aux FOREIGN KEY constraints
    await db.execute(
      'DELETE FROM projects WHERE id = ?',
      [id]
    )

    return {
      success: true
    }

  } catch (error) {
    console.error('Error deleting project:', error)
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error occurred'
    }
  }
}
