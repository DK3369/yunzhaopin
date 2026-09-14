<template>
  <div class="member-field">
    <div v-if="label" class="verification_formname">{{ label }}</div>
    <div class="verification_form_code" :class="{ 'verification_form_code--area': area }">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  label?: string
  area?: boolean
}>()
</script>
