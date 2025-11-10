// Projects store using Pinia
// Manages project state with persistence via Tauri store

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useTauriStore } from '~/composables/useTauriProject'
import type { TextEntry } from '~/types/scanning-commands'

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
  // Tauri store instance
  const tauriStore = useTauriStore({ storeName: 'ludolingo.json' })

  // State
  const projects = ref<Project[]>([])
  const currentProjectId = ref<number | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

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

  // Actions
  const loadProjects = async () => {
    try {
      isLoading.value = true
    error.value = null

      // Charger les données depuis Tauri store
      const storedProjects = await tauriStore.getItem<Project[]>('projects')
      if (storedProjects) {
        projects.value = storedProjects
      }

      const storedCurrentProjectId = await tauriStore.getItem<number>('currentProjectId')
      currentProjectId.value = storedCurrentProjectId
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load projects'
      console.error('Error loading projects:', err)
    } finally {
      isLoading.value = false
    }
  }

  const saveProjects = async () => {
    try {
      // Sauvegarder les données avec le composable Tauri store
      await tauriStore.setItemsAndSave({
        projects: projects.value,
        currentProjectId: currentProjectId.value
      })
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to save projects'
      console.error('Error saving projects:', err)
      throw err
    }
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

      const now = new Date().toISOString()
      const newProject: Project = {
        id: Date.now(), // Simple ID generation - could be improved
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
      await saveProjects()

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

      await saveProjects()
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

      await saveProjects()
    } catch (err) {
      console.error('Error updating project stats:', err)
      throw err
    }
  }

  const updateProjectTexts = async (projectId: number, texts: TextEntry[]) => {
    try {
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Project not found')
      }

      project.extractedTexts = texts
      project.totalTexts = texts.length
      project.translatedTexts = texts.filter(t => t.status === 'Translated').length
      project.lastAccessedAt = new Date().toISOString()

      await saveProjects()
    } catch (err) {
      console.error('Error updating project texts:', err)
      throw err
    }
  }

  const getProjectTexts = (projectId: number): TextEntry[] => {
    const project = projects.value.find(p => p.id === projectId)
    return project?.extractedTexts || []
  }

  const addScanToHistory = async (projectId: number, scan: ProjectScan) => {
    try {
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Project not found')
      }

      project.scanHistory.unshift(scan) // Add to beginning
      project.scanHistory = project.scanHistory.slice(0, 10) // Keep only last 10

      await saveProjects()
    } catch (err) {
      console.error('Error adding scan to history:', err)
      throw err
    }
  }

  const deleteProject = async (projectId: number) => {
    try {
      const index = projects.value.findIndex(p => p.id === projectId)
      if (index === -1) {
        throw new Error('Project not found')
      }

      projects.value.splice(index, 1)

      if (currentProjectId.value === projectId) {
        currentProjectId.value = projects.value.length > 0 ? projects.value[0]?.id ?? null : null
      }

      await saveProjects()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to delete project'
      throw err
    }
  }

  const clearError = () => {
    error.value = null
  }

  // Initialize store on first access
  const initialize = async () => {
    if (projects.value.length === 0) {
      await loadProjects()
    }
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
    loadProjects,
    saveProjects,
    createProject,
    setCurrentProject,
    updateProjectStats,
    updateProjectTexts,
    getProjectTexts,
    addScanToHistory,
    deleteProject,
    clearError,
    initialize
  }
})