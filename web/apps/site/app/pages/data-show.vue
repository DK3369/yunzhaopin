<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

type Dist = { key: number; num: number; rate: number }
type Point = { month: string; count: number }
type DictItem = { id: number; name: string }

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const cityLevel = computed(() => {
  const n = Number(settings.value.sy_datashow_city_lev || 2)
  return n === 1 || n === 3 ? n : 2
})
const pageTitle = computed(() => {
  const custom = String(settings.value.sy_datashow_title || '').trim()
  return custom ? `${custom}${t('wap_00124')}` : t('wap_01776') + (settings.value.sy_webname || '')
})

function dictName(list: DictItem[] | undefined, key: number) {
  const hit = (list || []).find((d) => Number(d.id) === Number(key))
  return hit?.name || String(key)
}

const ageLabels = ['16-24', '25-30', '31-40', '41-65']

const { data, error } = await useAsyncData('data-show-board', async () => {
  const cityBody = { level: cityLevel.value }
  const emptyD: Dist[] = []
  const emptyP: Point[] = []
  const [sex, edu, exp, age, rCity, cCity, scale, prop, ureg, cjob, clog, bundle, cities] =
    await Promise.all([
      api.post<Dist[]>('/v1/wap/data-show/resume-sex', {}).catch(() => emptyD),
      api.post<Dist[]>('/v1/wap/data-show/resume-edu', {}).catch(() => emptyD),
      api.post<Dist[]>('/v1/wap/data-show/resume-exp', {}).catch(() => emptyD),
      api.post<Dist[]>('/v1/wap/data-show/resume-age', {}).catch(() => emptyD),
      api.post<Dist[]>('/v1/wap/data-show/resume-city', cityBody).catch(() => emptyD),
      api.post<Dist[]>('/v1/wap/data-show/company-city', cityBody).catch(() => emptyD),
      api.post<Dist[]>('/v1/wap/data-show/company-scale', {}).catch(() => emptyD),
      api.post<Dist[]>('/v1/wap/data-show/company-property', {}).catch(() => emptyD),
      api.post<Point[]>('/v1/wap/data-show/user-register-trend', {}).catch(() => emptyP),
      api.post<Point[]>('/v1/wap/data-show/company-job-trend', {}).catch(() => emptyP),
      api.post<Point[]>('/v1/wap/data-show/company-login-trend', {}).catch(() => emptyP),
      api.get<{
        educations_user: DictItem[]
        experiences: DictItem[]
        company_natures: DictItem[]
        company_sizes: DictItem[]
      }>('/v1/wap/initjobs').catch(() => ({
        educations_user: [] as DictItem[],
        experiences: [] as DictItem[],
        company_natures: [] as DictItem[],
        company_sizes: [] as DictItem[],
      })),
      api.get<DictItem[]>('/v1/wap/dict/cities').catch(() => [] as DictItem[]),
    ])
  return {
    sex,
    edu,
    exp,
    age,
    rCity,
    cCity,
    scale,
    prop,
    ureg,
    cjob,
    clog,
    edus: bundle.educations_user,
    exps: bundle.experiences,
    natures: bundle.company_natures,
    sizes: bundle.company_sizes,
    cities,
  }
})

useSeoMeta({ title: () => pageTitle.value })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))

function sexLabel(key: number) {
  if (key === 1) return t('common_02092')
  if (key === 2) return t('common_02069')
  return String(key)
}
</script>

<template>
  <section class="data-show">
    <h1>{{ pageTitle }}</h1>
    <p class="muted">{{ $t('wap_01501') }}{{ $t('wap_01502') }}</p>
    <p v-if="error" class="muted">{{ failMsg }}</p>
    <template v-else-if="data">
      <h2>{{ $t('common.resume') }}</h2>
      <h3>{{ $t('wap_00064') }}</h3>
      <div v-for="row in data.sex" :key="'sx' + row.key" class="data-show-row">
        <span>{{ sexLabel(row.key) }}</span>
        <i class="data-show-bar" :style="{ width: Math.round(row.rate * 100) + '%' }" />
        <em>{{ row.num }}</em>
      </div>
      <h3>{{ $t('wap_00016') }}</h3>
      <div v-for="row in data.edu" :key="'ed' + row.key" class="data-show-row">
        <span>{{ dictName(data.edus, row.key) }}</span>
        <i class="data-show-bar" :style="{ width: Math.round(row.rate * 100) + '%' }" />
        <em>{{ row.num }}</em>
      </div>
      <h3>{{ $t('wap_00018') }}</h3>
      <div v-for="row in data.exp" :key="'xp' + row.key" class="data-show-row">
        <span>{{ dictName(data.exps, row.key) }}</span>
        <i class="data-show-bar" :style="{ width: Math.round(row.rate * 100) + '%' }" />
        <em>{{ row.num }}</em>
      </div>
      <h3>{{ $t('wap_00063') }}</h3>
      <div v-for="row in data.age" :key="'ag' + row.key" class="data-show-row">
        <span>{{ ageLabels[row.key] || row.key }}</span>
        <i class="data-show-bar" :style="{ width: Math.round(row.rate * 100) + '%' }" />
        <em>{{ row.num }}</em>
      </div>
      <h3>{{ $t('wap_00015') }}</h3>
      <div v-for="row in data.rCity" :key="'rc' + row.key" class="data-show-row">
        <span>{{ dictName(data.cities, row.key) }}</span>
        <i class="data-show-bar" :style="{ width: Math.round(row.rate * 100) + '%' }" />
        <em>{{ row.num }}</em>
      </div>

      <h2>{{ $t('wap_00060') }}</h2>
      <h3>{{ $t('wap_00052') }}</h3>
      <div v-for="row in data.cCity" :key="'cc' + row.key" class="data-show-row">
        <span>{{ dictName(data.cities, row.key) }}</span>
        <i class="data-show-bar" :style="{ width: Math.round(row.rate * 100) + '%' }" />
        <em>{{ row.num }}</em>
      </div>
      <h3>{{ $t('wap_00054') }}</h3>
      <div v-for="row in data.scale" :key="'sc' + row.key" class="data-show-row">
        <span>{{ dictName(data.sizes, row.key) }}</span>
        <i class="data-show-bar" :style="{ width: Math.round(row.rate * 100) + '%' }" />
        <em>{{ row.num }}</em>
      </div>
      <h3>{{ $t('wap_com_00159') }}</h3>
      <div v-for="row in data.prop" :key="'pr' + row.key" class="data-show-row">
        <span>{{ dictName(data.natures, row.key) }}</span>
        <i class="data-show-bar" :style="{ width: Math.round(row.rate * 100) + '%' }" />
        <em>{{ row.num }}</em>
      </div>

      <h2>{{ $t('wap_00047') }}</h2>
      <table class="data-show-table">
        <thead>
          <tr>
            <th>{{ $t('common.latest') }}</th>
            <th>{{ $t('common.register') }}</th>
            <th>{{ $t('common.job') }}</th>
            <th>{{ $t('common.login') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in data.ureg" :key="row.month">
            <td>{{ row.month }}</td>
            <td>{{ row.count }}</td>
            <td>{{ data.cjob.find((p) => p.month === row.month)?.count ?? 0 }}</td>
            <td>{{ data.clog.find((p) => p.month === row.month)?.count ?? 0 }}</td>
          </tr>
        </tbody>
      </table>
    </template>
  </section>
</template>
