// Scan progress store using Pinia
// Manages scanning state and progress across the application

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ScanProgress, ScanHistoryEntry } from '~/types/scanning-commands'

export const useScanStore = defineStore('scan', () => {
  // State
  const currentScan = ref<ScanProgress | null>(null)
  const scanHistory = ref<ScanHistoryEntry[]>([])
  const isScanningDialogOpen = ref(false)

  // Getters
  const isScanning = computed(() => {
    return currentScan.value?.status === 'InProgress'
  })

  const isScanCompleted = computed(() => {
    return currentScan.value?.status === 'Completed'
  })

  const isScanFailed = computed(() => {
    return currentScan.value?.status === 'Failed'
  })

  const currentProgress = computed(() => {
    if (!currentScan.value) return 0
    const progress = currentScan.value
    return progress.total_files > 0
      ? (progress.files_processed / progress.total_files) * 100
      : 0
  })

  const hasErrors = computed(() => {
    return currentScan.value?.errors.length ?? 0 > 0
  })

  const recentScans = computed(() => {
    return scanHistory.value
      .filter(scan => scan.status !== 'InProgress')
      .sort((a, b) => {
        // Sort by completion time (most recent first)
        return new Date(b.completed_at || 0).getTime() - new Date(a.completed_at || 0).getTime()
      })
      .slice(0, 10) // Keep only 10 most recent
  })

  const totalEntriesFound = computed(() => {
    return scanHistory.value
      .filter(scan => scan.status === 'Completed')
      .reduce((total, scan) => total + scan.entries_extracted, 0)
  })

  // Actions
  function startScan(scanId: string, projectId: number, folderPath: string) {
    currentScan.value = {
      scan_id: scanId,
      current_file: '',
      files_processed: 0,
      total_files: 0,
      entries_extracted: 0,
      errors: [],
      status: 'InProgress'
    }
  }

  function updateScanProgress(progress: ScanProgress) {
    currentScan.value = progress

    // If scan is completed or failed, add to history
    if (progress.status === 'Completed' || progress.status === 'Failed') {
      const historyEntry: ScanHistoryEntry = {
        ...progress,
        completed_at: new Date().toISOString()
      }
      scanHistory.value.push(historyEntry)
    }
  }

  function completeScan(entriesFound: number) {
    if (currentScan.value) {
      currentScan.value.status = 'Completed'
      currentScan.value.entries_extracted = entriesFound
      currentScan.value.current_file = 'Scan completed'

      // Add to history
      const historyEntry: ScanHistoryEntry = {
        ...currentScan.value,
        completed_at: new Date().toISOString()
      }
      scanHistory.value.push(historyEntry)
    }
  }

  function failScan(errors: string[]) {
    if (currentScan.value) {
      currentScan.value.status = 'Failed'
      currentScan.value.errors = errors

      // Add to history
      const historyEntry: ScanHistoryEntry = {
        ...currentScan.value,
        completed_at: new Date().toISOString()
      }
      scanHistory.value.push(historyEntry)
    }
  }

  function cancelScan() {
    if (currentScan.value && currentScan.value.status === 'InProgress') {
      currentScan.value.status = 'Failed'
      currentScan.value.errors = ['Scan cancelled by user']

      // Add to history
      const historyEntry: ScanHistoryEntry = {
        ...currentScan.value,
        completed_at: new Date().toISOString()
      }
      scanHistory.value.push(historyEntry)
    }
  }

  function resetCurrentScan() {
    currentScan.value = null
  }

  function openScanningDialog() {
    isScanningDialogOpen.value = true
  }

  function closeScanningDialog() {
    isScanningDialogOpen.value = false
  }

  function clearHistory() {
    scanHistory.value = []
  }

  function removeFromHistory(scanId: string) {
    const index = scanHistory.value.findIndex(scan => scan.scan_id === scanId)
    if (index > -1) {
      scanHistory.value.splice(index, 1)
    }
  }

  // Utility functions
  function getScanById(scanId: string): ScanHistoryEntry | undefined {
    return scanHistory.value.find(scan => scan.scan_id === scanId)
  }

  function getSuccessfulScans(): ScanHistoryEntry[] {
    return scanHistory.value.filter(scan => scan.status === 'Completed')
  }

  function getFailedScans(): ScanHistoryEntry[] {
    return scanHistory.value.filter(scan => scan.status === 'Failed')
  }

  return {
    // State
    currentScan,
    scanHistory,
    isScanningDialogOpen,

    // Getters
    isScanning,
    isScanCompleted,
    isScanFailed,
    currentProgress,
    hasErrors,
    recentScans,
    totalEntriesFound,

    // Actions
    startScan,
    updateScanProgress,
    completeScan,
    failScan,
    cancelScan,
    resetCurrentScan,
    openScanningDialog,
    closeScanningDialog,
    clearHistory,
    removeFromHistory,

    // Utilities
    getScanById,
    getSuccessfulScans,
    getFailedScans
  }
})
