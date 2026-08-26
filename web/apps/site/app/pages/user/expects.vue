<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('expects', () => api.post('/v1/mcenter/resume/expects/list', {}))
useSeoMeta({ title: t('home.intention') })
</script>

<template>
  <MemberPanel :title="$t('home.intention')" :error="error" :empty="!error && !(data?.list || data || []).length">
    <article v-for="row in data?.list || data || []" :key="row.id" class="look_resume_list">
      <h3>{{ row.name }}</h3>
      <p class="muted">{{ row.job_classid_n }} · {{ row.city_classid_n }}</p>
    </article>
  </MemberPanel>
</template>
