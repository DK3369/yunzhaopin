<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('downloads', () =>
  api.post('/v1/mcenter/resume-downloads/outbox', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_com_00235') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00235')" :error="error" :empty="!error && !(data?.list || []).length">
    <article v-for="row in data?.list || []" :key="row.id || row.uid" class="look_resume_list">
      <NuxtLink :to="`/resumes/${row.uid}`">{{ row.name || row.display_name || row.uid }}</NuxtLink>
      <p class="muted">{{ row.datetime_n }}</p>
    </article>
  </MemberPanel>
</template>
