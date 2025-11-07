<template>
  <UContainer class="py-8">
    <!-- Header -->
    <div class="text-center mb-12">
      <UIcon name="i-heroicons-heart" class="h-16 w-16 mx-auto mb-6 text-pink-500" />
      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
        {{ tm('donations', 'title') }}
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
        {{ tm('donations', 'subtitle') }}
      </p>
    </div>

    <!-- Donation Options -->
    <div class="max-w-4xl mx-auto">
      <!-- Why Donate Section -->
      <UCard class="mb-8">
        <template #header>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
            {{ tm('donations', 'whyDonate') }}
          </h2>
        </template>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="flex items-start gap-3">
            <UIcon name="i-heroicons-code-bracket" class="h-6 w-6 text-primary mt-1" />
            <div>
              <h3 class="font-medium text-gray-900 dark:text-white">{{ tm('donations', 'featureDev') }}</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">{{ tm('donations', 'featureDevDesc') }}</p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <UIcon name="i-heroicons-server" class="h-6 w-6 text-primary mt-1" />
            <div>
              <h3 class="font-medium text-gray-900 dark:text-white">{{ tm('donations', 'infrastructure') }}</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">{{ tm('donations', 'infrastructureDesc') }}</p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <UIcon name="i-heroicons-language" class="h-6 w-6 text-primary mt-1" />
            <div>
              <h3 class="font-medium text-gray-900 dark:text-white">{{ tm('donations', 'localization') }}</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">{{ tm('donations', 'localizationDesc') }}</p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <UIcon name="i-heroicons-user-group" class="h-6 w-6 text-primary mt-1" />
            <div>
              <h3 class="font-medium text-gray-900 dark:text-white">{{ tm('donations', 'community') }}</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">{{ tm('donations', 'communityDesc') }}</p>
            </div>
          </div>
        </div>
      </UCard>

      <!-- Donation Amounts -->
      <UCard class="mb-8">
        <template #header>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
            {{ tm('donations', 'chooseAmount') }}
          </h2>
        </template>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <UButton
            v-for="amount in predefinedAmounts"
            :key="amount"
            :variant="selectedAmount === amount ? 'solid' : 'outline'"
            color="primary"
            size="lg"
            class="py-4"
            @click="selectAmount(amount)"
          >
            {{ amount }}€
          </UButton>
        </div>

        <!-- Custom Amount -->
        <div class="border-t pt-6">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">
            {{ tm('donations', 'customAmount') }}
          </h3>
          <UInput
            v-model.number="customAmount"
            type="number"
            :placeholder="tm('donations', 'enterAmount')"
            size="lg"
            class="max-w-xs"
            @input="selectCustomAmount"
          />
        </div>
      </UCard>

      <!-- Donation Button -->
      <div class="text-center">
        <UButton
          color="error"
          size="xl"
          icon="i-heroicons-heart"
          :disabled="!selectedAmount || selectedAmount <= 0"
          class="px-8 py-4 text-lg"
          @click="handleDonate"
        >
          {{ selectedAmount && selectedAmount > 0 ? tm('donations', 'donateAmount', { amount: selectedAmount }) : 'Donate' }}
        </UButton>

        <p class="text-sm text-gray-500 dark:text-gray-400 mt-4">
          {{ tm('donations', 'securePayment') }}
        </p>
      </div>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
import { useMessages } from '~/composables/useMessages'

// Reactive state
const selectedAmount = ref<number | null>(null)
const customAmount = ref<number | null>(null)

const { tm } = useMessages()

// Predefined donation amounts
const predefinedAmounts = [5, 10, 20, 50]

// Methods
function selectAmount(amount: number) {
  selectedAmount.value = amount
  customAmount.value = null
}

function selectCustomAmount() {
  if (customAmount.value && customAmount.value > 0) {
    selectedAmount.value = customAmount.value
  } else {
    selectedAmount.value = null
  }
}

async function handleDonate() {
  if (!selectedAmount.value || selectedAmount.value <= 0) {
    return
  }

  try {
    // TODO: Implement Stripe Payment Links integration
    // This would typically create a payment link with the selected amount
    console.log(`Processing donation of ${selectedAmount.value}€...`)

    // For now, just show a message
    alert(`Thank you for your donation of ${selectedAmount.value}€! (This is a placeholder - Stripe integration coming soon)`)
  } catch (error) {
    console.error('Donation error:', error)
  }
}

// Watch for custom amount changes
watch(customAmount, selectCustomAmount)
</script>
