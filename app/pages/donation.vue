<template>
  <UContainer class="py-8">
    <!-- Header -->
    <div class="text-center mb-12">
      <UIcon name="i-heroicons-heart" class="h-16 w-16 mx-auto mb-6 text-pink-500" />
      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
        {{ tmReactive('donations', 'title').value }}
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
        {{ tmReactive('donations', 'subtitle').value }}
      </p>
    </div>

    <!-- Donation Options -->
    <div class="max-w-4xl mx-auto">
      <!-- Why Donate Section -->
      <UCard class="mb-8">
        <template #header>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
            {{ tmReactive('donations', 'whyDonate').value }}
          </h2>
        </template>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="flex items-start gap-3">
            <UIcon name="i-heroicons-code-bracket" class="h-6 w-6 text-primary mt-1" />
            <div>
              <h3 class="font-medium text-gray-900 dark:text-white">{{ tmReactive('donations', 'featureDev').value }}</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">{{ tmReactive('donations', 'featureDevDesc').value }}</p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <UIcon name="i-heroicons-server" class="h-6 w-6 text-primary mt-1" />
            <div>
              <h3 class="font-medium text-gray-900 dark:text-white">{{ tmReactive('donations', 'infrastructure').value }}</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">{{ tmReactive('donations', 'infrastructureDesc').value }}</p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <UIcon name="i-heroicons-language" class="h-6 w-6 text-primary mt-1" />
            <div>
              <h3 class="font-medium text-gray-900 dark:text-white">{{ tmReactive('donations', 'localization').value }}</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">{{ tmReactive('donations', 'localizationDesc').value }}</p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <UIcon name="i-heroicons-user-group" class="h-6 w-6 text-primary mt-1" />
            <div>
              <h3 class="font-medium text-gray-900 dark:text-white">{{ tmReactive('donations', 'community').value }}</h3>
              <p class="text-sm text-gray-600 dark:text-gray-300">{{ tmReactive('donations', 'communityDesc').value }}</p>
            </div>
          </div>
        </div>
      </UCard>

      <!-- Donation Amounts -->
      <UCard class="mb-8">
        <template #header>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
            {{ tmReactive('donations', 'chooseAmount').value }}
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
            {{ tmReactive('donations', 'customAmount').value }}
          </h3>
          <UInput
            v-model.number="customAmount"
            type="number"
            :placeholder="tmReactive('donations', 'enterAmount').value"
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
          {{ selectedAmount && selectedAmount > 0 ? tmReactive('donations', 'donateAmount', { amount: selectedAmount }) : 'Donate' }}
        </UButton>

        <p class="text-sm text-gray-500 dark:text-gray-400 mt-4">
          {{ tmReactive('donations', 'securePayment').value }}
        </p>
      </div>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
import { useAppLocale } from '~/composables/useLocale'

// Reactive state
const selectedAmount = ref<number | null>(null)
const customAmount = ref<number | null>(null)

const { tmReactive } = useAppLocale()

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
