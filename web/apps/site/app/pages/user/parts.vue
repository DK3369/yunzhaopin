<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const tab = ref<'apply' | 'collect'>('apply')
const { page, pageSize, inferTotal, go } = useMemberListPage()
watch(tab, () => go(1))
const { data: applies, error, refresh: refreshApplies } = await useAsyncData(
  () => `my-part-applies-${page.value}`,
  () => api.post('/v1/mcenter/my-part-applications/list', { page: page.value, page_size: pageSize }),
)
const { data: collects, refresh: refreshCollects } = await useAsyncData(
  () => `my-part-collects-${page.value}`,
  () => api.post('/v1/mcenter/my-part-collects/list', { page: page.value, page_size: pageSize }),
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
const list = computed(() => (tab.value === 'apply' ? applies.value?.list : collects.value?.list) || [])
const total = computed(() => inferTotal(tab.value === 'apply' ? applies.value : collects.value, list.value))
useSeoMeta({ title: t('wap_user_00303') })
</script>

<template>
  <MemberPanel
    :title="$t('member_user_00185')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !list.length"
    empty-to="/parts"
    :empty-action="$t('common.search')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="job_list_tit">
      <ul>
        <li :class="{ job_list_tit_cur: tab === 'apply' }">
          <a href="javascript:;" @click="tab = 'apply'">{{ $t('wap_user_00302') }}</a>
        </li>
        <li :class="{ job_list_tit_cur: tab === 'collect' }">
          <a href="javascript:;" @click="tab = 'collect'">{{ $t('wap_user_00303') }}</a>
        </li>
      </ul>
    </div>
    <div v-if="list.length" class="part_time_job_tit site-pc">
      <div class="part_time_job_span part_time_jobname">{{ $t('wap_com_00326') }}</div>
      <div class="part_time_job_span part_time_jobname">{{ $t('wap_com_00157') }}</div>
      <div class="part_time_job_span part_time_jobzt">{{ $t('member_user_00530') }}</div>
      <div class="part_time_job_span part_time_jobtime">{{ $t('wap_com_00342') }}</div>
      <div class="part_time_job_span part_time_jobcz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in list" :key="row.id" class="part_time_job_list site-pc">
      <div class="part_time_job_span part_time_jobname">
        <NuxtLink v-if="row.job_id" :to="`/parts/${row.job_id}`" class="part_time_job_a">{{ row.job_name || row.job_id }}</NuxtLink>
        <span v-else>{{ row.job_name || row.job_id }}</span>
      </div>
      <div class="part_time_job_span part_time_jobname">{{ row.com_name }}</div>
      <div class="part_time_job_span part_time_jobzt">{{ tab === 'apply' ? statusLabel(row.status) : '' }}</div>
      <div class="part_time_job_span part_time_jobtime">{{ row.ctime_n }}</div>
      <div class="part_time_job_span part_time_jobcz">
        <a href="javascript:;" class="List_dete cblue" @click="tab === 'apply' ? delApply(row.id) : delCollect(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in list"
          :key="'h5-' + row.id"
          :title="row.job_name || String(row.job_id)"
          :pay="tab === 'apply' ? statusLabel(row.status) : undefined"
          :sub="row.com_name"
          :time="row.ctime_n"
          :to="row.job_id ? `/parts/${row.job_id}` : undefined"
        >
          <div class="Posted_state_hrtip">
            <a href="javascript:;" @click.prevent="tab === 'apply' ? delApply(row.id) : delCollect(row.id)">{{ $t('common.delete') }}</a>
          </div>
        </MemberPostedCard>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
