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
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('member_user_00104') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="user_new_jobname">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.job_name || $t('common_02082') }}</span>
        <div class="user_new_comname">
          <NuxtLink v-if="row.job_uid" :to="`/companies/${row.job_uid}`">{{ row.com_name }}</NuxtLink>
        </div>
      </div>
      <div class="user_new_time">{{ statusLabel(row.status) }} · {{ row.datetime_n }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
      <p class="muted">{{ row.content }}</p>
      <p v-if="row.reply" class="muted">{{ row.reply }}</p>
      <p v-else class="muted">{{ $t('member_user_00481') }}</p>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :title="row.job_name || $t('common.job')"
          :pay="statusLabel(row.status)"
          :sub="row.com_name"
          :time="row.datetime_n"
          :to="row.job_id ? `/jobs/${row.job_id}` : undefined"
        >
          <p class="muted">{{ row.content }}</p>
        </MemberPostedCard>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
