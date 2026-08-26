<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-parts', () =>
  api.post('/v1/mcenter/com-parts/list', { page: 1, page_size: 20 }),
)
const { data: applies } = await useAsyncData('com-part-applies', () =>
  api.post('/v1/mcenter/com-part-applications', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('ui.com_parts') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.com_parts') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.please_login_com') }}</p>
    <h2>{{ $t('ui.published') }}</h2>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_parts') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.name || row.id }}</h3>
      </article>
    </div>
    <h2>{{ $t('ui.recv_applies') }}</h2>
    <p v-if="!(applies?.list || []).length" class="muted">{{ $t('ui.no_apply') }}</p>
    <div class="stack">
      <article v-for="row in applies?.list || []" :key="row.id" class="job-card">
        <h3>uid {{ row.uid }} · job_id {{ row.job_id }}</h3>
        <p class="muted">status {{ row.status }}</p>
      </article>
    </div>
  </section>
</template>
