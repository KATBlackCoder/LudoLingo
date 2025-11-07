<template>
  <UContainer class="py-8">
    <!-- Titre principal -->
    <div class="text-center mb-12">
      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
        {{ tm('welcome', 'title') }}
        </h1>
      <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
        {{ tm('welcome', 'subtitle') }}
      </p>
      </div>

      <!-- Section principale -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <ProjectSection
          @create-project="handleCreateProject"
          @scan-projects="handleScanProjects"
        />

        <SupportedGamesSection />
      </div>

      <!-- Section dons -->
      <DonationSection />
  </UContainer>
</template>

<script setup lang="ts">
import { useMessages } from '~/composables/useMessages'
import ProjectSection from '~/components/projects/ProjectSection.vue'
import SupportedGamesSection from '~/components/projects/SupportedGamesSection.vue'
import DonationSection from '~/components/projects/DonationSection.vue'

const { tm } = useMessages()

function handleCreateProject() {
  // TODO: Implement project creation logic
  console.log('Creating new project...')
}

function handleScanProjects() {
  // TODO: Implement project scanning logic
  console.log('Scanning for projects...')
}
</script>
