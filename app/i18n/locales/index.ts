// Langues supportées par LudoLingo
// Liste des codes de langue disponibles dans l'application

import fr from './fr'
import en from './en'

// Langues disponibles dans l'application
export const availableLocales = {
  fr,
  en
} as const

// Types générés automatiquement
export type AvailableLocales = typeof availableLocales
export type SupportedLanguage = keyof AvailableLocales

// Liste des codes de langue supportés
export const supportedLanguages: SupportedLanguage[] = Object.keys(availableLocales) as SupportedLanguage[]

// Vérification de support de langue
export function isSupportedLanguage(lang: string): lang is SupportedLanguage {
  return lang in availableLocales
}
