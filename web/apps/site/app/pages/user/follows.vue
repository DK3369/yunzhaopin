<script setup lang="ts">
import { formatUnixDate } from '~/utils/site'

type FavDetail = Record<string, unknown>
type FavItem = { kind?: number; target_id?: number; time?: number; detail?: FavDetail }
type FavRow = {
  target_id: number
  target_uid: number
  uid: number
  name?: string
  com_name?: string
  com_pr?: string
  com_mun?: string
  jobnum?: number | string
  job_name?: string
  jobname?: string
  ctime_n?: string
  time_n?: string
}

const api = useApi()
const { t } = useI18n()
const kind = ref(2)
const favKind = computed(() => (kind.value === 1 ? 3 : 2))
const { page, pageSize, inferTotal, go } = useMemberListPage()
watch(kind, () => go(1))
const { data, error, refresh } = await useAsyncData(
  () => `favorites-${favKind.value}-${page.value}`,
  () =>
    api.post<{ list?: FavItem[]; total?: number }>('/v1/mcenter/favorites/list', {
      kind: favKind.value,
      page: page.value,
      page_size: pageSize,
    }),
)
function flatten(row: FavItem): FavRow {
  const d = row.detail && typeof row.detail === 'object' ? row.detail : {}
  const id = Number(row.target_id || d.uid || 0)
  const jobs = Array.isArray(d.open_jobs) ? (d.open_jobs as Array<{ name?: string }>) : []
  return {
    target_id: id,
    target_uid: id,
    uid: id,
    name: String(d.name || d.com_name || d.username || ''),
    com_name: String(d.name || d.com_name || ''),
    com_pr: String(d.pr_n || d.com_pr || ''),
    com_mun: String(d.mun_n || d.com_mun || ''),
    jobnum: (d.job_num as number | undefined) ?? (d.jobnum as number | undefined),
    job_name: String(d.job_name || jobs[0]?.name || ''),
    jobname: String(d.jobname || d.job_name || jobs[0]?.name || ''),
    ctime_n: formatUnixDate(row.time),
    time_n: formatUnixDate(row.time),
  }
}
const rows = computed(() => (data.value?.list || []).map(flatten))
async function toggle(row: FavRow) {
  await api.post('/v1/mcenter/favorites', { kind: favKind.value, target_id: row.target_id })
  refresh()
}
function nameOf(row: FavRow) {
  return row.name || row.com_name || String(row.target_uid || '')
}
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('wap_01142') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_01142')"
    :error="error"
    :empty="!error && !rows.length"
    empty-to="/companies"
    :empty-action="$t('common.search')"
  >
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li :class="{ m_tabactive: kind === 2 }" @click="kind = 2">{{ $t('common.company') }}</li>
          <li :class="{ m_tabactive: kind === 1 }" @click="kind = 1">{{ $t('ui.user_kind') }}</li>
        </ul>
      </div>
    </div>
    <div class="site-pc job_list_tit">
      <ul>
        <li :class="{ job_list_tit_cur: kind === 2 }" @click="kind = 2">
          <a href="javascript:;">{{ $t('common.company') }}</a>
        </li>
        <li :class="{ job_list_tit_cur: kind === 1 }" @click="kind = 1">
          <a href="javascript:;">{{ $t('ui.user_kind') }}</a>
        </li>
      </ul>
    </div>
    <div v-if="rows.length" class="attention_enterprises_tit site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">{{ $t('wap_com_00157') }}</div>
      <div class="attention_enterprises_span attention_enterprises_job">{{ $t('member_user_00047') }}</div>
      <div class="attention_enterprises_span attention_enterprises_time">{{ $t('member_user_00046') }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in rows" :key="row.target_id" class="attention_enterprises_list site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">
        <NuxtLink v-if="kind === 2" :to="`/companies/${row.target_id}`" class="attention_enterprises_name_a">{{ nameOf(row) }}</NuxtLink>
        <span v-else class="attention_enterprises_name_a">{{ nameOf(row) }}</span>
        <div class="mt10">
          {{ row.com_pr }}
          <span v-if="row.com_pr && row.com_mun" class="look_myresume_comline">|</span>
          {{ row.com_mun }}
        </div>
      </div>
      <div class="attention_enterprises_span attention_enterprises_job mt15">
        <template v-if="row.jobnum || row.job_name">
          {{ row.jobname || row.job_name }}
          <a v-if="kind === 2" :href="`/companies/${row.target_id}`" class="attention_enterprises_job_n">{{ $t('common_02057') }}{{ row.jobnum || '' }}{{ $t('wap_user_00151') }}</a>
        </template>
        <template v-else>{{ $t('default_00033') }}</template>
      </div>
      <div class="attention_enterprises_span attention_enterprises_time mt15">{{ row.ctime_n || row.time_n }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">
        <a href="javascript:;" class="cblue" @click="toggle(row)">{{ $t('wap_js_00140') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in rows"
          :key="'h5-' + row.target_id"
          variant="issue"
          :title="nameOf(row)"
          :sub="kind === 2 ? `${row.com_pr || ''} ${row.com_mun || ''}`.trim() : ''"
          :time="row.ctime_n || row.time_n"
          :to="kind === 2 ? `/companies/${row.target_id}` : undefined"
        >
          <div class="Posted_state_hrtip">
            <a href="javascript:;" @click.prevent="toggle(row)">{{ $t('wap_js_00140') }}</a>
          </div>
        </MemberPostedCard>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
