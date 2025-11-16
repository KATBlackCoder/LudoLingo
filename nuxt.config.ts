export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  
  // Enable Nuxt devtools
  devtools: { enabled: true },
  
  // Disable SSR for Tauri desktop app
  ssr: false,
  
  // Development server configuration
  devServer: {
    host: '0',
  },
  
  // Vite configuration for Tauri
  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
    },
  },
  
  // Ignore Tauri source files
  ignore: ['**/src-tauri/**'],
  
  // Nuxt modules
  modules: ['@nuxt/ui', '@pinia/nuxt'],

  // Pinia configuration
  pinia: {
    storesDirs: ['./app/stores/**'],
  },
  
  // TypeScript configuration
  typescript: {
    strict: true,
    typeCheck: false, // Disabled for now to speed up dev, will enable for production
  },
  
  // CSS
  css: ['~/assets/main.css'],
});