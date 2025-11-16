// Opérations de création de projets
// Implémentation des opérations CRUD avec le plugin SQL Tauri

import { executeQuery, executeStatement } from '../useDatabase'
import { executeDBOperation } from '../useDBOperation'
import type { CreateProjectData, ProjectDB, DBOperationResult } from './types'

/**
 * Crée un nouveau projet en base de données
 */
export async function createProject(data: CreateProjectData): Promise<DBOperationResult<ProjectDB>> {
  return executeDBOperation(async () => {
    // Laisser SQLite générer l'ID automatiquement avec AUTOINCREMENT
    const now = new Date().toISOString()

    // Insérer le projet (sans spécifier l'ID pour utiliser AUTOINCREMENT)
    const result = await executeStatement(
      `INSERT INTO projects (
        name, source_language, target_language, game_path, game_engine,
        created_at, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?)`,
      [
        data.name,
        'ja', // Japonais par défaut
        'fr', // Français par défaut
        data.game_path,
        data.game_engine || null,
        now,
        now
      ]
    )

    if (!result || !result.lastInsertId) {
      throw new Error('Failed to create project - no ID returned')
    }

    // Récupérer le projet créé avec l'ID généré par SQLite
    const projectResult = await executeQuery<ProjectDB>(
      'SELECT * FROM projects WHERE id = ?',
      [result.lastInsertId]
    )

    if (projectResult.length === 0) {
      throw new Error('Project creation failed - could not retrieve created project')
    }

    return projectResult[0]
  }, 'creating project')
}
