<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: applies, error, refresh: refreshApplies } = await useAsyncData('my-part-applies', () =>
  api.post('/v1/mcenter/my-part-applications/list', { page: 1, page_size: 20 }),
)
const { data: collects, refresh: refreshCollects } = await useAsyncData('my-part-collects', () =>
  api.post('/v1/mcenter/my-part-collects/list', { page: 1, page_size: 20 }),
)
const msg = ref('')
function statusLabel(status?: number) {
  if (status === 1) return t('wap_user_00260')
  if (status === 2) return t('wap_com_00427')
  if (status === 3) return t('wap_com_00046')
  return String(status ?? '')
}
async function delApply(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-part-applications', { ids: [id] })
    await refreshApplies()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function delCollect(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-part-collects', { ids: [id] })
    await refreshCollects()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00303') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00303')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="site-pc job_list_tit">
      <ul>
        <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('ui.apply') }}</a></li>
      </ul>
    </div>
    <div v-if="(applies?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('member_user_00104') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in applies?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.job_id" :to="`/parts/${row.job_id}`" class="user_new_jobname">{{ row.job_name || row.job_id }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.job_name || row.job_id }}</span>
        <div class="user_new_comname">{{ row.com_name }}</div>
      </div>
      <div class="user_new_time">{{ statusLabel(row.status) }} · {{ row.ctime_n }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="delApply(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in applies?.list || []"
          :key="'ha-' + row.id"
          :title="row.job_name || String(row.job_id)"
          :pay="statusLabel(row.status)"
          :sub="row.com_name"
          :time="row.ctime_n"
          :to="row.job_id ? `/parts/${row.job_id}` : undefined"
        />
      </div>
    </div>
    <MemberResumeH1 :title="$t('member_user_00103')" />
    <div v-for="row in collects?.list || []" :key="'c-' + row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.job_id" :to="`/parts/${row.job_id}`" class="user_new_jobname">{{ row.job_name || row.job_id }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.job_name || row.job_id }}</span>
      </div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="delCollect(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in collects?.list || []"
          :key="'hc-' + row.id"
          :title="row.job_name || String(row.job_id)"
          :sub="row.com_name"
          :time="row.ctime_n"
          :to="row.job_id ? `/parts/${row.job_id}` : undefined"
        />
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
