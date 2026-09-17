<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Metric = { num?: number; jzr?: number }
type PackItem = { key?: string; title: string; tc_num: string; num: string; unit: string; width: string }
type Kv = { name: string; value: number }
type ChartSeries = {
  dates: string[]
  wkg: number[]
  kgw: number[]
  wdl: number[]
  xzjl: number[]
  tdjl: number[]
  yqms: number[]
}
type TalentDims = { exp: Kv[]; edu: Kv[]; salary: Kv[] }
type TalentPack = { wkg: TalentDims; kgw: TalentDims; xzjl: TalentDims; tdjl: TalentDims; yqms: TalentDims }
type WeekCard = { title: string; num: number }
type WeekPack = { dates: string; look_data: WeekCard[]; resume_data: WeekCard[]; ms_data: WeekCard[] }

const api = useApi()
const { t } = useI18n()
const tab = ref<'day' | 'range'>('day')
const trendType = ref(1)
const trendDays = ref(7)
const rangeKind = ref(1)
const dayFrom = ref('')
const dayTo = ref('')
const monthVal = ref('')
const yearVal = ref('')
const weekTimes = ref(1)

function ymd(d: Date) {
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  return `${y}-${m}-${day}`
}
const now = new Date()
dayTo.value = ymd(now)
const from = new Date(now)
from.setDate(from.getDate() - 29)
dayFrom.value = ymd(from)
monthVal.value = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`
yearVal.value = String(now.getFullYear())

const rangeTimes = computed(() => {
  if (rangeKind.value === 2) return monthVal.value
  if (rangeKind.value === 3) return yearVal.value
  return [dayFrom.value, dayTo.value]
})

const { data, error } = await useAsyncData('com-dashboard-full', () =>
  api.post('/v1/mcenter/com-dashboard/full', {}),
)
const today = computed(
  () =>
    (data.value as {
      today?: {
        look_resume?: Metric
        look_job?: Metric
        down_resume?: Metric
        apply?: Metric
        invite?: Metric
      }
    } | null)?.today || null,
)

const { data: pack } = await useAsyncData('com-stats-package', () =>
  api.post<PackItem[]>('/v1/mcenter/com-stats/package', {}).catch(() => [] as PackItem[]),
)
const packList = computed(() => (Array.isArray(pack.value) ? pack.value : []))

const { data: trend } = await useAsyncData(
  () => `com-stats-trend-${trendType.value}-${trendDays.value}`,
  () =>
    api
      .post<{ name?: string; data?: Record<string, number> }>('/v1/mcenter/com-stats/trend', {
        type: trendType.value,
        days: trendDays.value,
      })
      .catch(() => ({ name: '', data: {} as Record<string, number> })),
)

const { data: range } = await useAsyncData(
  () => `com-stats-range-${rangeKind.value}-${JSON.stringify(rangeTimes.value)}`,
  () =>
    api
      .post('/v1/mcenter/com-stats/range', { type: rangeKind.value, times: rangeTimes.value })
      .catch(() => null),
)
const { data: chart } = await useAsyncData(
  () => `com-stats-chart-${rangeKind.value}-${JSON.stringify(rangeTimes.value)}`,
  () =>
    api
      .post<ChartSeries>('/v1/mcenter/com-stats/chart', { type: rangeKind.value, times: rangeTimes.value })
      .catch(() => null),
)
const { data: talent } = await useAsyncData(
  () => `com-stats-talent-${rangeKind.value}-${JSON.stringify(rangeTimes.value)}`,
  () =>
    api
      .post<TalentPack>('/v1/mcenter/com-stats/talent', { type: rangeKind.value, times: rangeTimes.value })
      .catch(() => null),
)
const { data: details } = await useAsyncData(
  () => `com-stats-details-${rangeKind.value}-${JSON.stringify(rangeTimes.value)}`,
  () =>
    api
      .post<ChartSeries>('/v1/mcenter/com-stats/details', { type: rangeKind.value, times: rangeTimes.value })
      .catch(() => null),
)
const { data: week } = await useAsyncData(
  () => `com-stats-week-${weekTimes.value}`,
  () =>
    api.post<WeekPack>('/v1/mcenter/com-stats/week', { times: weekTimes.value }).catch(() => null),
)

function jzrText(n?: number) {
  const v = Number(n || 0)
  const sign = v > 0 ? '+' : ''
  return `${t('member_com_00373')} ${sign}${v}`
}

const trendOpt = computed(() => {
  const dataMap = trend.value?.data || {}
  const labels = Object.keys(dataMap)
  return {
    tooltip: { trigger: 'axis' },
    grid: { left: 40, right: 16, top: 24, bottom: 32 },
    xAxis: { type: 'category', data: labels },
    yAxis: { type: 'value', minInterval: 1 },
    series: [{ type: 'line', smooth: true, data: labels.map((k) => Number(dataMap[k] || 0)) }],
  }
})

function barOpt(dates: string[], series: Array<{ name: string; data: number[] }>) {
  return {
    tooltip: { trigger: 'axis' },
    legend: { data: series.map((s) => s.name) },
    grid: { left: 40, right: 16, top: 40, bottom: 32 },
    xAxis: { type: 'category', data: dates },
    yAxis: { type: 'value', minInterval: 1 },
    series: series.map((s) => ({ type: 'bar', name: s.name, data: s.data })),
  }
}

function pieOpt(rows: Kv[]) {
  return {
    tooltip: { trigger: 'item' },
    series: [
      {
        type: 'pie',
        radius: '62%',
        data: (rows || []).map((r) => ({ name: r.name, value: Number(r.value || 0) })),
      },
    ],
  }
}

const viewBar = computed(() =>
  barOpt(chart.value?.dates || [], [
    { name: t('member_com_00371'), data: chart.value?.wkg || [] },
    { name: t('member_com_00372'), data: chart.value?.kgw || [] },
    { name: t('member_com_00370'), data: chart.value?.wdl || [] },
  ]),
)
const resumeBar = computed(() =>
  barOpt(chart.value?.dates || [], [
    { name: t('wap_00451'), data: chart.value?.xzjl || [] },
    { name: t('wap_com_00235'), data: chart.value?.tdjl || [] },
  ]),
)
const msBar = computed(() =>
  barOpt(chart.value?.dates || [], [{ name: t('wap_user_00216'), data: chart.value?.yqms || [] }]),
)

const talentPies = computed(() => {
  const p = talent.value
  if (!p) return []
  return [
    { title: t('member_com_00371'), dims: p.wkg },
    { title: t('member_com_00372'), dims: p.kgw },
    { title: t('wap_com_00235'), dims: p.tdjl },
  ]
})

const detailRows = computed(() => {
  const d = details.value
  if (!d?.dates?.length) return []
  return d.dates.map((label, i) => ({
    label,
    wkg: d.wkg?.[i] || 0,
    kgw: d.kgw?.[i] || 0,
    wdl: d.wdl?.[i] || 0,
    xzjl: d.xzjl?.[i] || 0,
    tdjl: d.tdjl?.[i] || 0,
    yqms: d.yqms?.[i] || 0,
  }))
})

const trendKinds = computed(() => [
  { v: 1, label: t('member_com_00371') },
  { v: 2, label: t('member_com_00372') },
  { v: 5, label: t('wap_00451') },
  { v: 6, label: t('wap_com_00235') },
  { v: 8, label: t('wap_user_00216') },
])

useSeoMeta({ title: t('admin_tool_00224') })
</script>

<template>
  <MemberPanel :title="$t('admin_tool_00224')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <MemberComScreen
        :tabs="[
          { value: 'day', label: $t('member_com_00375'), on: tab === 'day', select: () => (tab = 'day') },
          { value: 'range', label: $t('member_com_00363'), on: tab === 'range', select: () => (tab = 'range') },
        ]"
      />
      <p>
        <NuxtLink to="/com/tongji">{{ $t('admin_tool_00181') }}</NuxtLink>
      </p>
      <template v-if="tab === 'day'">
        <div v-if="today" class="membSubGuaTwo site-pc">
          <ul>
            <li>
              <div class="twoDivTite"><span>{{ $t('member_com_00371') }}</span></div>
              <div class="twoDivNum"><span>{{ today.look_resume?.num ?? 0 }}</span><b>{{ jzrText(today.look_resume?.jzr) }}</b></div>
            </li>
            <li>
              <div class="twoDivTite"><span>{{ $t('member_com_00372') }}</span></div>
              <div class="twoDivNum"><span>{{ today.look_job?.num ?? 0 }}</span><b>{{ jzrText(today.look_job?.jzr) }}</b></div>
            </li>
            <li>
              <div class="twoDivTite"><span>{{ $t('wap_00451') }}</span></div>
              <div class="twoDivNum"><span>{{ today.down_resume?.num ?? 0 }}</span><b>{{ jzrText(today.down_resume?.jzr) }}</b></div>
            </li>
            <li>
              <div class="twoDivTite"><span>{{ $t('wap_com_00235') }}</span></div>
              <div class="twoDivNum"><span>{{ today.apply?.num ?? 0 }}</span><b>{{ jzrText(today.apply?.jzr) }}</b></div>
            </li>
            <li>
              <div class="twoDivTite"><span>{{ $t('wap_user_00216') }}</span></div>
              <div class="twoDivNum"><span>{{ today.invite?.num ?? 0 }}</span><b>{{ jzrText(today.invite?.jzr) }}</b></div>
            </li>
          </ul>
        </div>
        <div v-if="today" class="site-h5">
          <div class="com_cardlist">
            <div class="com_cardlist_tit">{{ $t('member_com_00371') }}</div>
            <div class="com_cardlist_p">{{ today.look_resume?.num ?? 0 }} {{ jzrText(today.look_resume?.jzr) }}</div>
          </div>
          <div class="com_cardlist">
            <div class="com_cardlist_tit">{{ $t('member_com_00372') }}</div>
            <div class="com_cardlist_p">{{ today.look_job?.num ?? 0 }} {{ jzrText(today.look_job?.jzr) }}</div>
          </div>
          <div class="com_cardlist">
            <div class="com_cardlist_tit">{{ $t('wap_00451') }}</div>
            <div class="com_cardlist_p">{{ today.down_resume?.num ?? 0 }} {{ jzrText(today.down_resume?.jzr) }}</div>
          </div>
          <div class="com_cardlist">
            <div class="com_cardlist_tit">{{ $t('wap_com_00235') }}</div>
            <div class="com_cardlist_p">{{ today.apply?.num ?? 0 }} {{ jzrText(today.apply?.jzr) }}</div>
          </div>
          <div class="com_cardlist">
            <div class="com_cardlist_tit">{{ $t('wap_user_00216') }}</div>
            <div class="com_cardlist_p">{{ today.invite?.num ?? 0 }} {{ jzrText(today.invite?.jzr) }}</div>
          </div>
        </div>
        <div class="job_list_tit">
          <ul>
            <li v-for="k in trendKinds" :key="k.v" :class="{ job_list_tit_cur: trendType === k.v }">
              <a href="javascript:;" @click.prevent="trendType = k.v">{{ k.label }}</a>
            </li>
          </ul>
        </div>
        <p>
          <a href="javascript:;" :class="{ cblue: trendDays === 7 }" @click.prevent="trendDays = 7">{{ $t('member_com_00367') }}</a>
          ·
          <a href="javascript:;" :class="{ cblue: trendDays === 30 }" @click.prevent="trendDays = 30">{{ $t('member_com_00368') }}</a>
        </p>
        <h3>{{ $t('member_com_00369') }}</h3>
        <ClientOnly>
          <ChartBox :option="trendOpt" />
        </ClientOnly>
        <h3>{{ $t('wap_com_00064') }}</h3>
        <div v-for="row in packList" :key="row.key || row.title" class="com_cardlist">
          <div class="com_cardlist_tit">{{ $t(row.title) }} {{ row.num }} / {{ row.tc_num }} {{ row.unit }}</div>
          <div class="com_cardlist_p">
            <div class="pack-bar"><span :style="{ width: `${row.width}%` }" /></div>
          </div>
        </div>
        <div class="site-h5">
          <h3>{{ $t('member_com_00365') }}</h3>
          <p>
            <a href="javascript:;" @click.prevent="weekTimes = 1">1</a> ·
            <a href="javascript:;" @click.prevent="weekTimes = 2">2</a> ·
            <a href="javascript:;" @click.prevent="weekTimes = 3">3</a> ·
            <a href="javascript:;" @click.prevent="weekTimes = 4">4</a>
            <span v-if="week?.dates"> {{ week.dates }}</span>
          </p>
          <div v-for="row in week?.look_data || []" :key="'lk-' + row.title" class="com_cardlist">
            <div class="com_cardlist_tit">{{ $t(row.title) }}</div>
            <div class="com_cardlist_p">{{ row.num }}</div>
          </div>
          <div v-for="row in week?.resume_data || []" :key="'rs-' + row.title" class="com_cardlist">
            <div class="com_cardlist_tit">{{ $t(row.title) }}</div>
            <div class="com_cardlist_p">{{ row.num }}</div>
          </div>
          <div v-for="row in week?.ms_data || []" :key="'ms-' + row.title" class="com_cardlist">
            <div class="com_cardlist_tit">{{ $t(row.title) }}</div>
            <div class="com_cardlist_p">{{ row.num }}</div>
          </div>
        </div>
      </template>
      <template v-else>
        <div class="job_list_tit">
          <ul>
            <li :class="{ job_list_tit_cur: rangeKind === 1 }"><a href="javascript:;" @click.prevent="rangeKind = 1">{{ $t('member_com_00375') }}</a></li>
            <li :class="{ job_list_tit_cur: rangeKind === 2 }"><a href="javascript:;" @click.prevent="rangeKind = 2">{{ $t('member_com_00376') }}</a></li>
            <li :class="{ job_list_tit_cur: rangeKind === 3 }"><a href="javascript:;" @click.prevent="rangeKind = 3">{{ $t('member_com_00374') }}</a></li>
          </ul>
        </div>
        <p v-if="rangeKind === 1">
          <input v-model="dayFrom" type="date" />
          <input v-model="dayTo" type="date" />
        </p>
        <p v-else-if="rangeKind === 2"><input v-model="monthVal" type="month" /></p>
        <p v-else><input v-model="yearVal" type="number" min="2000" max="2100" /></p>
        <div v-if="range" class="membSubGuaTwo site-pc">
          <ul>
            <li><div class="twoDivTite"><span>{{ $t('member_com_00371') }}</span></div><div class="twoDivNum"><span>{{ range.wkg }}</span></div></li>
            <li><div class="twoDivTite"><span>{{ $t('member_com_00372') }}</span></div><div class="twoDivNum"><span>{{ range.kgw }}</span></div></li>
            <li><div class="twoDivTite"><span>{{ $t('member_com_00370') }}</span></div><div class="twoDivNum"><span>{{ range.wdl }}</span></div></li>
            <li><div class="twoDivTite"><span>{{ $t('wap_00451') }}</span></div><div class="twoDivNum"><span>{{ range.xzjl }}</span></div></li>
            <li><div class="twoDivTite"><span>{{ $t('wap_com_00235') }}</span></div><div class="twoDivNum"><span>{{ range.tdjl }}</span></div></li>
            <li><div class="twoDivTite"><span>{{ $t('wap_user_00216') }}</span></div><div class="twoDivNum"><span>{{ range.yqms }}</span></div></li>
          </ul>
        </div>
        <ClientOnly>
          <ChartBox :option="viewBar" />
          <ChartBox :option="resumeBar" />
          <ChartBox :option="msBar" />
        </ClientOnly>
        <h3>{{ $t('member_com_00362') }}</h3>
        <div v-for="block in talentPies" :key="block.title">
          <h4>{{ block.title }}</h4>
          <ClientOnly>
            <ChartBox :option="pieOpt(block.dims.exp)" height="220px" />
            <ChartBox :option="pieOpt(block.dims.edu)" height="220px" />
            <ChartBox :option="pieOpt(block.dims.salary)" height="220px" />
          </ClientOnly>
        </div>
        <h3>{{ $t('member_com_00364') }}</h3>
        <table v-if="detailRows.length" class="com_table site-pc">
          <tr>
            <th>{{ $t('member_user_00106') }}</th>
            <th>{{ $t('member_com_00371') }}</th>
            <th>{{ $t('member_com_00372') }}</th>
            <th>{{ $t('member_com_00370') }}</th>
            <th>{{ $t('wap_00451') }}</th>
            <th>{{ $t('wap_com_00235') }}</th>
            <th>{{ $t('wap_user_00216') }}</th>
          </tr>
          <tr v-for="row in detailRows" :key="row.label">
            <td>{{ row.label }}</td>
            <td>{{ row.wkg }}</td>
            <td>{{ row.kgw }}</td>
            <td>{{ row.wdl }}</td>
            <td>{{ row.xzjl }}</td>
            <td>{{ row.tdjl }}</td>
            <td>{{ row.yqms }}</td>
          </tr>
        </table>
        <div class="site-h5">
          <div v-for="row in detailRows" :key="'h5d-' + row.label" class="com_cardlist">
            <div class="com_cardlist_tit">{{ row.label }}</div>
            <div class="com_cardlist_p">
              {{ $t('member_com_00371') }} {{ row.wkg }} · {{ $t('member_com_00372') }} {{ row.kgw }} ·
              {{ $t('wap_com_00235') }} {{ row.tdjl }}
            </div>
          </div>
        </div>
      </template>
    </template>
  </MemberPanel>
</template>

<style scoped>
.pack-bar {
  height: 8px;
  background: #eee;
  border-radius: 4px;
  overflow: hidden;
}
.pack-bar span {
  display: block;
  height: 100%;
  background: #2e8ded;
}
</style>
