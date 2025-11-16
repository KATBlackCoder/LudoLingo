// Opérations de lecture des projets
// Implémentation des opérations CRUD avec le plugin SQL Tauri

import { useDatabase, executeQuery } from '../useDatabase'
import { executeDBOperation, buildWhereClause } from '../useDBOperation'
import type { ProjectDB, ProjectFilters, ProjectListResult, DBOperationResult } from './types'

/**
 * Récupère tous les projets avec filtres optionnels
 */
export async function getProjects(filters?: ProjectFilters): Promise<DBOperationResult<ProjectListResult>> {
  return executeDBOperation(async () => {
    const db = await useDatabase()

    // Construction de la requête avec filtres
    const baseFilters: Record<string, unknown> = {}
    if (filters?.search) {
      // Pour la recherche, on doit gérer manuellement car c'est une recherche multi-colonnes
      baseFilters._search = filters.search
    }
    if (filters?.game_path) baseFilters.game_path = filters.game_path
    if (filters?.game_engine) baseFilters.game_engine = filters.game_engine

    // Construire WHERE clause pour la recherche (gestion spéciale)
    let whereClause = ''
    const params: unknown[] = []

    if (filters?.search) {
      whereClause = 'WHERE name LIKE ?'
      const searchPattern = `%${filters.search}%`
      params.push(searchPattern)
    } else {
      whereClause = 'WHERE 1=1'
    }

    // Ajouter autres filtres
    if (filters?.game_path) {
      whereClause += ' AND game_path = ?'
      params.push(filters.game_path)
    }
    if (filters?.game_engine) {
      whereClause += ' AND game_engine = ?'
      params.push(filters.game_engine)
    }

    // Tri par date de mise à jour décroissante
    let sql = `SELECT * FROM projects ${whereClause} ORDER BY updated_at DESC`

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
    const countSql = `SELECT COUNT(*) as total FROM projects ${whereClause}`
    const countResult = await executeQuery<{ total: number }>(countSql, params)

    return {
        projects,
        total: countResult[0]?.total || 0
      }
  }, 'fetching projects')
}

/**
 * Récupère un projet par son ID
 */
export async function getProject(id: number): Promise<DBOperationResult<ProjectDB>> {
  return executeDBOperation(async () => {
    const db = await useDatabase()

    const result = await executeQuery<ProjectDB>(
      'SELECT * FROM projects WHERE id = ?',
      [id]
    )

    if (result.length === 0) {
      throw new Error(`Project with ID ${id} not found`)
    }

    return result[0]
  }, 'fetching project')
}
