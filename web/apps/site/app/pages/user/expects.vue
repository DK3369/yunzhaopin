<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('expects', () => api.post('/v1/mcenter/resume/expects/list', {}))
useSeoMeta({ title: t('home.intention') })
</script>

<template>
  <section>
    <h1>{{ $t('home.intention') }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <p v-else-if="!(data?.list || data || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
    <article v-for="row in data?.list || data || []" :key="row.id" class="job-card">
      <h3>{{ row.name }}</h3>
      <p class="muted">{{ row.job_classid_n }} · {{ row.city_classid_n }}</p>
    </article>
  </section>
</template>
