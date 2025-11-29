// Review Commands Contract
// Tauri commands for translation quality review and validation

export interface ReviewIssue {
  text_id: number;
  issue_type: 'low_quality' | 'inconsistent' | 'missing_glossary' | 'too_long' | 'too_short' | 'untranslated_chars';
  severity: 'low' | 'medium' | 'high';
  message: string;
  details?: {
    expected_value?: string;
    actual_value?: string;
    suggestion?: string;
  };
}

export interface ReviewResult {
  text_id: number;
  quality_score: number; // 0.0 - 1.0
  issues: ReviewIssue[];
  criteria_scores: {
    length_ratio: number;
    glossary_usage: number;
    consistency: number;
    anomalies: number;
  };
}

export interface GlobalReviewRequest {
  project_id: number;
  text_ids?: number[]; // Optional: review specific texts, otherwise all translated texts
}

export interface GlobalReviewResult {
  total_texts: number;
  reviewed_texts: number;
  average_quality_score: number;
  quality_distribution: {
    excellent: number; // >= 0.9
    good: number; // 0.7 - 0.9
    fair: number; // 0.5 - 0.7
    poor: number; // < 0.5
  };
  total_issues: number;
  issues_by_severity: {
    high: number;
    medium: number;
    low: number;
  };
  text_reviews: ReviewResult[];
}

export interface SingleReviewRequest {
  text_id: number;
  source_text: string;
  translated_text: string;
  project_id?: number; // Optional: for glossary and consistency checks
}

// Tauri Commands
export interface ReviewCommands {
  // Review all translated texts in a project
  review_translations: (request: GlobalReviewRequest) => Promise<GlobalReviewResult>;

  // Review a single translation
  review_single_translation: (request: SingleReviewRequest) => Promise<ReviewResult>;
}

