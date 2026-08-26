<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-iv', () =>
  api.post('/v1/mcenter/company/interviews', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_user_00216') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00216')" :error="error" :empty="!error && !(data?.list || []).length">
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <p>{{ row.uid }} · {{ row.status_n }}</p>
      <p class="muted">{{ row.datetime_n }}</p>
    </article>
  </MemberPanel>
</template>
