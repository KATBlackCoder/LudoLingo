import { defineStore } from 'pinia'

export const useProvidersStore = defineStore('providers', () => {

  const providers = ref<string[]>([])

  return {
    providers
  }
})
