// Project Markers Management
// Handles .ludolingo.json marker files for project identification

import { writeTextFile, readTextFile, exists, remove } from '@tauri-apps/plugin-fs'

export interface ProjectMarker {
  projectId: number
  createdAt: string
  version: string
}

/**
 * Create a project marker file in the game directory
 */
export async function createProjectMarker(projectPath: string, projectId: number): Promise<void> {
  try {
    const markerData: ProjectMarker = {
      projectId,
      createdAt: new Date().toISOString(),
      version: '1.0'
    }

    const markerPath = `${projectPath}/.ludolingo.json`
    await writeTextFile(markerPath, JSON.stringify(markerData, null, 2))
    console.log(`📄 Project marker created: ${markerPath}`)
  } catch (error) {
    console.warn('Failed to create project marker:', error)
    // Don't throw - marker creation is not critical for project functionality
  }
}

/**
 * Read project marker file from game directory
 */
export async function readProjectMarker(projectPath: string): Promise<ProjectMarker | null> {
  try {
    const markerPath = `${projectPath}/.ludolingo.json`
    const existsMarker = await exists(markerPath)

    if (!existsMarker) {
      return null
    }

    const markerContent = await readTextFile(markerPath)
    const markerData: ProjectMarker = JSON.parse(markerContent)

    // Validate marker structure
    if (markerData.projectId && typeof markerData.projectId === 'number') {
      return markerData
    }

    console.warn('Invalid project marker structure:', markerData)
    return null
  } catch (error) {
    console.warn('Failed to read project marker:', error)
    return null
  }
}

/**
 * Delete project marker file from game directory
 */
export async function deleteProjectMarker(projectPath: string): Promise<void> {
  try {
    const markerPath = `${projectPath}/.ludolingo.json`
    await remove(markerPath)
    console.log(`🗑️ Project marker deleted: ${markerPath}`)
  } catch (error) {
    console.warn('Failed to delete project marker:', error)
    // Don't throw - marker deletion is not critical
  }
}

/**
 * Check if a directory has a valid project marker
 */
export async function hasProjectMarker(projectPath: string): Promise<boolean> {
  const marker = await readProjectMarker(projectPath)
  return marker !== null
}
