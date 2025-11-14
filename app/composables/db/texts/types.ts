// Text Database Operations Types
// Based on translation_entries table structure

import type { TextEntry } from '~/types/scanning-commands'

export interface DBTextEntry {
  id: number;
  project_id: number;
  game_file_id?: number;
  source_text: string;
  translated_text?: string;
  location: string;  // Structured identifier: "object_type:object_id:field"
  text_type: 'dialogue' | 'system' | 'item' | 'skill' | 'other';
  status: 'extracted' | 'translated' | 'reviewed';
  created_at: string;
  updated_at: string;
}

export interface CreateTextEntry {
  project_id: number;
  game_file_id?: number;
  source_text: string;
  translated_text?: string;
  location: string;  // Structured identifier: "object_type:object_id:field"
  text_type?: 'dialogue' | 'system' | 'item' | 'skill' | 'other';
  status?: 'extracted' | 'translated' | 'reviewed';
}

export interface UpdateTextEntry {
  id: number;
  translated_text?: string;
  location?: string;  // Structured identifier: "object_type:object_id:field"
  text_type?: 'dialogue' | 'system' | 'item' | 'skill' | 'other';
  status?: 'extracted' | 'translated' | 'reviewed';
}

export interface TextFilters {
  status?: string[];
  text_type?: string[];
  game_file_id?: number;
  limit?: number;
  offset?: number;
}

export interface TextQueryResult {
  entries: TextEntry[];
  total_count: number;
}

export interface DBGameFile {
  id: number;
  project_id: number;
  file_path: string;
  file_format: string;
  file_size: number;
  checksum: string;
  last_modified: string;
  scan_status: 'pending' | 'scanning' | 'completed' | 'error';
  created_at: string;
}

export interface CreateGameFile {
  project_id: number;
  file_path: string;
  file_format: string;
  file_size?: number;
  checksum?: string;
  last_modified?: string;
  scan_status?: 'pending' | 'scanning' | 'completed' | 'error';
}

export interface UpdateGameFile {
  id: number;
  file_size?: number;
  checksum?: string;
  last_modified?: string;
  scan_status?: 'pending' | 'scanning' | 'completed' | 'error';
}

// Result types for operations
export interface TextOperationResult<T = void> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface BulkTextOperationResult {
  success: boolean;
  inserted_count: number;
  errors: string[];
}
