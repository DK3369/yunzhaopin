<template>
  <NuxtLink :to="`/jobs/${job.id}`" class="job-card">
    <h3>{{ job.name }}</h3>
    <p class="meta">{{ job.com_name }} · {{ job.job_city_two || job.city_two }}</p>
    <p class="salary">{{ salary }}</p>
  </NuxtLink>
</template>

<script setup lang="ts">
const props = defineProps<{
  job: {
    id: number
    name: string
    com_name?: string
    job_city_two?: string
    city_two?: string
    minsalary?: number
    maxsalary?: number
  }
}>()
const salary = computed(() => {
  const min = Number(props.job.minsalary || 0)
  const max = Number(props.job.maxsalary || 0)
  if (!min && !max) return '面议'
  if (min && max) return `${min}-${max}`
  return String(min || max)
})
</script>

<style scoped>
.job-card {
  display: block;
  padding: 1rem;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  text-decoration: none;
  color: inherit;
}
.job-card:hover { border-color: #2563eb; }
.meta, .salary { margin: 0.25rem 0 0; color: #6b7280; font-size: 0.9rem; }
.salary { color: #dc2626; }
</style>
