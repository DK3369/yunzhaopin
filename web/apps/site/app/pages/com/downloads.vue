<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('downloads', () =>
  api.post('/v1/mcenter/resume-downloads/outbox', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_com_00235') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_com_00235') }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <article v-for="row in data?.list || []" :key="row.id || row.uid" class="job-card">
      <NuxtLink :to="`/resumes/${row.uid}`">{{ row.name || row.display_name || row.uid }}</NuxtLink>
      <p class="muted">{{ row.datetime_n }}</p>
    </article>
  </section>
</template>
