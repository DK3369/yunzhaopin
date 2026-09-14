<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const state = ref<number | null>(null)
const days = ref<number | null>(null)
const { data, error, refresh } = await useAsyncData(
  () => `my-apps-${state.value ?? 'all'}-${days.value ?? 'all'}`,
  () =>
    api.post('/v1/mcenter/my-applications', {
      page: 1,
      page_size: 20,
      ...(state.value === null ? {} : { state: state.value }),
      ...(days.value === null ? {} : { days: days.value }),
    }),
)
watch([state, days], () => refresh())
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
useSeoMeta({ title: t('wap_user_00270') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_user_00270')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !list.length"
    :empty-text="$t('ui.no_applies')"
    empty-to="/jobs"
    :empty-action="$t('wap_user_00254')"
  >
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
    <p class="site-pc">
      <button v-for="tab in stateTabs" :key="'pc-' + String(tab.v)" type="button" :class="{ on: state === tab.v }" @click="state = tab.v">
        {{ tab.label }}
      </button>
    </p>
    <p>
      <button type="button" @click="days = null">{{ $t('common.all') }}</button>
      <button type="button" @click="days = 1">1</button>
      <button type="button" @click="days = 3">3</button>
      <button type="button" @click="days = 7">7</button>
      <button type="button" @click="days = 15">15</button>
      <button type="button" @click="days = 30">30</button>
    </p>
    <div v-if="list.length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('member_user_00106') }}</div>
      <div class="user_new_zt">{{ $t('member_user_00104') }}</div>
      <div class="user_new_yqh">{{ $t('member_user_00107') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in list" :key="row.id" class="jobnotice_list">
      <div class="user_new_job">
        <NuxtLink :to="`/jobs/${row.job_id}`" class="user_new_jobname">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <div class="user_new_comname">
          <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`">{{ row.com_name }}</NuxtLink>
        </div>
      </div>
      <div class="user_new_time">{{ row.datetime_n }}</div>
      <div class="user_new_zt">{{ browseLabel(row) }}</div>
      <div class="user_new_yqh">
        <a v-if="row.apply_url" :href="row.apply_url" target="_blank" rel="noopener">{{ $t('ui.apply_official') }}</a>
      </div>
      <div class="user_new_cz">
        <a v-if="!row.quxiao" href="javascript:;" class="user_new_yqh_sc" @click="withdraw(row.id)">{{ $t('common.cancel') }}</a>
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
