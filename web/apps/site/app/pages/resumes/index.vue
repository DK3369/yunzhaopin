<script setup lang="ts">
import { catTree, listFailMsg, mediaUrl, PLACEHOLDER_LOGO, type CatNode } from '~/utils/site'
import type { DictItem } from '~/utils/query'

const SALARY_PRESETS: Array<{ min: number; max?: number }> = [
  { min: 2000, max: 4000 },
  { min: 4000, max: 6000 },
  { min: 6000, max: 8000 },
  { min: 8000, max: 10000 },
  { min: 10000 },
]
const AGE_PRESETS: Array<{ min: number; max?: number }> = [
  { min: 16, max: 20 },
  { min: 21, max: 30 },
  { min: 31, max: 40 },
  { min: 41, max: 50 },
  { min: 50 },
]

const route = useRoute()
const { t, locale } = useI18n()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const education = computed(() => numQuery(route.query.education) || numQuery(route.query.edu))
const exp = computed(() => numQuery(route.query.exp))
const job1 = computed(() => numQuery(route.query.job1))
const job1Son = computed(() => numQuery(route.query.job1_son))
const jobPost = computed(() => numQuery(route.query.job_post))
const provinceId = computed(() => numQuery(route.query.province_id))
const cityId = computed(() => numQuery(route.query.city_id))
const threeCityId = computed(() => numQuery(route.query.three_city_id))
const sex = computed(() => numQuery(route.query.sex))
const hy = computed(() => numQuery(route.query.hy))
const tag = computed(() => numQuery(route.query.tag))
const report = computed(() => numQuery(route.query.report))
const workType = computed(() => numQuery(route.query.type))
const minSalary = computed(() => numQuery(route.query.min_salary))
const maxSalary = computed(() => numQuery(route.query.max_salary))
const minAge = computed(() => numQuery(route.query.min_age))
const maxAge = computed(() => numQuery(route.query.max_age))
const salaryPresetId = computed(() => numQuery(route.query.salary))
const agePresetId = computed(() => numQuery(route.query.age))
const salaryPresetBound = computed(() => {
  const n = salaryPresetId.value
  return n ? SALARY_PRESETS[n - 1] : undefined
})
const agePresetBound = computed(() => {
  const n = agePresetId.value
  return n ? AGE_PRESETS[n - 1] : undefined
})
const filterMinSalary = computed(() => salaryPresetBound.value?.min ?? minSalary.value)
const filterMaxSalary = computed(() => salaryPresetBound.value?.max ?? maxSalary.value)
const filterMinAge = computed(() => agePresetBound.value?.min ?? minAge.value)
const filterMaxAge = computed(() => agePresetBound.value?.max ?? maxAge.value)
const uptime = computed(() => numQuery(route.query.uptime))
const integrity = computed(() => numQuery(route.query.integrity))
const order = computed(() => String(route.query.order || ''))
const photo = computed(() => boolQuery(route.query.photo) || boolQuery(route.query.pic))
const idcard = computed(() => boolQuery(route.query.idcard))
const work = computed(() => boolQuery(route.query.work))
const recg = computed(() => boolQuery(route.query.recg))
const { settings, me } = useSiteChrome()
const resumeGate = computed(() => {
  const needLogin = String(settings.value.com_search || '') === '1' && !me.value
  const seekerOff =
    String(settings.value.sy_user_visit_resume ?? '1') === '0' && Number(me.value?.usertype) === 1
  if (needLogin) return 'login'
  if (seekerOff) return 'seeker'
  return ''
})
const moreOpen = ref(
  !!(
    education.value ||
    exp.value ||
    hy.value ||
    sex.value ||
    report.value ||
    workType.value ||
    uptime.value ||
    integrity.value ||
    minSalary.value ||
    maxSalary.value ||
    minAge.value ||
    maxAge.value ||
    salaryPresetId.value ||
    agePresetId.value
  ),
)
const api = useApi()
const { applyToQuery } = useSubSite()

function resumeListParams(extra: Record<string, unknown> = {}) {
  return applyToQuery({
    page: page.value,
    page_size: 20,
    keyword: keyword.value || undefined,
    education: education.value,
    exp: exp.value,
    job1: job1.value,
    job1_son: job1Son.value,
    job_post: jobPost.value,
    province_id: provinceId.value,
    city_id: cityId.value,
    three_city_id: threeCityId.value,
    sex: sex.value,
    hy: hy.value,
    tag: tag.value,
    report: report.value,
    type: workType.value,
    min_salary: filterMinSalary.value,
    max_salary: filterMaxSalary.value,
    min_age: filterMinAge.value,
    max_age: filterMaxAge.value,
    uptime: uptime.value,
    integrity: integrity.value,
    order: order.value || undefined,
    photo: photo.value ? true : undefined,
    idcard: idcard.value ? true : undefined,
    work: work.value ? true : undefined,
    recg: recg.value ? true : undefined,
    ...extra,
  })
}

const listKey = computed(
  () =>
    `resumes-${locale.value}-${page.value}-${keyword.value}-${education.value}-${exp.value}-${job1.value}-${job1Son.value}-${jobPost.value}-${provinceId.value}-${cityId.value}-${threeCityId.value}-${sex.value}-${hy.value}-${tag.value}-${report.value}-${workType.value}-${filterMinSalary.value}-${filterMaxSalary.value}-${filterMinAge.value}-${filterMaxAge.value}-${uptime.value}-${integrity.value}-${order.value}-${photo.value}-${idcard.value}-${work.value}-${recg.value}`,
)

const { data, error } = await useAsyncData(
  () => listKey.value,
  () =>
    api.get<{ list: Array<Record<string, unknown>>; total: number }>('/v1/wap/resumes', resumeListParams()),
)

const { data: topResumes } = await useAsyncData(
  () => `resumes-top-${listKey.value}`,
  () =>
    page.value > 1
      ? Promise.resolve({ list: [] as Array<Record<string, unknown>> })
      : api
          .get<{ list: Array<Record<string, unknown>> }>('/v1/wap/resumes', {
            ...resumeListParams({ page: 1, page_size: 5, recg: undefined, top: true }),
          })
          .catch(() => ({ list: [] as Array<Record<string, unknown>> })),
)

const { data: cats } = await useAsyncData(
  () => `job-cats-${locale.value}`,
  () => api.get<CatNode[]>('/v1/wap/categories', { kind: 'job' }).catch(() => [] as CatNode[]),
)
const jobRoots = computed(() => catTree(cats.value || [], 40))
const jobLevel2 = computed(() => jobRoots.value.find((c) => c.id === job1.value)?.children || [])
const jobLevel3 = computed(() => jobLevel2.value.find((c) => c.id === job1Son.value)?.children || [])
const jobItems = computed(() => jobRoots.value.map((c) => ({ id: c.id, name: c.name })))
const job2Items = computed(() => jobLevel2.value.map((c) => ({ id: c.id, name: c.name })))
const job3Items = computed(() => jobLevel3.value.map((c) => ({ id: c.id, name: c.name })))

const { data: edus } = await useAsyncData(
  () => `dict-edu-user-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/educations', { source: 'user' }).catch(() => [] as DictItem[]),
)
const { data: exps } = await useAsyncData(
  () => `dict-exp-user-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/experiences', { source: 'user' }).catch(() => [] as DictItem[]),
)
const { data: provinces } = await useAsyncData(
  () => `dict-city-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/cities').catch(() => [] as DictItem[]),
)
const { data: cities } = await useAsyncData(
  () => `dict-city-child-${locale.value}-${provinceId.value || 0}`,
  () =>
    provinceId.value
      ? api
          .get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: provinceId.value })
          .catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
const { data: districts } = await useAsyncData(
  () => `dict-city-dist-${locale.value}-${cityId.value || 0}`,
  () =>
    cityId.value
      ? api
          .get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: cityId.value })
          .catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
const { data: tags } = await useAsyncData(
  () => `dict-user-tag-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/tags').catch(() => [] as DictItem[]),
)
const { data: industries } = await useAsyncData(
  () => `dict-hy-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/industries').catch(() => [] as DictItem[]),
)
const { data: reports } = await useAsyncData(
  () => `dict-user-report-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/reports', { source: 'user' }).catch(() => [] as DictItem[]),
)
const { data: jobTypes } = await useAsyncData(
  () => `dict-user-type-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/job-types', { source: 'user' }).catch(() => [] as DictItem[]),
)
const { data: adsTop } = await useAsyncData('ads-508', () =>
  api.get<Array<{ image_n?: string }>>('/v1/wap/ads', { slot: '508', limit: 1 }).catch(() => []),
)
const { data: adsSide } = await useAsyncData('ads-36', () =>
  api.get<Array<{ image_n?: string }>>('/v1/wap/ads', { slot: '36', limit: 5 }).catch(() => []),
)
const { data: recSide } = await useAsyncData(
  () => `resumes-rec-side-${locale.value}`,
  () =>
    api
      .get<{ list: Array<Record<string, unknown>> }>('/v1/wap/resumes', { recg: true, page_size: 18 })
      .catch(() => ({ list: [] as Array<Record<string, unknown>> })),
)

const sexItems = computed<DictItem[]>(() => [
  { id: 1, name: t('common_02092') },
  { id: 2, name: t('common_02069') },
])
const uptimeItems = computed<DictItem[]>(() => [
  { id: 1, name: t('common_01940') },
  { id: 3, name: t('wap_00432') },
  { id: 7, name: t('wap_00433') },
  { id: 30, name: t('admin_user_00175') },
  { id: 90, name: t('wap_00431') },
])
const integrityItems = computed<DictItem[]>(() => [
  { id: 1, name: `55%${t('common_01942')}` },
  { id: 2, name: `65%${t('common_01942')}` },
  { id: 3, name: `75%${t('common_01942')}` },
  { id: 4, name: `85%${t('common_01942')}` },
])
const salaryPresets = computed(() =>
  SALARY_PRESETS.map((p) => ({
    min: p.min,
    max: p.max,
    label: p.max ? `${p.min}-${p.max}` : `${p.min}${t('common_01942')}`,
  })),
)
const agePresets = computed(() =>
  AGE_PRESETS.map((p) => ({
    min: p.min,
    max: p.max,
    label: p.max ? `${p.min}-${p.max}${t('home.age_suffix')}` : `50${t('wap_com_00296')}`,
  })),
)
const salaryItems = computed<DictItem[]>(() =>
  salaryPresets.value.map((p, i) => ({ id: i + 1, name: p.label })),
)
const ageItems = computed<DictItem[]>(() => agePresets.value.map((p, i) => ({ id: i + 1, name: p.label })))

useSeoMeta({ title: t('default_00312') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
useListLoginGate(error)
const topList = computed(() => (page.value > 1 ? [] : topResumes.value?.list || []).slice(0, 5))
const list = computed(() => {
  const raw = data.value?.list || []
  const ids = new Set(topList.value.map((r) => String(r.uid)))
  return raw.filter((r) => !ids.has(String(r.uid)))
})
const dictName = (items: DictItem[] | null | undefined, id?: number) =>
  items?.find((x) => x.id === id)?.name || ''
const jobLabel = computed(() => {
  const hit = [...jobItems.value, ...job2Items.value, ...job3Items.value].find(
    (c) => c.id === jobPost.value || c.id === job1Son.value || c.id === job1.value,
  )
  return hit?.name || ''
})
const cityLabel = computed(() => {
  const hit = [...(provinces.value || []), ...(cities.value || []), ...(districts.value || [])].find(
    (c) => c.id === threeCityId.value || c.id === cityId.value || c.id === provinceId.value,
  )
  return hit?.name || ''
})
const salaryActive = (min?: number, max?: number) => minSalary.value === min && maxSalary.value === max
const ageActive = (min?: number, max?: number) => minAge.value === min && maxAge.value === max
const salaryLabel = computed(() => {
  if (minSalary.value && maxSalary.value) return `${minSalary.value}-${maxSalary.value}`
  if (minSalary.value) return `${minSalary.value}${t('common_01942')}`
  if (maxSalary.value) return `${maxSalary.value}${t('default_00374')}`
  return ''
})
const ageLabel = computed(() => {
  if (minAge.value && maxAge.value) return `${minAge.value}-${maxAge.value}`
  if (minAge.value) return `${minAge.value}${t('default_00373')}`
  return ''
})

const selected = computed(() => {
  const rows: Array<{ param: string; name: string; extra?: Record<string, undefined> }> = []
  if (keyword.value) rows.push({ param: 'keyword', name: keyword.value })
  if (job1.value) {
    const n = dictName(jobItems.value, job1.value)
    if (n) rows.push({ param: 'job1', name: n, extra: { job1_son: undefined, job_post: undefined } })
  }
  if (job1Son.value) {
    const n = dictName(job2Items.value, job1Son.value)
    if (n) rows.push({ param: 'job1_son', name: n, extra: { job_post: undefined } })
  }
  if (jobPost.value) {
    const n = dictName(job3Items.value, jobPost.value)
    if (n) rows.push({ param: 'job_post', name: n })
  }
  if (provinceId.value) {
    const n = dictName(provinces.value, provinceId.value)
    if (n) rows.push({ param: 'province_id', name: n, extra: { city_id: undefined, three_city_id: undefined } })
  }
  if (cityId.value) {
    const n = dictName(cities.value, cityId.value)
    if (n) rows.push({ param: 'city_id', name: n, extra: { three_city_id: undefined } })
  }
  if (threeCityId.value) {
    const n = dictName(districts.value, threeCityId.value)
    if (n) rows.push({ param: 'three_city_id', name: n })
  }
  const tagN = dictName(tags.value, tag.value)
  if (tagN) rows.push({ param: 'tag', name: tagN })
  if (salaryLabel.value) {
    rows.push({ param: 'min_salary', name: salaryLabel.value, extra: { max_salary: undefined } })
  }
  if (ageLabel.value) {
    rows.push({ param: 'min_age', name: ageLabel.value, extra: { max_age: undefined } })
  }
  const hyN = dictName(industries.value, hy.value)
  if (hyN) rows.push({ param: 'hy', name: hyN })
  const eduN = dictName(edus.value, education.value)
  if (eduN) rows.push({ param: 'education', name: eduN })
  const expN = dictName(exps.value, exp.value)
  if (expN) rows.push({ param: 'exp', name: expN })
  const sexN = dictName(sexItems.value, sex.value)
  if (sexN) rows.push({ param: 'sex', name: sexN })
  const reportN = dictName(reports.value, report.value)
  if (reportN) rows.push({ param: 'report', name: reportN })
  const typeN = dictName(jobTypes.value, workType.value)
  if (typeN) rows.push({ param: 'type', name: typeN })
  const upN = dictName(uptimeItems.value, uptime.value)
  if (upN) rows.push({ param: 'uptime', name: upN })
  const intN = dictName(integrityItems.value, integrity.value)
  if (intN) rows.push({ param: 'integrity', name: intN })
  if (idcard.value) rows.push({ param: 'idcard', name: t('default_00318') })
  if (work.value) rows.push({ param: 'work', name: t('member_com_00340') })
  return rows
})

function applyRange(minKey: string, maxKey: string, e: Event) {
  const form = e.target as HTMLFormElement
  const fd = new FormData(form)
  return navigateTo({
    query: mergeQuery(route.query, {
      [minKey]: numQuery(fd.get(minKey)) || undefined,
      [maxKey]: numQuery(fd.get(maxKey)) || undefined,
    }),
  })
}

function goPage(p: number) {
  return navigateTo({ query: { ...route.query, page: p } })
}

function recName(row: Record<string, unknown>) {
  return String(row.display_name || row.name || row.uid || '')
}
function recPhoto(row: Record<string, unknown>) {
  return mediaUrl(String(row.photo_n || row.photo || ''), PLACEHOLDER_LOGO)
}
</script>

<template>
  <div class="site-pc">
    <div class="yun_jobbody">
      <div class="yun_content">
        <div class="current_Location com_current_Location png none">
          <div class="fl">
            {{ $t('common_01498') }}：
            <NuxtLink to="/">{{ $t('common.home') }}</NuxtLink> >
            <span>{{ $t('default_00312') }}</span>
          </div>
        </div>
        <div v-if="adsTop?.length" class="yun_jobbanner">
          <img v-for="(ad, i) in adsTop" :key="i" :src="ad.image_n" alt="" />
        </div>
        <form action="/resumes" method="get" class="jobsearch_newbox">
          <div class="yun_job_search">
            <div class="yun_job_search_cont searchContButton">
              <div class="yun_job_search_textcont">
                <input class="Search_jobs_text" name="keyword" :value="keyword" :placeholder="$t('admin_system_00198')" />
              </div>
              <input class="Search_jobs_submit yun_bg_color jobsSubmit" type="submit" :value="$t('common.search')" />
            </div>
          </div>
        </form>
        <div class="Search_jobs_box">
          <FilterRow
            v-if="!job1"
            :label="$t('common.job')"
            param="job1"
            :items="jobItems"
            :current="job1"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="job1 && !job1Son"
            :label="$t('common_01972')"
            param="job1_son"
            :items="job2Items"
            :current="job1Son"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="job1Son"
            :label="$t('admin_00223')"
            param="job_post"
            :items="job3Items"
            :current="jobPost"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <CityFilterBox
            :label="$t('member_com_00378')"
            path="/resumes"
            :all-label="$t('common.all')"
            :unlimited-label="$t('common_01936')"
            :more-label="$t('common.more')"
            :provinces="provinces || []"
            :cities="cities || []"
            :districts="districts || []"
            :province-id="provinceId"
            :city-id="cityId"
            :three-city-id="threeCityId"
          />
          <FilterRow
            v-if="(tags || []).length"
            extra-class="search_more"
            :label="$t('admin_user_company_00159')"
            param="tag"
            :items="tags || []"
            :current="tag"
            path="/resumes"
            :all-label="$t('common.all')"
            :limit="10"
          />
          <div class="searchmorelist" :class="{ none: !moreOpen }">
            <div class="Search_jobs_form_list search_more">
              <div class="Search_jobs_name">{{ $t('member_user_00106') }}：</div>
              <div>
                <NuxtLink
                  :to="{ path: '/resumes', query: mergeQuery(route.query, { min_salary: undefined, max_salary: undefined }) }"
                  class="Search_jobs_sub_a"
                  :class="{ Search_jobs_sub_cur: !minSalary && !maxSalary }"
                >
                  {{ $t('common.all') }}
                </NuxtLink>
                <NuxtLink
                  v-for="p in salaryPresets"
                  :key="p.label"
                  :to="{ path: '/resumes', query: mergeQuery(route.query, { min_salary: p.min, max_salary: p.max }) }"
                  class="Search_jobs_cxz"
                  :class="{ Search_jobs_sub_cur: salaryActive(p.min, p.max) }"
                >
                  {{ p.label }}
                </NuxtLink>
              </div>
              <form @submit.prevent="applyRange('min_salary', 'max_salary', $event)">
                <input class="job_xz_text" name="min_salary" :value="minSalary || ''" inputmode="numeric" />
                <span class="job_xz_line">-</span>
                <input class="job_xz_text" name="max_salary" :value="maxSalary || ''" inputmode="numeric" />
                <input class="job_xz_bth" type="submit" :value="$t('common.confirm')" />
              </form>
            </div>
            <div class="Search_jobs_form_list search_more">
              <div class="Search_jobs_name">{{ $t('wap_com_00302') }}：</div>
              <div>
                <NuxtLink
                  :to="{ path: '/resumes', query: mergeQuery(route.query, { min_age: undefined, max_age: undefined }) }"
                  class="Search_jobs_sub_a"
                  :class="{ Search_jobs_sub_cur: !minAge && !maxAge }"
                >
                  {{ $t('common.all') }}
                </NuxtLink>
                <NuxtLink
                  v-for="p in agePresets"
                  :key="p.label"
                  :to="{ path: '/resumes', query: mergeQuery(route.query, { min_age: p.min, max_age: p.max }) }"
                  class="Search_jobs_sub_a"
                  :class="{ Search_jobs_sub_cur: ageActive(p.min, p.max) }"
                >
                  {{ p.label }}
                </NuxtLink>
              </div>
              <form @submit.prevent="applyRange('min_age', 'max_age', $event)">
                <input class="job_xz_text" name="min_age" :value="minAge || ''" inputmode="numeric" />
                <span class="job_xz_line">-</span>
                <input class="job_xz_text" name="max_age" :value="maxAge || ''" inputmode="numeric" />
                <input class="job_xz_bth" type="submit" :value="$t('common.confirm')" />
              </form>
            </div>
            <FilterRow
              extra-class="search_more"
              :label="$t('member_user_00151')"
              param="integrity"
              :items="integrityItems"
              :current="integrity"
              path="/resumes"
              :all-label="$t('common.all')"
            />
            <FilterRow
              extra-class="search_more"
              :label="$t('default_00361')"
              param="hy"
              :items="industries || []"
              :current="hy"
              path="/resumes"
              :all-label="$t('common.all')"
            />
            <FilterRow
              extra-class="search_more"
              :label="$t('wap_com_00283')"
              param="education"
              :items="edus || []"
              :current="education"
              path="/resumes"
              :all-label="$t('common.all')"
            />
            <FilterRow
              extra-class="search_more"
              :label="$t('wap_user_00240')"
              param="exp"
              :items="exps || []"
              :current="exp"
              path="/resumes"
              :all-label="$t('common.all')"
            />
            <FilterRow
              extra-class="search_more"
              :label="$t('wap_com_00332')"
              param="sex"
              :items="sexItems"
              :current="sex"
              path="/resumes"
              :all-label="$t('common.all')"
            />
            <FilterRow
              v-if="(reports || []).length"
              extra-class="search_more"
              :label="$t('wap_com_00279')"
              param="report"
              :items="reports || []"
              :current="report"
              path="/resumes"
              :all-label="$t('common.all')"
            />
            <FilterRow
              extra-class="search_more"
              :label="$t('wap_00326')"
              param="uptime"
              :items="uptimeItems"
              :current="uptime"
              path="/resumes"
              :all-label="$t('common.all')"
            />
            <FilterRow
              v-if="(jobTypes || []).length"
              extra-class="search_more"
              :label="$t('wap_user_00012')"
              param="type"
              :items="jobTypes || []"
              :current="workType"
              path="/resumes"
              :all-label="$t('common.all')"
            />
          </div>
          <div v-if="selected.length" class="Search_close_box">
            <div>
              <div class="Search_clear">
                <NuxtLink to="/resumes">{{ $t('default_00059') }}</NuxtLink>
              </div>
              <span class="Search_close_box_s">{{ $t('default_00058') }}</span>
            </div>
            <NuxtLink
              v-for="s in selected"
              :key="s.param"
              :to="{ path: '/resumes', query: mergeQuery(route.query, { [s.param]: undefined, ...(s.extra || {}) }) }"
              class="Search_jobs_c_a disc_fac"
            >
              {{ s.name }}
            </NuxtLink>
          </div>
        </div>
        <div class="user_zk">
          <a href="javascript:;" class="user_zk_b" @click.prevent="moreOpen = !moreOpen" />
        </div>
        <div class="search_h1_box">
          <div class="search_h1_box_title">
            <ul class="search_h1_box_list">
              <li :class="{ search_job_all: !photo && !idcard && !work && !order }">
                <NuxtLink to="/resumes">{{ $t('common_02274') }}</NuxtLink>
                <i class="search_h1_box_list_icon" />
              </li>
              <li :class="{ search_Filter_current: order === 'lastdate' }">
                <NuxtLink :to="{ path: '/resumes', query: mergeQuery(route.query, { order: order === 'lastdate' ? undefined : 'lastdate' }) }">
                  <span>{{ $t('wap_00326') }}</span><i class="search_Filter_icon" />
                </NuxtLink>
              </li>
              <li :class="{ search_Filter_current: order === 'ctime' }">
                <NuxtLink :to="{ path: '/resumes', query: mergeQuery(route.query, { order: order === 'ctime' ? undefined : 'ctime' }) }">
                  <span>{{ $t('admin_user_weipin_00030') }}</span><i class="search_Filter_icon" />
                </NuxtLink>
              </li>
              <li :class="{ search_h1_box_cur: photo }" class="job_tj_t">
                <NuxtLink :to="{ path: '/resumes', query: mergeQuery(route.query, { photo: photo ? undefined : '1' }) }" class="job_zt">
                  {{ $t('common_02275') }}<i class="job_tj_chk" />
                </NuxtLink>
              </li>
              <li :class="{ search_h1_box_cur: idcard }">
                <NuxtLink :to="{ path: '/resumes', query: mergeQuery(route.query, { idcard: idcard ? undefined : '1' }) }" class="job_zt">
                  {{ $t('member_com_00026') }}<i class="job_tj_chk" />
                </NuxtLink>
              </li>
              <li :class="{ search_h1_box_cur: work }">
                <NuxtLink :to="{ path: '/resumes', query: mergeQuery(route.query, { work: work ? undefined : '1' }) }" class="job_zt">
                  {{ $t('member_com_00340') }}<i class="job_tj_chk" />
                </NuxtLink>
              </li>
            </ul>
          </div>
        </div>
        <div class="user_left_sidebar">
          <div v-if="resumeGate" class="firm_login" style="padding: 40px 20px">
            <p>{{ resumeGate === 'login' ? $t('wap_00376') : $t('resume_00038') }}</p>
            <NuxtLink v-if="resumeGate === 'login'" to="/login" class="firm_login_dl">{{ $t('common.login') }}</NuxtLink>
          </div>
          <p v-else-if="error" class="muted" style="padding: 30px 0">{{ failMsg }}</p>
          <template v-else>
            <ResumeCard v-for="r in topList" :key="'top-' + String(r.uid)" :row="r" />
            <ResumeCard v-for="r in list" :key="String(r.uid)" :row="r" />
            <EmptyState v-if="!list.length && !topList.length" />
          </template>
          <Pager v-if="!resumeGate" :page="page" :page-size="20" :total="data?.total || 0" @update:page="goPage" />
        </div>
        <div v-if="(adsSide && adsSide.length) || (recSide?.list || []).length" class="yun_job_list_right">
          <div v-if="adsSide?.length" class="yun_job_list_right_banner">
            <img v-for="(ad, i) in adsSide" :key="i" :src="ad.image_n" alt="" />
          </div>
          <div v-if="(recSide?.list || []).length" class="user_recommendation">
            <div class="job_recommendation_title">
              <span class="job_recommendation_span">{{ $t('default_00320') }}</span>
            </div>
            <div class="userresume_recommendation">
              <ul>
                <li v-for="row in recSide?.list || []" :key="String(row.uid)">
                  <div class="userresume_people_box">
                    <div class="userresume_people_box_rt fl">
                      <NuxtLink :to="`/resumes/${row.uid}`">
                        <img :src="recPhoto(row)" width="50" height="50" alt="" />
                      </NuxtLink>
                      <i class="userresume_people_box_rt_tj" />
                    </div>
                    <div class="userresume_people_box_ft fl">
                      <div class="userresume_people_box_ft_nm">
                        <NuxtLink :to="`/resumes/${row.uid}`">{{ recName(row) }}</NuxtLink>
                      </div>
                      <div class="userresume_people_box_ft_v">
                        <template v-if="row.exp_n">{{ row.exp_n }}{{ $t('home.experience_suffix') }}</template>
                        <i v-if="row.exp_n && row.education_n" class="userresume_line">|</i>
                        <template v-if="row.education_n">{{ row.education_n }}{{ $t('home.education_suffix') }}</template>
                      </div>
                      <div class="userresume_people_box_ft_y">{{ row.expect_name }}</div>
                    </div>
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <section class="site-h5">
    <div class="job_header" style="position: relative">
      <form class="job_header_center" action="/resumes" method="get" style="padding: 0.16rem 0.32rem">
        <input
          class="searchnew"
          name="keyword"
          :value="keyword"
          :placeholder="$t('wap_00575')"
          style="width: 100%; height: 0.8rem; border-radius: 0.4rem; padding: 0 0.32rem; background: #f5f5f5"
        />
      </form>
    </div>
    <div class="job_header_nav resumeAdeFlex">
      <div class="job_header_nav_left category">
        <ul>
          <li :class="{ active: !recg }">
            <NuxtLink :to="{ path: '/resumes', query: mergeQuery(route.query, { recg: undefined }) }">{{
              $t('common.latest')
            }}</NuxtLink>
          </li>
          <li :class="{ active: recg }">
            <NuxtLink :to="{ path: '/resumes', query: mergeQuery(route.query, { recg: '1' }) }">{{
              $t('common.recommended')
            }}</NuxtLink>
          </li>
        </ul>
      </div>
      <H5FilterBar
        :all-label="$t('common.all')"
        :tabs="[
          {
            key: 'province_id',
            label: $t('common_02110'),
            current: cityLabel,
            items: provinces || [],
            childKey: 'city_id',
            childItems: cities || [],
            grandKey: 'three_city_id',
            grandItems: districts || [],
          },
          {
            key: 'job1',
            label: $t('wap_com_00021'),
            current: jobLabel,
            items: jobItems,
            childKey: 'job1_son',
            childItems: job2Items,
            grandKey: 'job_post',
            grandItems: job3Items,
          },
          {
            key: 'more',
            label: $t('wap_00238'),
            kind: 'more',
            items: [],
            groups: [
              { label: $t('home.education_suffix'), param: 'education', items: edus || [] },
              { label: $t('home.experience_suffix'), param: 'exp', items: exps || [] },
              { label: $t('wap_user_00016'), param: 'salary', items: salaryItems, extraClear: { min_salary: undefined, max_salary: undefined } },
              { label: $t('wap_com_00284'), param: 'age', items: ageItems, extraClear: { min_age: undefined, max_age: undefined } },
              { label: $t('member_user_00151'), param: 'integrity', items: integrityItems },
              { label: $t('wap_user_00012'), param: 'type', items: jobTypes || [] },
              { label: $t('wap_com_00303'), param: 'sex', items: sexItems },
              { label: $t('admin_user_company_00373'), param: 'hy', items: industries || [] },
              { label: $t('wap_com_00279'), param: 'report', items: reports || [] },
              { label: $t('wap_00326'), param: 'uptime', items: uptimeItems },
              { label: $t('common.more'), param: 'tag', items: tags || [] },
            ],
          },
        ]"
      />
    </div>
    <div v-if="resumeGate" class="firm_login" style="padding: 0.4rem">
      <p>{{ resumeGate === 'login' ? $t('wap_00376') : $t('resume_00038') }}</p>
      <NuxtLink v-if="resumeGate === 'login'" to="/login">{{ $t('common.login') }}</NuxtLink>
    </div>
    <p v-else-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
    <template v-else>
      <ResumeCard v-for="r in topList" :key="'htop-' + String(r.uid)" :row="r" />
      <ResumeCard v-for="r in list" :key="String(r.uid)" :row="r" />
      <EmptyState v-if="!list.length && !topList.length" />
    </template>
    <Pager :page="page" :page-size="20" :total="data?.total || 0" @update:page="goPage" />
  </section>
</template>
