// Opérations de suppression des projets
// Implémentation des opérations CRUD avec le plugin SQL Tauri

import { useDatabase, executeQuery } from '../useDatabase'
import { executeDBOperation, checkRecordExists } from '../useDBOperation'
import type { DBOperationResult } from './types'

/**
 * Supprime un projet par son ID
 * Note: SQLite gère automatiquement la suppression en cascade grâce aux FOREIGN KEY constraints
 */
export async function deleteProject(id: number): Promise<DBOperationResult> {
    // Vérifier que le projet existe avant suppression
  const exists = await checkRecordExists(
      'SELECT id FROM projects WHERE id = ?',
      [id]
    )

  if (!exists) {
      return {
        success: false,
        error: `Project with ID ${id} not found`
      }
    }

  return executeDBOperation(async () => {
    const db = await useDatabase()

    // Supprimer le projet
    // SQLite supprimera automatiquement les enregistrements liés grâce aux FOREIGN KEY constraints
    await db.execute(
      'DELETE FROM projects WHERE id = ?',
      [id]
    )
  }, 'deleting project')
}
