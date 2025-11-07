// Composable pour les messages personnalisés de LudoLingo
// Utilise les textes selon la langue sélectionnée dans les paramètres

import { messages } from '~/i18n/messages'
import { type SupportedLanguage } from '~/i18n/locales'
import { useSettingsStore } from '~/stores/settings'

export function useMessages() {
  const settingsStore = useSettingsStore()

  /**
   * Récupère un message selon la langue actuelle
   */
  function getMessage(...keys: string[]): any {
    const currentLang = settingsStore.settings.ui.language as SupportedLanguage
    let value: any = messages[currentLang]

    for (const key of keys) {
      if (value && typeof value === 'object' && key in value) {
        value = value[key]
      } else {
        // Fallback vers l'anglais si la clé n'existe pas
        value = messages.en
        for (const fallbackKey of keys) {
          if (value && typeof value === 'object' && fallbackKey in value) {
            value = value[fallbackKey]
          } else {
            return `Missing translation: ${keys.join('.')}`
          }
        }
        break
      }
    }

    return value
  }

  /**
   * Version réactive du message
   */
  function t(...keys: string[]): ComputedRef<any> {
    return computed(() => getMessage(...keys))
  }

  /**
   * Interpolation de variables dans les messages
   */
  function interpolate(message: string, params: Record<string, any>): string {
    return message.replace(/{(\w+)}/g, (match, key) => {
      return params[key] !== undefined ? String(params[key]) : match
    })
  }

  /**
   * Message avec interpolation
   */
  function tm(...args: any[]): string {
    const params = typeof args[args.length - 1] === 'object' ? args.pop() : {}
    const keys = args
    const message = getMessage(...keys)

    if (typeof message === 'string') {
      return interpolate(message, params)
    }

    return String(message)
  }

  /**
   * Version réactive avec interpolation
   */
  function tmReactive(...args: any[]): ComputedRef<string> {
    return computed(() => tm(...args))
  }

  return {
    getMessage,
    t,
    tm,
    tmReactive,
    interpolate
  }
}
