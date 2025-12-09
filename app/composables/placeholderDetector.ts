// Détection de codes non convertis directement depuis la DB avec plugin SQL

import Database from '@tauri-apps/plugin-sql'

export interface UnconvertedCode {
  code: string
  occurrences: number
  exampleTexts: string[]
  entryType: string
}

export interface DatabaseDetectionResult {
  unconvertedCodes: UnconvertedCode[]
  totalTextsAnalyzed: number
  analysisDuration: number
}

/**
 * Analyser les placeholders manquants dans un projet directement depuis la DB
 */
export async function analyzeProjectPlaceholders(projectId: number): Promise<DatabaseDetectionResult> {
  const startTime = Date.now()

  // Charger la base de données
  const db = await Database.load('sqlite:ludolingo.db')

  try {
    // Récupérer tous les textes du projet
    const result = await db.select<{ source_text: string; text_type: string }[]>(
      'SELECT source_text, text_type FROM translation_entries WHERE project_id = $1',
      [projectId]
    )

    const texts = result as { source_text: string; text_type: string }[]
    const totalTexts = texts.length

    const codeMap = new Map<string, { count: number; examples: string[]; entryType: string }>()

    // Patterns suspects qui pourraient être des codes de formatage
    const suspiciousPatterns = [
      /\\[a-zA-Z][a-zA-Z0-9_]*(\[[^\]]*\])?/g,  // \code, \code[number]
      /@[a-zA-Z0-9_]+/g,                        // @variable
      /[#%&][a-zA-Z0-9_]+/g,                    // #code, %code, &code
      /[|<][a-zA-Z0-9_]+[|>]/g,                 // |code|, <code>
      /\\[0-9]+/g,                             // \1, \2, etc.
    ]

    // Analyser chaque texte depuis la DB
    for (const text of texts) {
      const sourceText = text.source_text
      const entryType = text.text_type

      // Appliquer tous les patterns suspects
      for (const pattern of suspiciousPatterns) {
        const matches = sourceText.match(pattern)
        if (matches) {
          for (const match of matches) {
            // Vérifier que ce n'est pas déjà un placeholder
            if (!isPlaceholder(match)) {
              const key = `${match}|${entryType}`
              const existing = codeMap.get(key)
              if (existing) {
                existing.count++
                if (existing.examples.length < 3) {
                  existing.examples.push(sourceText)
                }
              } else {
                codeMap.set(key, {
                  count: 1,
                  examples: [sourceText],
                  entryType
                })
              }
            }
          }
        }
      }
    }

    // Convertir en tableau de résultats
    const unconvertedCodes: UnconvertedCode[] = Array.from(codeMap.entries())
      .filter(([, data]) => data.count >= 1) // Au moins 1 occurrence
      .map(([key, data]) => {
        const parts = key.split('|')
        const code = parts[0] || '' // Garantir que code n'est jamais undefined
        return {
          code,
          occurrences: data.count,
          exampleTexts: data.examples,
          entryType: data.entryType
        }
      })
      .sort((a, b) => b.occurrences - a.occurrences) // Trier par occurrences décroissantes

    const analysisDuration = Date.now() - startTime

    return {
      unconvertedCodes,
      totalTextsAnalyzed: totalTexts,
      analysisDuration
    }

  } finally {
    // Fermer la connexion DB
    await db.close()
  }
}

/**
 * Vérifier si une chaîne est déjà un placeholder
 */
function isPlaceholder(text: string): boolean {
  return text.startsWith('[') && text.endsWith(']') &&
         text.length > 2 && text.charAt(1) >= 'A' && text.charAt(1) <= 'Z'
}
