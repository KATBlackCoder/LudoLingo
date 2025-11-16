// Project Management Commands Contract
// Tauri commands for project CRUD operations

export interface Project {
  id: number;
  name: string;
  description?: string;
  source_language: string;
  target_language: string;
  game_engine?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateProjectRequest {
  name: string;
  description?: string;
  source_language?: string;
  target_language?: string;
  game_engine?: string;
}

export interface UpdateProjectRequest {
  id: number;
  name?: string;
  description?: string;
  source_language?: string;
  target_language?: string;
  game_engine?: string;
}

// Tauri Commands
export interface ProjectCommands {
  // Create new project
  create_project: (request: CreateProjectRequest) => Promise<Project>;

  // Get all projects
  list_projects: () => Promise<Project[]>;

  // Get project by ID
  get_project: (id: number) => Promise<Project>;

  // Update project
  update_project: (request: UpdateProjectRequest) => Promise<Project>;

  // Delete project
  delete_project: (id: number) => Promise<void>;

  // Get project statistics
  get_project_stats: (id: number) => Promise<{
    total_files: number;
    total_entries: number;
    translated_entries: number;
    completed_percentage: number;
  }>;
}
