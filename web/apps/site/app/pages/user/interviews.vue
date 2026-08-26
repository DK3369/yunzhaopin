<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('interviews', () =>
  api.post('/v1/mcenter/interviews', { page: 1, page_size: 20 }),
)
async function accept(id: number) {
  await api.post('/v1/mcenter/interviews/accept', { id })
  refresh()
}
async function reject(id: number) {
  await api.post('/v1/mcenter/interviews/reject', { id })
  refresh()
}
useSeoMeta({ title: t('wap_user_00216') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00216')" :error="error" :empty="!error && !(data?.list || []).length">
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <p>{{ row.content || row.job_name || row.id }}</p>
      <p class="muted">{{ row.datetime_n || row.status_n }}</p>
      <button type="button" @click="accept(row.id)">{{ $t('common.yes') }}</button>
      <button type="button" @click="reject(row.id)">{{ $t('common.no') }}</button>
    </article>
  </MemberPanel>
</template>
