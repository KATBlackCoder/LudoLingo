// Projects store using Pinia
// Manages project state in memory (no persistence - data stored in SQLite)

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { deleteProjectMarker, readProjectMarker } from '~/composables/projectMarkers'
import { useBaseStoreState, executeAsyncOperation } from '~/composables/stores/useBaseStore'
import type { TextEntry } from '~/types/scanning-commands'
import { createBulkTextEntries, getProjectTexts as getProjectTextsFromDB, getProjectTextStats, deleteProjectTexts } from '~/composables/db/texts'
import { hasProjectTexts } from '~/composables/db/texts/create'
import { createProject as createProjectDB, getProjects as getProjectsFromDB, deleteProject as deleteProjectDB, getProject, getProjects } from '~/composables/db/project'

export interface Project {
  id: number
  name: string
  gamePath: string
  gameEngine: 'RPG Maker MV' | 'RPG Maker MZ' | 'Unknown'
  createdAt: string
  lastAccessedAt: string
  scanHistory: ProjectScan[]
  totalTexts: number
  translatedTexts: number
  extractedTexts: TextEntry[] // Textes extraits persistés
}

export interface ProjectScan {
  id: string
  timestamp: string
  totalTexts: number
  status: 'completed' | 'failed' | 'in_progress'
}

export interface CreateProjectData {
  name: string
  gamePath: string
  gameEngine: string
}

export const useProjectsStore = defineStore('projects', () => {
  // Base store state (isLoading, error, clearError)
  const { isLoading, error, clearError } = useBaseStoreState()
  
  // State
  const projects = ref<Project[]>([])
  const currentProjectId = ref<number | null>(null)

  // Getters
  const currentProject = computed(() => {
    if (currentProjectId.value === null) return null
    return projects.value.find(p => p.id === currentProjectId.value) || null
  })

  const recentProjects = computed(() => {
    return [...projects.value]
      .sort((a, b) => new Date(b.lastAccessedAt).getTime() - new Date(a.lastAccessedAt).getTime())
      .slice(0, 5)
  })

  const totalProjects = computed(() => projects.value.length)

  // Helper for load operations with consistent error handling
  const executeLoadOperation = async <T>(
    operation: () => Promise<T>,
    errorMessage: string,
    options: { skipLoading?: boolean } = {}
  ): Promise<T> => {
    return executeAsyncOperation(
      operation,
      errorMessage,
      { isLoading, error },
      options
    )
  }

  // Actions

  // Charger les projets depuis la DB SQLite avec statistiques
  const loadProjectsFromDB = async () => {
    return executeLoadOperation(async () => {

      // Récupérer tous les projets depuis la DB
      const dbResult = await getProjectsFromDB()
      if (!dbResult.success || !dbResult.data) {
        throw new Error(dbResult.error || 'Failed to load projects from DB')
      }

      // Transformer les projets DB en projets Pinia avec statistiques
      const dbProjects = dbResult.data.projects
      const projectsWithStats: Project[] = []
      
      // Créer un map des projets existants pour préserver leurs extractedTexts
      const existingProjectsMap = new Map(projects.value.map(p => [p.id, p]))

      for (const dbProject of dbProjects) {
        // Obtenir les statistiques des textes pour ce projet
        const statsResult = await getProjectTextStats(dbProject.id)
        const stats = statsResult.success ? statsResult.data : { total_texts: 0, translated_texts: 0 }

        // Utiliser le helper pour transformer le projet
        const project = dbProjectToStoreProject(dbProject)
        project.totalTexts = stats?.total_texts || 0
        project.translatedTexts = stats?.translated_texts || 0
        
        // Préserver les extractedTexts existants si le projet est déjà en mémoire
        const existingProject = existingProjectsMap.get(dbProject.id)
        if (existingProject && existingProject.extractedTexts.length > 0) {
          project.extractedTexts = existingProject.extractedTexts
        }

        projectsWithStats.push(project)
      }

      // Mettre à jour le store avec les projets de la DB (en préservant les textes chargés)
      projects.value = projectsWithStats


      return projectsWithStats
    }, 'Failed to load projects from DB').catch(() => [])
  }


  const createProject = async (data: CreateProjectData): Promise<Project> => {
    try {
      isLoading.value = true
      error.value = null

      // Validate project name
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('validate_project_name', { name: data.name })

      // Validate game path
      await invoke('validate_game_path', { path: data.gamePath })

      // Detect game engine
      const { validateFileFormat } = await import('~/composables/db/scanning')
      const validation = await validateFileFormat(data.gamePath)

      const gameEngine = validation.supported && validation.detected_engine
        ? (validation.detected_engine.includes('MZ') ? 'RPG Maker MZ' : 'RPG Maker MV')
        : 'Unknown'

      // Créer le projet dans la base de données SQLite
      const dbResult = await createProjectDB({
        name: data.name,
        description: '',
        game_path: data.gamePath,
        game_engine: gameEngine
      })

      if (!dbResult.success || !dbResult.data) {
        throw new Error(`Échec de création du projet en DB: ${dbResult.error}`)
      }


      // Créer le projet pour le store Pinia (UI)
      const now = new Date().toISOString()
      const newProject: Project = {
        id: dbResult.data.id, // Utiliser l'ID de la DB au lieu de Date.now()
        name: data.name,
        gamePath: data.gamePath,
        gameEngine,
        createdAt: now,
        lastAccessedAt: now,
        scanHistory: [],
        totalTexts: 0,
        translatedTexts: 0,
        extractedTexts: [] // Initialiser avec un tableau vide
      }

      projects.value.push(newProject)

      return newProject
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to create project'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const setCurrentProject = async (projectId: number) => {
    try {
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Project not found')
      }

      currentProjectId.value = projectId
      project.lastAccessedAt = new Date().toISOString()

      // Charger les textes depuis la DB si le projet n'en a pas en mémoire
      if (project.extractedTexts.length === 0) {
        await loadProjectTextsFromDB(projectId)
      }

      // No persistence - data stays in memory only
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to set current project'
      throw err
    }
  }

  const updateProjectStats = async (projectId: number, totalTexts: number, translatedTexts: number) => {
    try {
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Project not found')
      }

      project.totalTexts = totalTexts
      project.translatedTexts = translatedTexts

      // No persistence - data stays in memory only
    } catch (err) {
      console.error('Error updating project stats:', err)
      throw err
    }
  }

  const updateProjectTexts = async (projectId: number, texts: TextEntry[]) => {
    let projectUpdated = false
    const originalTexts: TextEntry[] = []

    try {
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Project not found')
      }

      // Sauvegarder l'état original pour rollback
      originalTexts.push(...project.extractedTexts)

      // Sauvegarder les textes en base de données
      const dbResult = await createBulkTextEntries(projectId, texts)
      if (!dbResult.success) {
        console.error('❌ Erreur sauvegarde DB:', dbResult.errors)
        throw new Error(`Erreur sauvegarde DB: ${dbResult.errors.join(', ')}`)
      }

      // Mettre à jour le store Pinia pour l'UI temps réel
      project.extractedTexts = texts
      project.totalTexts = texts.length
      project.translatedTexts = texts.filter((t: TextEntry) => t.status === 'Translated').length
      project.lastAccessedAt = new Date().toISOString()
      projectUpdated = true

      // No persistence - data stays in memory only
    } catch (err) {
      console.error('Error updating project texts:', err)

      // Rollback: remettre les textes originaux si la DB a échoué mais que Pinia a été modifié
      if (projectUpdated) {
        const project = projects.value.find(p => p.id === projectId)
        if (project) {
          project.extractedTexts = originalTexts
          project.totalTexts = originalTexts.length
          project.translatedTexts = originalTexts.filter((t: TextEntry) => t.status === 'Translated').length
        }
      }

      throw err
    }
  }

  const getProjectTexts = (projectId: number): TextEntry[] => {
    const project = projects.value.find(p => p.id === projectId)
    return project?.extractedTexts || []
  }

  // Charger les textes d'un projet depuis la base de données
  const loadProjectTextsFromDB = async (projectId: number): Promise<TextEntry[]> => {
    return executeLoadOperation(async () => {
      const dbResult = await getProjectTextsFromDB(projectId)
      if (!dbResult.success) {
        console.error('❌ Erreur chargement DB:', dbResult.error)
        throw new Error(`Erreur chargement DB: ${dbResult.error}`)
      }

      const texts = dbResult.data || []

      // Mettre à jour le store Pinia
      const project = projects.value.find(p => p.id === projectId)
      if (project) {
        project.extractedTexts = texts
        project.totalTexts = texts.length
        project.translatedTexts = texts.filter((t: TextEntry) => t.status === 'Translated').length
            // No persistence - data stays in memory only
      }

      return texts
    }, `Failed to load project texts for project ${projectId}`, { skipLoading: true })
  }

  const addScanToHistory = async (projectId: number, scan: ProjectScan) => {
    try {
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Project not found')
      }

      project.scanHistory.unshift(scan) // Add to beginning
      project.scanHistory = project.scanHistory.slice(0, 10) // Keep only last 10

      // No persistence - data stays in memory only
    } catch (err) {
      console.error('Error adding scan to history:', err)
      throw err
    }
  }

  const deleteProject = async (projectId: number) => {
    try {

      // Trouver le projet dans le store
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Project not found')
      }

      // 1. Supprimer les textes de la DB
      const deleteTextsResult = await deleteProjectTexts(projectId)
      if (!deleteTextsResult.success) {
        console.warn('⚠️ Échec suppression textes:', deleteTextsResult.error)
        // On continue quand même pour supprimer le projet
      }

      // 2. Supprimer le projet de la DB
      const deleteProjectResult = await deleteProjectDB(projectId)
      if (!deleteProjectResult.success) {
        throw new Error(`Failed to delete project from DB: ${deleteProjectResult.error}`)
      }

      // 3. Supprimer le fichier marqueur .ludolingo.json
      await deleteProjectMarker(project.gamePath)

      // 4. Supprimer du store Pinia
      const index = projects.value.findIndex(p => p.id === projectId)
      if (index !== -1) {
        projects.value.splice(index, 1)
      }

      // 5. Réinitialiser le projet actuel si c'était celui supprimé
      if (currentProjectId.value === projectId) {
        currentProjectId.value = projects.value.length > 0 ? projects.value[0]?.id ?? null : null
      }

          // 6. No persistence needed - data stays in memory

    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to delete project'
      throw err
    }
  }

  // Load or create project from game path (centralized logic)
  const loadOrCreateProject = async (gamePath: string, projectName: string): Promise<Project> => {
    return executeLoadOperation(async () => {
      // Check if project already exists in store
      let project = projects.value.find(p => p.gamePath === gamePath)

      if (!project) {
        // Check project marker first for quick validation
        const markerData = await readProjectMarker(gamePath)

        if (markerData) {
          // Marker exists, check if project in DB has texts
          const projectExistsWithTexts = await hasProjectTexts(markerData.projectId)

          if (projectExistsWithTexts) {
            // Load project from DB using marker ID
            const dbResult = await getProject(markerData.projectId)

            if (dbResult.success && dbResult.data) {
              const dbProject = dbResult.data
              project = dbProjectToStoreProject(dbProject)
              projects.value.push(project)
            }
          }
        }

        // If no project loaded from marker, check DB or create new
        if (!project) {
          const dbResult = await getProjects({ game_path: gamePath })

          if (dbResult.success && dbResult.data && dbResult.data.projects.length > 0) {
            // Project exists in DB, load it into store
            const dbProject = dbResult.data.projects[0]!
            project = dbProjectToStoreProject(dbProject)
            projects.value.push(project)
          } else {
            // Create new project
            project = await createProject({
              name: projectName,
              gamePath,
              gameEngine: 'Unknown' // Will be detected automatically
            })
          }
        }
      }

      // Ensure project was created/loaded
      if (!project) {
        throw new Error('Failed to create or load project')
      }

      return project
    }, 'Failed to load or create project')
  }

  // Helper to transform DB project to store project
  const dbProjectToStoreProject = (dbProject: any): Project => {
    return {
      id: dbProject.id,
      name: dbProject.name,
      gamePath: dbProject.game_path,
      gameEngine: (dbProject.game_engine || 'Unknown') as 'RPG Maker MV' | 'RPG Maker MZ' | 'Unknown',
      createdAt: dbProject.created_at,
      lastAccessedAt: new Date().toISOString(),
      scanHistory: [],
      totalTexts: 0,
      translatedTexts: 0,
      extractedTexts: []
    }
  }


  // Initialize store on first access
  const initialize = async () => {
    // No persistence - store starts empty and gets populated from DB when needed
    // Projects are loaded on-demand via loadProjectsFromDB() calls
  }

  return {
    // State
    projects,
    currentProjectId,
    isLoading,
    error,
    
    // Getters
    currentProject,
    recentProjects,
    totalProjects,
    
    // Actions
    loadProjectsFromDB,
    createProject,
    loadOrCreateProject,
    setCurrentProject,
    updateProjectStats,
    updateProjectTexts,
    getProjectTexts,
    loadProjectTextsFromDB,
    addScanToHistory,
    deleteProject,
    clearError,
    initialize
  }
})