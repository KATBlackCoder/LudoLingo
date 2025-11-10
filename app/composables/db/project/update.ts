// Opérations de mise à jour des projets
// Implémentation des opérations CRUD avec le plugin SQL Tauri

import { useDatabase, executeQuery } from '../useDatabase'
import type { UpdateProjectData, ProjectDB, DBOperationResult } from './types'

/**
 * Met à jour un projet existant
 */
export async function updateProject(id: number, data: UpdateProjectData): Promise<DBOperationResult<ProjectDB>> {
  try {
    const db = await useDatabase()

    // Vérifier que le projet existe
    const existingProject = await executeQuery<ProjectDB>(
      'SELECT * FROM projects WHERE id = ?',
      [id]
    )

    if (existingProject.length === 0) {
      return {
        success: false,
        error: `Project with ID ${id} not found`
      }
    }

    // Construction de la requête UPDATE dynamique
    const updateFields: string[] = []
    const params: any[] = []

    if (data.name !== undefined) {
      updateFields.push('name = ?')
      params.push(data.name)
    }

    if (data.description !== undefined) {
      updateFields.push('description = ?')
      params.push(data.description)
    }

    if (data.source_language !== undefined) {
      updateFields.push('source_language = ?')
      params.push(data.source_language)
    }

    if (data.target_language !== undefined) {
      updateFields.push('target_language = ?')
      params.push(data.target_language)
    }

    if (data.game_path !== undefined) {
      updateFields.push('game_path = ?')
      params.push(data.game_path)
    }

    if (data.game_engine !== undefined) {
      updateFields.push('game_engine = ?')
      params.push(data.game_engine)
    }

    // Toujours mettre à jour updated_at
    updateFields.push('updated_at = ?')
    params.push(new Date().toISOString())

    if (updateFields.length === 1) {
      // Seulement updated_at a été modifié
      return {
        success: true,
        data: existingProject[0]
      }
    }

    // Ajouter l'ID à la fin des paramètres
    params.push(id)

    const sql = `UPDATE projects SET ${updateFields.join(', ')} WHERE id = ?`

    await db.execute(sql, params)

    // Récupérer le projet mis à jour
    const updatedResult = await executeQuery<ProjectDB>(
      'SELECT * FROM projects WHERE id = ?',
      [id]
    )

    return {
      success: true,
      data: updatedResult[0]
    }

  } catch (error) {
    console.error('Error updating project:', error)
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error occurred'
    }
  }
}
