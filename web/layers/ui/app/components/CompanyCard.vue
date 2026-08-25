<template>
  <NuxtLink :to="`/companies/${company.uid}`" class="com-card">
    <h3>{{ company.name }}</h3>
    <p>{{ company.hy_n }} · {{ company.city_two }}</p>
  </NuxtLink>
</template>

<script setup lang="ts">
defineProps<{
  company: {
    uid: number
    name: string
    hy_n?: string
    city_two?: string
  }
}>()
</script>

<style scoped>
.com-card {
  display: block;
  padding: 1rem;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  text-decoration: none;
  color: inherit;
}
.com-card:hover { border-color: #2563eb; }
p { margin: 0.25rem 0 0; color: #6b7280; }
</style>
