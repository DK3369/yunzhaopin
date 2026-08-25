<template>
  <NuxtLink :to="to" class="simple-card">
    <h3>{{ title }}</h3>
    <p v-if="meta" class="meta">{{ meta }}</p>
  </NuxtLink>
</template>

<script setup lang="ts">
defineProps<{
  to: string
  title: string
  meta?: string
}>()
</script>

<style scoped>
.simple-card {
  display: block;
  padding: 1rem;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  text-decoration: none;
  color: inherit;
}
.simple-card:hover { border-color: #2563eb; }
h3 { margin: 0; font-size: 1rem; }
.meta { margin: 0.25rem 0 0; color: #6b7280; font-size: 0.9rem; }
</style>
