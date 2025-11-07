// Index automatique des langues disponibles
// Ce fichier importe toutes les langues du dossier locales/

import fr from './fr'
import en from './en'

// Objet contenant toutes les langues disponibles
export const availableLocales = {
  fr,
  en
} as const

// Type généré automatiquement depuis les langues disponibles
export type AvailableLocales = typeof availableLocales
export type SupportedLanguage = keyof AvailableLocales

// Liste des codes de langue disponibles
export const supportedLanguages: SupportedLanguage[] = Object.keys(availableLocales) as SupportedLanguage[]

// Fonction helper pour vérifier si une langue est supportée
export function isSupportedLanguage(lang: string): lang is SupportedLanguage {
  return lang in availableLocales
}

// Fonction helper pour obtenir une langue avec fallback
export function getLocale(lang: string): AvailableLocales[SupportedLanguage] | undefined {
  if (isSupportedLanguage(lang)) {
    return availableLocales[lang]
  }
  // Fallback vers le français par défaut
  return availableLocales.fr
}

// Fonction helper pour obtenir le drapeau d'une langue
export function getLocaleFlag(code: string): string {
  const flags: Record<string, string> = {
    fr: '🇫🇷',
    en: '🇺🇸',
    es: '🇪🇸',
    de: '🇩🇪',
    it: '🇮🇹',
    pt: '🇵🇹',
    ja: '🇯🇵',
    ko: '🇰🇷',
    zh: '🇨🇳'
  }
  return flags[code] || '🌐'
}
