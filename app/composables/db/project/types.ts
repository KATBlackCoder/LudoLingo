// Types spécifiques aux opérations CRUD des projets
// Préparé pour future implémentation

export interface ProjectDB {
  id: number
  name: string
  source_language: string
  target_language: string
  game_path: string
  game_engine?: string
  created_at: string
  updated_at: string
}

export interface CreateProjectData {
  name: string
  game_path: string
  game_engine?: string
}

export interface UpdateProjectData {
  name?: string
  source_language?: string
  target_language?: string
  game_path?: string
  game_engine?: string
}

export interface ProjectFilters {
  search?: string
  game_path?: string
  game_engine?: string
  limit?: number
  offset?: number
}

export interface ProjectListResult {
  projects: ProjectDB[]
  total: number
}

// Types pour les opérations DB
export interface DBOperationResult<T = void> {
  success: boolean
  data?: T
  error?: string
}
