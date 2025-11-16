// Composable unifié pour la gestion de la locale (Nuxt UI + Messages)
// Basé sur https://ui.nuxt.com/docs/getting-started/integrations/i18n/nuxt

import * as locales from '@nuxt/ui/locale'
import { messages } from '~/i18n/messages'
import type { SupportedLanguage } from '~/i18n/locales'
import { ref, computed, type ComputedRef } from 'vue'

// État global partagé entre toutes les instances
const currentLocaleCode = ref<SupportedLanguage>('fr')

export function useAppLocale() {
  /**
   * Locale Nuxt UI actuelle (réactive)
   */
  const locale = computed(() => {
    const localesMap = locales as Record<string, typeof locales.fr>
    return localesMap[currentLocaleCode.value] || locales.fr
  })

  /**
   * Attributs HTML pour lang et dir
   */
  const lang = computed(() => locale.value.code)
  const dir = computed(() => locale.value.dir)

  /**
   * Changer la locale
   */
  function setLocale(localeCode: SupportedLanguage) {
    currentLocaleCode.value = localeCode
  }

  // ==========================================
  // Fonctions pour les messages personnalisés
  // ==========================================

  /**
   * Récupère un message selon la langue actuelle
   */
  function getMessage(...keys: string[]): unknown {
    const currentLang = currentLocaleCode.value
    let value: unknown = messages[currentLang]

    for (const key of keys) {
      if (value && typeof value === 'object' && key in value) {
        value = (value as Record<string, unknown>)[key]
      } else {
        // Fallback vers l'anglais si la clé n'existe pas
        value = messages.en
        for (const fallbackKey of keys) {
          if (value && typeof value === 'object' && fallbackKey in value) {
            value = (value as Record<string, unknown>)[fallbackKey]
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
   * Interpolation de variables dans les messages
   */
  function interpolate(message: string, params: Record<string, string | number>): string {
    return message.replace(/{(\w+)}/g, (match, key) => {
      return params[key] !== undefined ? String(params[key]) : match
    })
  }

  /**
   * Message avec interpolation (synchrone, retourne string directement)
   */
  function tm(...args: Array<string | Record<string, string | number>>): string {
    const params = typeof args[args.length - 1] === 'object' ? args[args.length - 1] as Record<string, string | number> : {}
    const keys = typeof args[args.length - 1] === 'object' ? args.slice(0, -1) as string[] : args as string[]
    const message = getMessage(...keys)

    if (typeof message === 'string') {
      return interpolate(message, params)
    }

    return String(message)
  }

  /**
   * Message avec interpolation (réactif via computed)
   */
  function tmReactive(...args: Array<string | Record<string, string | number>>): ComputedRef<string> {
    return computed(() => {
      const params = typeof args[args.length - 1] === 'object' ? args[args.length - 1] as Record<string, string | number> : {}
      const keys = typeof args[args.length - 1] === 'object' ? args.slice(0, -1) as string[] : args as string[]

      const currentLang = currentLocaleCode.value
      let value: unknown = messages[currentLang]

      for (const key of keys) {
        if (value && typeof value === 'object' && key in value) {
          value = (value as Record<string, unknown>)[key]
        } else {
          value = messages.en
          for (const fallbackKey of keys) {
            if (value && typeof value === 'object' && fallbackKey in value) {
              value = (value as Record<string, unknown>)[fallbackKey]
            } else {
              return `Missing translation: ${keys.join('.')}`
            }
          }
          break
        }
      }

      if (typeof value === 'string') {
        return interpolate(value, params)
      }

      return String(value)
    })
  }

  return {
    // Locale Nuxt UI
    locale,
    currentLocaleCode,
    lang,
    dir,
    setLocale,
    
    // Messages personnalisés
    getMessage,
    tm,
    tmReactive,
    interpolate
  }
}

