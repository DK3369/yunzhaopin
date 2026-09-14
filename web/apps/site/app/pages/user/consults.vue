<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('job-msg-mine', () =>
  api.post('/v1/mcenter/job-messages/mine', { page: 1, page_size: 20 }),
)
const msg = ref('')
function statusLabel(status?: number) {
  if (status === 2) return t('wap_user_00167')
  if (status === 1) return t('wap_user_00165')
  return t('wap_user_00166')
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/job-messages/hide', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_user_00115') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00115')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="jobnotice_list">
        <h3>
          <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`">{{ row.job_name || $t('common.job') }}</NuxtLink>
          <span v-else>{{ row.job_name || $t('common_02082') }}</span>
        </h3>
        <p>
          <NuxtLink v-if="row.job_uid" :to="`/companies/${row.job_uid}`">{{ row.com_name }}</NuxtLink>
        </p>
        <p class="muted">{{ statusLabel(row.status) }} · {{ row.datetime_n }}</p>
        <p>{{ row.content }}</p>
        <p v-if="row.reply" class="muted">{{ row.reply }}</p>
        <p v-else class="muted">{{ $t('member_user_00481') }}</p>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
