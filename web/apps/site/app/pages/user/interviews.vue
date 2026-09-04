<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('yqms', () =>
  api.post('/v1/mcenter/yqms/list', { page: 1, page_size: 20 }),
)
const openId = ref(0)
const msg = ref('')
async function accept(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/accept', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function reject(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/reject', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function browseLabel(state?: number) {
  if (state === 3) return t('common.yes')
  if (state === 4) return t('common.no')
  return t('wap_00129')
}
useSeoMeta({ title: t('wap_user_00216') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00216')" :error="error" :empty="!error && !(data?.list || []).length">
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <h3>
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else>{{ row.job_name || row.title || row.id }}</span>
      </h3>
      <p>
        <NuxtLink v-if="row.fid" :to="`/companies/${row.fid}`">{{ row.fname }}</NuxtLink>
      </p>
      <p class="muted">{{ row.intertime }} · {{ row.address }} · {{ row.datetime_n }}</p>
      <p class="muted">{{ browseLabel(row.is_browse) }}</p>
      <p v-if="openId === row.id" class="muted">{{ row.content }} · {{ row.linkman }} {{ row.linktel }}</p>
      <button v-if="row.is_browse !== 3 && row.is_browse !== 4" type="button" @click="accept(row.id)">{{ $t('common.yes') }}</button>
      <button v-if="row.is_browse !== 3 && row.is_browse !== 4" type="button" @click="reject(row.id)">{{ $t('common.no') }}</button>
      <button type="button" @click="openId = openId === row.id ? 0 : row.id">{{ $t('common.more') }}</button>
      <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
