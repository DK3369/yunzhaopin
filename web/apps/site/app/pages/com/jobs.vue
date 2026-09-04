<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-jobs', () =>
  api.post('/v1/mcenter/jobs/list', { page: 1, page_size: 20 }),
)
const { data: counts } = await useAsyncData('com-job-counts', () =>
  api.post<{ total: number; online: number; breakjob_num?: number }>('/v1/mcenter/jobs/counts', {}).catch(() => null),
)
const list = computed(() => (data.value?.list || []) as Array<{ id: number; name?: string; state?: number; status?: number }>)
const msg = ref('')
function jobPhase(job: { state?: number; status?: number }) {
  if (Number(job.status) === 1) return t('wap_com_00242')
  if (Number(job.state) === 0) return t('wap_user_00006')
  if (Number(job.state) === 3) return t('wap_user_00167')
  if (Number(job.state) === 1) return t('wap_com_00243')
  return t('member_user_00181')
}
async function refreshJob(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/jobs/refresh', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
async function setStatus(id: number, status: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/jobs/status', { id, status })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
useSeoMeta({ title: t('wap_com_00106') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00106')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p><NuxtLink to="/com/jobs/new">{{ $t('wap_00322') }}</NuxtLink></p>
    <p v-if="counts" class="muted">{{ $t('wap_com_00029') }} {{ counts.breakjob_num ?? 0 }}</p>
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <article v-for="job in list" :key="job.id" class="look_resume_list">
      <h3>{{ job.name }}</h3>
      <p class="muted">{{ $t('member_user_00181') }} {{ jobPhase(job) }}</p>
      <p>
        <NuxtLink :to="`/com/jobs/new?id=${job.id}`">{{ $t('common.edit') }}</NuxtLink>
        <button type="button" @click="refreshJob(job.id)">{{ $t('wap_com_00029') }}</button>
        <button type="button" @click="setStatus(job.id, 0)">{{ $t('wap_com_00244') }}</button>
        <button type="button" @click="setStatus(job.id, 1)">{{ $t('wap_com_00245') }}</button>
      </p>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
