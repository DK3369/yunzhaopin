<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-msgs', () =>
  api.post('/v1/mcenter/messages', { page: 1, page_size: 20 }),
)
const { data: dash } = await useAsyncData('com-msg-dash', () =>
  api
    .post<{
      applies_unread?: number
      job_msg_unanswered?: number
      unread_messages?: number
    }>('/v1/mcenter/com-dashboard', {})
    .catch(() => null),
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
  <MemberPanel :title="$t('common.message')" :error="error" :empty="false">
    <nav class="stack" style="margin-bottom: 12px">
      <NuxtLink to="/com/applications" class="job-card">
        {{ $t('wap_00794') }}
        <span v-if="dash?.applies_unread" class="yun_m_n">{{ dash.applies_unread }}</span>
      </NuxtLink>
      <NuxtLink to="/com/job-messages" class="job-card">
        {{ $t('wap_com_00408') }}
        <span v-if="dash?.job_msg_unanswered" class="yun_m_n">{{ dash.job_msg_unanswered }}</span>
      </NuxtLink>
    </nav>
    <h2>{{ $t('wap_user_00363') }}</h2>
    <p><button type="button" @click="readAll">{{ $t('common.confirm') }}</button></p>
    <p v-if="!error && !(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <p>{{ row.body || row.content || row.title || row.id }}</p>
      <p class="muted">{{ row.datetime_n }}</p>
      <button type="button" @click="read(row.id)">{{ $t('common.confirm') }}</button>
      <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
    </article>
  </MemberPanel>
</template>
