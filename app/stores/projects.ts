// Projects store using Pinia
// Manages project state with persistence via Tauri store

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useTauriStore } from '~/composables/useTauriProject'
import { remove } from '@tauri-apps/plugin-fs'
import type { TextEntry } from '~/types/scanning-commands'
import { createBulkTextEntries, getProjectTexts as getProjectTextsFromDB, getProjectTextStats, deleteProjectTexts } from '~/composables/db/texts'
import { createProject as createProjectDB, getProjects as getProjectsFromDB, deleteProject as deleteProjectDB } from '~/composables/db/project'

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

  // Charger les projets depuis la DB SQLite avec statistiques
  const loadProjectsFromDB = async () => {
    try {
      isLoading.value = true
      error.value = null

      console.log('🔄 Chargement des projets depuis DB...')

      // Récupérer tous les projets depuis la DB
      const dbResult = await getProjectsFromDB()
      if (!dbResult.success || !dbResult.data) {
        throw new Error(dbResult.error || 'Failed to load projects from DB')
      }

      // Transformer les projets DB en projets Pinia avec statistiques
      const dbProjects = dbResult.data.projects
      const projectsWithStats: Project[] = []

      for (const dbProject of dbProjects) {
        // Obtenir les statistiques des textes pour ce projet
        const statsResult = await getProjectTextStats(dbProject.id)
        const stats = statsResult.success ? statsResult.data : { total_texts: 0, translated_texts: 0 }

        // Créer l'objet projet avec les statistiques
        const project: Project = {
          id: dbProject.id,
          name: dbProject.name,
          gamePath: dbProject.game_path,
          gameEngine: (dbProject.game_engine || 'Unknown') as 'RPG Maker MV' | 'RPG Maker MZ' | 'Unknown',
          createdAt: dbProject.created_at,
          lastAccessedAt: new Date().toISOString(),
          scanHistory: [],
          totalTexts: stats?.total_texts || 0,
          translatedTexts: stats?.translated_texts || 0,
          extractedTexts: [] // Sera chargé à la demande
        }

        projectsWithStats.push(project)
      }

      // Mettre à jour le store avec les projets de la DB
      projects.value = projectsWithStats

      console.log(`✅ ${projectsWithStats.length} projets chargés depuis DB`)

      return projectsWithStats
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load projects from DB'
      console.error('Error loading projects from DB:', err)
      return []
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

      console.log(`✅ Projet créé en DB avec ID: ${dbResult.data.id}`)

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

      // Charger les textes depuis la DB si le projet n'en a pas en mémoire
      if (project.extractedTexts.length === 0) {
        await loadProjectTextsFromDB(projectId)
      }

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
      console.log(`💾 Sauvegarde de ${texts.length} textes en DB pour le projet ${projectId}...`)
      const dbResult = await createBulkTextEntries(projectId, texts)
      if (!dbResult.success) {
        console.error('❌ Erreur sauvegarde DB:', dbResult.errors)
        throw new Error(`Erreur sauvegarde DB: ${dbResult.errors.join(', ')}`)
      }
      console.log(`✅ ${dbResult.inserted_count} textes sauvegardés en DB`)

      // Mettre à jour le store Pinia pour l'UI temps réel
      project.extractedTexts = texts
      project.totalTexts = texts.length
      project.translatedTexts = texts.filter((t: TextEntry) => t.status === 'Translated').length
      project.lastAccessedAt = new Date().toISOString()
      projectUpdated = true

      await saveProjects()
    } catch (err) {
      console.error('Error updating project texts:', err)

      // Rollback: remettre les textes originaux si la DB a échoué mais que Pinia a été modifié
      if (projectUpdated) {
        const project = projects.value.find(p => p.id === projectId)
        if (project) {
          console.log('🔄 Rollback: remise des textes originaux dans le store')
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
    try {
      console.log(`🔄 Chargement des textes depuis DB pour le projet ${projectId}...`)
      const dbResult = await getProjectTextsFromDB(projectId)
      if (!dbResult.success) {
        console.error('❌ Erreur chargement DB:', dbResult.error)
        throw new Error(`Erreur chargement DB: ${dbResult.error}`)
      }

      const texts = dbResult.data || []
      console.log(`✅ ${texts.length} textes chargés depuis DB`)

      // Mettre à jour le store Pinia
      const project = projects.value.find(p => p.id === projectId)
      if (project) {
        project.extractedTexts = texts
        project.totalTexts = texts.length
        project.translatedTexts = texts.filter((t: TextEntry) => t.status === 'Translated').length
        await saveProjects()
      }

      return texts
    } catch (err) {
      console.error('Error loading project texts from DB:', err)
      throw err
    }
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
      console.log(`🗑️ Suppression du projet ${projectId}...`)

      // Trouver le projet dans le store
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Project not found')
      }

      // 1. Supprimer les textes de la DB
      console.log('📝 Suppression des textes en DB...')
      const deleteTextsResult = await deleteProjectTexts(projectId)
      if (!deleteTextsResult.success) {
        console.warn('⚠️ Échec suppression textes:', deleteTextsResult.error)
        // On continue quand même pour supprimer le projet
      }

      // 2. Supprimer le projet de la DB
      console.log('🗃️ Suppression du projet en DB...')
      const deleteProjectResult = await deleteProjectDB(projectId)
      if (!deleteProjectResult.success) {
        throw new Error(`Failed to delete project from DB: ${deleteProjectResult.error}`)
      }

      // 3. Supprimer le fichier marqueur .ludolingo.json
      try {
        const markerPath = `${project.gamePath}/.ludolingo.json`
        await remove(markerPath)
        console.log(`📄 Fichier marqueur supprimé: ${markerPath}`)
      } catch (markerError) {
        console.warn('⚠️ Impossible de supprimer le fichier marqueur:', markerError)
        // On ne fait pas échouer la suppression pour autant
      }

      // 4. Supprimer du store Pinia
      const index = projects.value.findIndex(p => p.id === projectId)
      if (index !== -1) {
        projects.value.splice(index, 1)
      }

      // 5. Réinitialiser le projet actuel si c'était celui supprimé
      if (currentProjectId.value === projectId) {
        currentProjectId.value = projects.value.length > 0 ? projects.value[0]?.id ?? null : null
      }

      // 6. Sauvegarder le store
      await saveProjects()

      console.log(`✅ Projet ${projectId} supprimé avec succès`)
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
    loadProjectsFromDB,
    saveProjects,
    createProject,
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