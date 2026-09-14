<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const kind = ref(1)
const { data, error, refresh } = await useAsyncData(
  () => `fav-${kind.value}`,
  () => api.post('/v1/mcenter/favorites/list', { kind: kind.value, page: 1, page_size: 20 }),
)
watch(kind, () => refresh())
async function remove(targetId: number) {
  await api.post('/v1/mcenter/favorites/remove', { kind: kind.value, target_id: targetId })
  refresh()
}
function titleOf(row: { target_id: number; detail?: Record<string, string> }) {
  const d = row.detail || {}
  return d.name || d.job_name || d.com_name || d.display_name || String(row.target_id)
}
function salaryOf(row: { detail?: Record<string, string> }) {
  const d = row.detail || {}
  return d.salary || d.job_salary || ''
}
function toOf(row: { target_id: number }) {
  if (kind.value === 1) return `/jobs/${row.target_id}`
  if (kind.value === 2) return `/companies/${row.target_id}`
  return ''
}
useSeoMeta({ title: t('member_user_00103') })
</script>

<template>
  <MemberPanel
    :title="$t('member_user_00103')"
    :sub="$t('member_user_00102')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !(data?.list || []).length"
    :empty-text="$t('ui.no_fav')"
    empty-to="/jobs"
    :empty-action="$t('wap_user_00254')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li :class="{ m_tabactive: kind === 1 }" @click="kind = 1">{{ $t('common.job') }}</li>
          <li :class="{ m_tabactive: kind === 2 }" @click="kind = 2">{{ $t('common.company') }}</li>
          <li :class="{ m_tabactive: kind === 3 }" @click="kind = 3">{{ $t('ui.user_kind') }}</li>
        </ul>
      </div>
    </div>
    <div class="site-pc job_list_tit">
      <ul>
        <li :class="{ job_list_tit_cur: kind === 1 }" @click="kind = 1">
          <a href="javascript:;">{{ $t('common.job') }}</a>
        </li>
        <li :class="{ job_list_tit_cur: kind === 2 }" @click="kind = 2">
          <a href="javascript:;">{{ $t('common.company') }}</a>
        </li>
        <li :class="{ job_list_tit_cur: kind === 3 }" @click="kind = 3">
          <a href="javascript:;">{{ $t('ui.user_kind') }}</a>
        </li>
      </ul>
    </div>
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('member_user_00106') }}</div>
      <div class="user_new_zt">{{ $t('member_user_00104') }}</div>
      <div class="user_new_yqh">{{ $t('member_user_00107') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.target_id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="toOf(row)" :to="toOf(row)" class="user_new_jobname">{{ titleOf(row) }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ titleOf(row) }}</span>
        <div v-if="row.detail?.com_name && kind === 1" class="user_new_comname">{{ row.detail.com_name }}</div>
      </div>
      <div class="user_new_time">
        <span class="user_new_xz_n">{{ salaryOf(row) }}</span>
      </div>
      <div class="user_new_zt">{{ row.datetime_n }}</div>
      <div class="user_new_yqh">{{ row.detail?.statename || row.detail?.status_n }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.target_id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.target_id"
          :title="titleOf(row)"
          :pay="salaryOf(row)"
          :sub="row.detail?.com_name || row.datetime_n"
          :time="row.datetime_n"
          :to="toOf(row) || undefined"
        />
      </div>
    </div>
  </MemberPanel>
</template>
