<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type JobOpt = { id: number; name: string }
type DayPoint = { date: string; td?: string; cnt: number }
type Trend = {
  tdnum: number
  lookjobnum: number
  useridmsg: number
  cgl: number
  apply: DayPoint[]
  look: DayPoint[]
  jobs: JobOpt[]
}
type Slice = { fields: string; num: number }

const api = useApi()
const { t } = useI18n()

function ymd(d: Date) {
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  return `${y}-${m}-${day}`
}
const now = new Date()
const edate = ref(ymd(now))
const s = new Date(now)
s.setDate(s.getDate() - 29)
const sdate = ref(ymd(s))
const jobId = ref(0)
const pieType = ref(1)
const pieTabs = [
  { n: 1, key: 'common_02110' },
  { n: 2, key: 'wap_com_00301' },
  { n: 3, key: 'member_user_00106' },
  { n: 4, key: 'wap_user_00240' },
] as const

const { data, error } = await useAsyncData(
  () => `com-tongji-${jobId.value}-${sdate.value}-${edate.value}`,
  () =>
    api.post<Trend>('/v1/mcenter/com-tongji/trend', {
      job_id: jobId.value || undefined,
      sdate: sdate.value,
      edate: edate.value,
    }),
  { watch: [jobId, sdate, edate] },
)
const { data: pie } = await useAsyncData(
  () => `com-tongji-pie-${pieType.value}-${jobId.value}-${sdate.value}-${edate.value}`,
  () =>
    api
      .post<Slice[]>('/v1/mcenter/com-tongji/pie', {
        type: pieType.value,
        job_id: jobId.value || undefined,
        sdate: sdate.value,
        edate: edate.value,
      })
      .catch(() => [] as Slice[]),
  { watch: [pieType, jobId, sdate, edate] },
)

const lineOpt = computed(() => {
  const apply = data.value?.apply || []
  const look = data.value?.look || []
  const labels = apply.map((p) => p.date)
  return {
    tooltip: { trigger: 'axis' },
    legend: { data: [t('wap_com_00235'), t('member_com_00372')] },
    grid: { left: 40, right: 16, top: 40, bottom: 32 },
    xAxis: { type: 'category', data: labels },
    yAxis: { type: 'value', minInterval: 1 },
    series: [
      { type: 'line', name: t('wap_com_00235'), smooth: true, data: apply.map((p) => Number(p.cnt || 0)) },
      { type: 'line', name: t('member_com_00372'), smooth: true, data: look.map((p) => Number(p.cnt || 0)) },
    ],
  }
})
const pieOpt = computed(() => ({
  tooltip: { trigger: 'item' },
  series: [
    {
      type: 'pie',
      radius: '62%',
      data: (Array.isArray(pie.value) ? pie.value : []).map((s) => ({
        name: s.fields,
        value: Number(s.num || 0),
      })),
    },
  ],
}))

useSeoMeta({ title: t('admin_tool_00181') })
</script>

<template>
  <MemberPanel :title="$t('admin_tool_00181')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <p>
        <select v-model.number="jobId">
          <option :value="0">{{ $t('wap_com_00420') }}</option>
          <option v-for="j in data?.jobs || []" :key="j.id" :value="j.id">{{ j.name }}</option>
        </select>
        <input v-model="sdate" type="date" />
        <input v-model="edate" type="date" />
      </p>
      <div class="membSubGuaTwo site-pc">
        <ul>
          <li><div class="twoDivTite"><span>{{ $t('wap_com_00235') }}</span></div><div class="twoDivNum"><span>{{ data?.tdnum ?? 0 }}</span></div></li>
          <li><div class="twoDivTite"><span>{{ $t('member_com_00372') }}</span></div><div class="twoDivNum"><span>{{ data?.lookjobnum ?? 0 }}</span></div></li>
          <li><div class="twoDivTite"><span>{{ $t('wap_user_00216') }}</span></div><div class="twoDivNum"><span>{{ data?.useridmsg ?? 0 }}</span></div></li>
          <li><div class="twoDivTite"><span>{{ $t('wap_com_00426') }}</span></div><div class="twoDivNum"><span>{{ data?.cgl ?? 0 }}%</span></div></li>
        </ul>
      </div>
      <ClientOnly>
        <ChartBox :option="lineOpt" />
      </ClientOnly>
      <div class="job_list_tit">
        <ul>
          <li v-for="tab in pieTabs" :key="tab.n" :class="{ job_list_tit_cur: pieType === tab.n }">
            <a href="javascript:;" @click.prevent="pieType = tab.n">{{ $t(tab.key) }}</a>
          </li>
        </ul>
      </div>
      <ClientOnly>
        <ChartBox :option="pieOpt" />
      </ClientOnly>
      <p><NuxtLink to="/com/stats">{{ $t('admin_tool_00224') }}</NuxtLink></p>
    </template>
  </MemberPanel>
</template>
