<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('look-jobs-mine', () =>
  api.post('/v1/mcenter/look-jobs/mine', { page: 1, page_size: 20 }),
)
const msg = ref('')
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/look-jobs/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function salaryOf(row: { minsalary?: number; maxsalary?: number }) {
  if (row.minsalary || row.maxsalary) return `${row.minsalary || ''} - ${row.maxsalary || ''}`
  return ''
}
useSeoMeta({ title: t('wap_user_00275') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_user_00275')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !(data?.list || []).length"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li @click="navigateTo('/user/views')">{{ $t('wap_com_00407') }}</li>
          <li class="m_tabactive">{{ $t('wap_user_00275') }}</li>
        </ul>
      </div>
    </div>
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('member_user_00104') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="user_new_jobname">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.job_name || row.id }}</span>
        <div v-if="salaryOf(row)" class="user_new_jobxz">{{ salaryOf(row) }}</div>
        <div class="user_new_comname">{{ row.com_name }}</div>
      </div>
      <div class="user_new_time">{{ row.datetime_n }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :title="row.job_name || $t('common.job')"
          :pay="salaryOf(row)"
          :sub="row.com_name"
          :time="row.datetime_n"
          :to="row.job_id ? `/jobs/${row.job_id}` : undefined"
        />
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
