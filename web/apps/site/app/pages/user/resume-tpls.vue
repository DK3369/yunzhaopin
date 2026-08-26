<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('resume-tpls', () => api.post('/v1/mcenter/resume-tpls', {}))
async function apply(id: number) {
  await api.post('/v1/mcenter/resume-tpls/apply', { id })
}
useSeoMeta({ title: t('wap_00328') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00328') }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <article v-for="row in data?.list || data || []" :key="row.id" class="job-card">
      <h3>{{ row.name }}</h3>
      <button type="button" @click="apply(row.id)">{{ $t('common.confirm') }}</button>
    </article>
  </section>
</template>
