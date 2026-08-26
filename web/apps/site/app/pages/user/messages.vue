<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('msgs', () =>
  api.post('/v1/mcenter/messages', { page: 1, page_size: 20 }),
)
async function read(id: number) {
  await api.post('/v1/mcenter/messages/read', { id })
  refresh()
}
useSeoMeta({ title: t('common.message') })
</script>

<template>
  <section>
    <h1>{{ $t('common.message') }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
    <article v-for="row in data?.list || []" :key="row.id" class="job-card">
      <p>{{ row.content || row.title }}</p>
      <p class="muted">{{ row.datetime_n }}</p>
      <button type="button" @click="read(row.id)">{{ $t('common.confirm') }}</button>
    </article>
  </section>
</template>
