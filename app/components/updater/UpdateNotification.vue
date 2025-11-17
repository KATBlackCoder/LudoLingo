<template>
  <!-- Badge discret dans le header -->
  <UButton
    v-if="hasUpdate && showNotification"
    variant="ghost"
    color="primary"
    size="sm"
    icon="i-heroicons-arrow-down-tray"
    @click="handleShowDialog"
  >
    <span class="hidden sm:inline">Mise à jour disponible</span>
    <UBadge
      v-if="availableUpdate"
      color="primary"
      variant="solid"
      class="ml-2"
    >
      {{ availableUpdate.version }}
    </UBadge>
  </UButton>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useUpdaterStore } from '~/stores/updater'
import { storeToRefs } from 'pinia'

interface Props {
  autoShow?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  autoShow: true
})

const emit = defineEmits<{
  'show-dialog': []
}>()

const updaterStore = useUpdaterStore()
const { availableUpdate, hasUpdate } = storeToRefs(updaterStore)

const showNotification = ref(false)

// Afficher la notification quand une mise à jour est disponible
watch(hasUpdate, (newValue) => {
  if (newValue && props.autoShow) {
    showNotification.value = true
  }
}, { immediate: true })

const handleShowDialog = () => {
  emit('show-dialog')
}

const handleDismiss = () => {
  showNotification.value = false
}
</script>

