/**
 * Composable for automatic translation detection during text extraction
 * Detects already translated texts in CJK → non-CJK workflows
 */

import { useSettings } from '~/composables/useTauriSetting'
import * as cjk from 'cjk-regex'
import type { TextEntry } from '~/types/scanning-commands'

export function useAutoTranslationDetection() {
  const settings = useSettings()

  /**
   * Applies automatic translation detection to extracted texts
   * Only processes CJK → non-CJK language pairs
   * @param texts Array of extracted TextEntry objects
   * @returns Promise<TextEntry[]> Processed texts with updated statuses
   */
  const applyAutoTranslationDetection = async (texts: TextEntry[]): Promise<TextEntry[]> => {
    // 1. Récupérer les langues depuis les settings
    const userSettings = await settings.loadSettings()
    const sourceLang = userSettings.translation.sourceLanguage
    const targetLang = userSettings.translation.targetLanguage

    // 2. Vérifier que source est CJK et cible non-CJK
    const isSourceCJK = ['ja', 'zh', 'ko'].includes(sourceLang.toLowerCase())
    const isTargetCJK = ['ja', 'zh', 'ko'].includes(targetLang.toLowerCase())

    if (!isSourceCJK || isTargetCJK) {
      console.log(`🔍 Auto-détection ignorée: ${sourceLang} → ${targetLang} (non CJK→non-CJK)`)
      return texts // Pas de détection automatique
    }

    console.log(`🔍 Auto-détection activée: ${sourceLang} → ${targetLang}`)

    // 3. Traiter chaque texte extrait
    let detectedCount = 0
    const processedTexts = texts.map(text => {
      const processedText = { ...text }

      // Nettoyer les placeholders [CODE_*]
      const cleanText = text.source_text.replace(/\[[A-Z_][A-Z0-9_]*(?:_\d+)*(?:_[A-Z0-9_]+)*\]/g, '').trim()

      // Vérifier que le texte n'est pas vide après nettoyage
      if (cleanText.length === 0) {
        return processedText // Texte vide, garder status original
      }

      // Vérifier que le texte NE CONTIENT PAS de caractères CJK
      const cjkRegex = cjk.all().toRegExp()
      const hasCJK = cjkRegex.test(cleanText)

      // Si pas de CJK = probablement déjà traduit (textes, symboles, ponctuation universels)
      if (!hasCJK) {
        processedText.status = 'Translated'
        processedText.translated_text = text.source_text // Copier source vers traduction
        detectedCount++
      }

      return processedText
    })

    console.log(`✅ Auto-détection terminée: ${detectedCount} textes marqués comme traduits sur ${texts.length}`)
    return processedTexts
  }

  return {
    applyAutoTranslationDetection
  }
}
