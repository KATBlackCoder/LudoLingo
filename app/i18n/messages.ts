// Messages personnalisés pour LudoLingo
// Utilise automatiquement toutes les langues disponibles dans locales/

import { availableLocales, type AvailableLocales } from './locales'

// Les messages sont directement les langues disponibles
export const messages = availableLocales

// Types dérivés automatiquement
export type Messages = AvailableLocales
