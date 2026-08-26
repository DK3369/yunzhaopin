<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-msgs', () =>
  api.post('/v1/mcenter/messages', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('common.message') })
</script>

<template>
  <MemberPanel :title="$t('common.message')" :error="error" :empty="!error && !(data?.list || []).length">
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <p>{{ row.content || row.title }}</p>
      <p class="muted">{{ row.datetime_n }}</p>
    </article>
  </MemberPanel>
</template>
