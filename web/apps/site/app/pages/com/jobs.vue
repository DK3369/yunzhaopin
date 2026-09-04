<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-jobs', () =>
  api.post('/v1/mcenter/jobs/list', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])
const msg = ref('')
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
useSeoMeta({ title: t('ui.job_mgmt') })
</script>

<template>
  <MemberPanel :title="$t('ui.job_mgmt')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p><NuxtLink to="/com/jobs/new">{{ $t('ui.publish_job') }}</NuxtLink></p>
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('ui.please_login_com') }}</p>
    <article v-for="job in list" :key="job.id" class="look_resume_list">
      <h3>{{ job.name }}</h3>
      <p class="muted">{{ $t('ui.status') }} {{ job.state }}</p>
      <p>
        <NuxtLink :to="`/com/jobs/new?id=${job.id}`">{{ $t('common.edit') }}</NuxtLink>
        <button type="button" @click="refreshJob(job.id)">{{ $t('wap_com_00029') }}</button>
        <button type="button" @click="setStatus(job.id, 0)">{{ $t('ui.open') }}</button>
        <button type="button" @click="setStatus(job.id, 2)">{{ $t('common.close') }}</button>
      </p>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
