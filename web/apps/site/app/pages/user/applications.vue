<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const state = ref<number | null>(null)
const days = ref<number | null>(null)
const { page, pageSize, inferTotal, go } = useMemberListPage()
watch([state, days], () => go(1))
const { data, error, refresh } = await useAsyncData(
  () => `my-apps-${state.value ?? 'all'}-${days.value ?? 'all'}-${page.value}`,
  () =>
    api.post('/v1/mcenter/my-applications', {
      page: page.value,
      page_size: pageSize,
      ...(state.value === null ? {} : { state: state.value }),
      ...(days.value === null ? {} : { days: days.value }),
    }),
)
const list = computed(() => data.value?.list || [])
const msg = ref('')
async function withdraw(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-applications/withdraw', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-applications/delete', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function browseLabel(row: { is_browse?: number; invited?: boolean }) {
  if (row.invited) return t('wap_user_00216')
  if (row.is_browse === 1) return t('wap_user_00260')
  if (row.is_browse === 2) return t('wap_user_00258')
  if (row.is_browse === 3) return t('wap_user_00266')
  if (row.is_browse === 4) return t('wap_user_00354')
  if (row.is_browse === 5) return t('member_com_00108')
  if (row.is_browse === 7) return t('wap_user_00356')
  return String(row.is_browse ?? '')
}
const stateTabs = computed(() => [
  { v: null, label: t('common.all') },
  { v: 1, label: t('wap_user_00260') },
  { v: 3, label: t('wap_user_00266') },
  { v: 4, label: t('wap_user_00354') },
  { v: 7, label: t('wap_user_00356') },
])
const dayTabs = computed(() => [
  { v: null as number | null, label: t('common.all') },
  { v: 1, label: t('common_01940') },
  { v: 3, label: t('admin_user_00179') },
  { v: 7, label: t('wap_00339') },
  { v: 15, label: '15' },
  { v: 30, label: t('member_com_00368') },
])
const filterGroups = computed(() => [
  {
    label: t('member_user_00175'),
    options: stateTabs.value.map((tab) => ({
      value: tab.v,
      label: tab.label,
      on: state.value === tab.v,
      select: () => {
        state.value = tab.v
      },
    })),
  },
  {
    label: t('member_user_00176'),
    options: dayTabs.value.map((tab) => ({
      value: tab.v,
      label: tab.label,
      on: days.value === tab.v,
      select: () => {
        days.value = tab.v
      },
    })),
  },
])
const sub = computed(() => {
  const n = Number(data.value?.total ?? list.value.length)
  return n ? String(n) : ''
})
const total = computed(() => inferTotal(data.value, list.value))
useSeoMeta({ title: t('wap_user_00270') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_user_00270')"
    :sub="sub"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !list.length"
    :empty-text="$t('ui.no_applies')"
    empty-to="/jobs"
    :empty-action="$t('wap_user_00254')"
  >
    <template #pcFilters>
      <MemberPcFilterBar :groups="filterGroups" />
    </template>
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li v-for="tab in stateTabs" :key="String(tab.v)" :class="{ m_tabactive: state === tab.v }" @click="state = tab.v">
            {{ tab.label }}
          </li>
        </ul>
      </div>
    </div>
    <div v-if="list.length" class="user_new_listtit site-pc">
      <div class="user_new_job" style="width: 260px">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('member_user_00106') }}</div>
      <div class="user_new_tdzt">{{ $t('member_user_00104') }}</div>
      <div class="user_new_yqh">{{ $t('member_user_00107') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in list" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job" style="width: 260px">
        <NuxtLink :to="`/jobs/${row.job_id}`" class="user_new_jobname">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <div v-if="row.job_salary" class="user_new_jobxz">{{ row.job_salary }}</div>
        <div class="user_new_comname">
          <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`">{{ row.com_name }}</NuxtLink>
        </div>
      </div>
      <div class="user_new_time">
        <span class="msg_zt_s">{{ row.datetime_n }}</span>
      </div>
      <MemberApplySteps :is-browse="row.is_browse" :invited="row.invited" :withdrawn="!!row.body" />
      <div class="user_new_yqh">
        <span>{{ row.status === 0 ? $t('wap_com_00243') : $t('wap_com_00242') }}</span>
      </div>
      <div class="user_new_cz">
        <a v-if="row.is_browse === 1 && !row.body" href="javascript:;" class="user_new_yqh_ch" @click="withdraw(row.id)">{{ $t('common.cancel') }}</a>
        <a v-else href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in list"
          :key="'h5-' + row.id"
          :title="row.job_name || $t('common.job')"
          :pay="row.job_salary"
          :sub="row.com_name"
          :time="row.datetime_n"
          :to="`/jobs/${row.job_id}`"
        >
          <MemberApplyH5State :is-browse="row.is_browse" :invited="row.invited" :withdrawn="!!row.body" />
        </MemberPostedCard>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg" class="muted">{{ msg }}</p>
  </MemberPanel>
</template>
