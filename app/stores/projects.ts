// Project management store
// Using Pinia setup store style

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

interface Project {
  id?: number
  name: string
  description?: string
  source_language: string
  target_language: string
  game_engine?: string
  created_at?: string
  updated_at?: string
}

export const useProjectsStore = defineStore('projects', () => {
  // State
  const projects = ref<Project[]>([])
  const currentProject = ref<Project | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const projectCount = computed(() => projects.value.length)
  const hasProjects = computed(() => projects.value.length > 0)

  // Actions
  async function loadProjects() {
    loading.value = true
    error.value = null
    try {
      // TODO: Implement database query via useDatabase composable
      // const db = await useDatabase()
      // projects.value = await db.select('SELECT * FROM projects')
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load projects'
    } finally {
      loading.value = false
    }
  }

  function setCurrentProject(project: Project | null) {
    currentProject.value = project
  }

  function clearError() {
    error.value = null
  }

  return {
    // State
    projects,
    currentProject,
    loading,
    error,
    
    // Getters
    projectCount,
    hasProjects,
    
    // Actions
    loadProjects,
    setCurrentProject,
    clearError,
  }
})

