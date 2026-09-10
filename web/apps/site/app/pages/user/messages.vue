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
async function remove(id: number) {
  await api.post('/v1/mcenter/messages/delete', { id })
  refresh()
}
async function readAll() {
  await api.post('/v1/mcenter/messages/read-all', {})
  refresh()
}
useSeoMeta({ title: t('common.message') })
</script>

<template>
  <MemberPanel :title="$t('common.message')" :error="error" :empty="!error && !(data?.list || []).length">
    <nav class="stack" style="margin-bottom: 12px">
      <NuxtLink to="/user/interviews" class="job-card">{{ $t('wap_user_00216') }}</NuxtLink>
      <NuxtLink to="/user/applications" class="job-card">{{ $t('wap_01133') }}</NuxtLink>
      <NuxtLink to="/user/consults" class="job-card">{{ $t('wap_user_00364') }}</NuxtLink>
    </nav>
    <p><button type="button" @click="readAll">{{ $t('common.confirm') }}</button></p>
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <p>{{ row.body || row.content || row.title || row.id }}</p>
      <p class="muted">{{ row.datetime_n }}</p>
    <p>
      <button type="button" @click="read(row.id)">{{ $t('common.confirm') }}</button>
      <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
    </p>
    </article>
  </MemberPanel>
</template>
