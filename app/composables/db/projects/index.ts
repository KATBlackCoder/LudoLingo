// Projects database operations
// Provides frontend interface to project management

import { invoke } from '@tauri-apps/api/core'
import { useProjectsStore, type Project, type CreateProjectData } from '~/stores/projects'

/**
 * Get all projects
 */
export async function getProjects(): Promise<Project[]> {
  const store = useProjectsStore()
  await store.initialize()
  return store.projects
}

/**
 * Get project by ID
 */
export async function getProjectById(projectId: number): Promise<Project | null> {
  const store = useProjectsStore()
  await store.initialize()
  return store.projects.find(p => p.id === projectId) || null
}

/**
 * Create a new project
 */
export async function createProject(data: CreateProjectData): Promise<Project> {
  const store = useProjectsStore()
  return await store.createProject(data)
}

/**
 * Update project statistics
 */
export async function updateProjectStats(
  projectId: number,
  totalTexts: number,
  translatedTexts: number
): Promise<void> {
  const store = useProjectsStore()
  await store.updateProjectStats(projectId, totalTexts, translatedTexts)
}

/**
 * Delete a project
 */
export async function deleteProject(projectId: number): Promise<void> {
  const store = useProjectsStore()
  await store.deleteProject(projectId)
}

/**
 * Validate project name
 */
export async function validateProjectName(name: string): Promise<void> {
  return await invoke('validate_project_name', { name })
}

/**
 * Validate game path
 */
export async function validateGamePath(path: string): Promise<void> {
  return await invoke('validate_game_path', { path })
}

/**
 * Get current project from store
 */
export async function getCurrentProject(): Promise<Project | null> {
  const store = useProjectsStore()
  await store.initialize()
  return store.currentProject
}

/**
 * Set current project
 */
export async function setCurrentProject(projectId: number): Promise<void> {
  const store = useProjectsStore()
  await store.setCurrentProject(projectId)
}

/**
 * Get recent projects (last 5 accessed)
 */
export async function getRecentProjects(): Promise<Project[]> {
  const store = useProjectsStore()
  await store.initialize()
  return store.recentProjects
}

/**
 * Initialize projects system
 * Call this once at app startup
 */
export async function initializeProjects(): Promise<void> {
  const store = useProjectsStore()
  await store.initialize()
}
