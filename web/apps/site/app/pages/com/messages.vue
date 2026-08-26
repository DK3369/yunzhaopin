<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-msgs', () =>
  api.post('/v1/mcenter/messages', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('common.message') })
</script>

<template>
  <section>
    <h1>{{ $t('common.message') }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <article v-for="row in data?.list || []" :key="row.id" class="job-card">
      <p>{{ row.content || row.title }}</p>
      <p class="muted">{{ row.datetime_n }}</p>
    </article>
  </section>
</template>
