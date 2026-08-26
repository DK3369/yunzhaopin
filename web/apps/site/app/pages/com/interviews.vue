<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-iv', () =>
  api.post('/v1/mcenter/company/interviews', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_user_00216') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00216') }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <article v-for="row in data?.list || []" :key="row.id" class="job-card">
      <p>{{ row.uid }} · {{ row.status_n }}</p>
      <p class="muted">{{ row.datetime_n }}</p>
    </article>
  </section>
</template>
