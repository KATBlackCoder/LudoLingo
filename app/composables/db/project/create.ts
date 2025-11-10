// Opérations de création de projets
// Implémentation des opérations CRUD avec le plugin SQL Tauri

import { useDatabase, executeQuery } from '../useDatabase'
import type { CreateProjectData, ProjectDB, DBOperationResult } from './types'

/**
 * Crée un nouveau projet en base de données
 */
export async function createProject(data: CreateProjectData): Promise<DBOperationResult<ProjectDB>> {
  try {
    const db = await useDatabase()

    // Générer un ID unique et timestamps
    const projectId = Date.now()
    const now = new Date().toISOString()

    // Insérer le projet
    await db.execute(
      `INSERT INTO projects (
        id, name, description, source_language, target_language, game_path, game_engine,
        created_at, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
      [
        projectId,
        data.name,
        data.description || null,
        'ja', // Japonais par défaut
        'fr', // Français par défaut
        data.game_path,
        data.game_engine || null,
        now,
        now
      ]
    )

    // Récupérer le projet créé pour le retourner
    const result = await executeQuery<ProjectDB>(
      'SELECT * FROM projects WHERE id = ?',
      [projectId]
    )

    if (result.length === 0) {
      throw new Error('Project creation failed - could not retrieve created project')
    }

    return {
      success: true,
      data: result[0]
    }

  } catch (error) {
    console.error('Error creating project:', error)
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error occurred'
    }
  }
}
