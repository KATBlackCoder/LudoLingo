// Opérations de lecture des projets
// Implémentation des opérations CRUD avec le plugin SQL Tauri

import { useDatabase, executeQuery } from '../useDatabase'
import type { ProjectDB, ProjectFilters, ProjectListResult, DBOperationResult } from './types'

/**
 * Récupère tous les projets avec filtres optionnels
 */
export async function getProjects(filters?: ProjectFilters): Promise<DBOperationResult<ProjectListResult>> {
  try {
    const db = await useDatabase()

    // Construction de la requête avec filtres
    let sql = 'SELECT * FROM projects WHERE 1=1'
    const params: any[] = []

    if (filters?.search) {
      sql += ' AND (name LIKE ? OR description LIKE ?)'
      const searchPattern = `%${filters.search}%`
      params.push(searchPattern, searchPattern)
    }

    if (filters?.game_path) {
      sql += ' AND game_path = ?'
      params.push(filters.game_path)
    }

    if (filters?.game_engine) {
      sql += ' AND game_engine = ?'
      params.push(filters.game_engine)
    }

    // Tri par date de mise à jour décroissante
    sql += ' ORDER BY updated_at DESC'

    // Pagination
    if (filters?.limit) {
      sql += ' LIMIT ?'
      params.push(filters.limit)

      if (filters?.offset) {
        sql += ' OFFSET ?'
        params.push(filters.offset)
      }
    }

    const projects = await executeQuery<ProjectDB>(sql, params)

    // Compter le total pour la pagination
    let countSql = 'SELECT COUNT(*) as total FROM projects WHERE 1=1'
    const countParams: any[] = []

    if (filters?.search) {
      countSql += ' AND (name LIKE ? OR description LIKE ?)'
      const searchPattern = `%${filters.search}%`
      countParams.push(searchPattern, searchPattern)
    }

    if (filters?.game_path) {
      countSql += ' AND game_path = ?'
      countParams.push(filters.game_path)
    }

    if (filters?.game_engine) {
      countSql += ' AND game_engine = ?'
      countParams.push(filters.game_engine)
    }

    const countResult = await executeQuery<{ total: number }>(countSql, countParams)

    return {
      success: true,
      data: {
        projects,
        total: countResult[0]?.total || 0
      }
    }

  } catch (error) {
    console.error('Error fetching projects:', error)
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error occurred'
    }
  }
}

/**
 * Récupère un projet par son ID
 */
export async function getProject(id: number): Promise<DBOperationResult<ProjectDB>> {
  try {
    const db = await useDatabase()

    const result = await executeQuery<ProjectDB>(
      'SELECT * FROM projects WHERE id = ?',
      [id]
    )

    if (result.length === 0) {
      return {
        success: false,
        error: `Project with ID ${id} not found`
      }
    }

    return {
      success: true,
      data: result[0]
    }

  } catch (error) {
    console.error('Error fetching project:', error)
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error occurred'
    }
  }
}
