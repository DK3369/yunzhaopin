<script setup lang="ts">
import { catTree, formatSalary, listFailMsg, type CatNode, type JobLike } from '~/utils/site'
import type { DictItem } from '~/utils/query'

const route = useRoute()
const { t, locale } = useI18n()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const job1 = computed(() => numQuery(route.query.job1))
const job1Son = computed(() => numQuery(route.query.job1_son))
const jobPost = computed(() => numQuery(route.query.job_post))
const provinceId = computed(() => numQuery(route.query.province_id))
const cityId = computed(() => numQuery(route.query.city_id))
const threeCityId = computed(() => numQuery(route.query.three_city_id))
const edu = computed(() => numQuery(route.query.edu))
const exp = computed(() => numQuery(route.query.exp))
const salaryId = computed(() => numQuery(route.query.salary))
const hy = computed(() => numQuery(route.query.hy))
const welfare = computed(() => numQuery(route.query.welfare))
const report = computed(() => numQuery(route.query.report))
const uptime = computed(() => numQuery(route.query.uptime))
const sex = computed(() => numQuery(route.query.sex))
const urgent = computed(() => route.query.urgent === '1')
const rec = computed(() => route.query.rec === '1')
const cert = computed(() => route.query.cert === '1')
const order = computed(() => String(route.query.order || ''))
const salaryBound = computed(() => (salaryId.value ? SALARY_BOUNDS[salaryId.value] : undefined))
const { settings, hotSearches } = useSiteChrome()
const sexSwitch = computed(() => String(settings.value.com_job_sexswitch || '') === '1')
const hiddenFilters = computed(() => {
  const skip = new Set(['keyword', 'page'])
  return Object.entries(route.query)
    .filter(([k, v]) => !skip.has(k) && v != null && String(v) !== '')
    .map(([k, v]) => [k, String(Array.isArray(v) ? v[0] : v)] as const)
})
const freeTel = computed(() => String(settings.value.sy_freewebtel || ''))
const api = useApi()

const { data, error } = await useAsyncData(
  () =>
    `jobs-${locale.value}-${page.value}-${keyword.value}-${job1.value}-${job1Son.value}-${jobPost.value}-${provinceId.value}-${cityId.value}-${threeCityId.value}-${edu.value}-${exp.value}-${salaryId.value}-${hy.value}-${welfare.value}-${report.value}-${uptime.value}-${sex.value}-${urgent.value}-${rec.value}-${cert.value}-${order.value}`,
  () =>
    api.get<{ list: JobLike[]; total: number }>('/v1/wap/jobs', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
      job1: job1.value,
      job1_son: job1Son.value,
      job_post: jobPost.value,
      province_id: provinceId.value,
      city_id: cityId.value,
      three_city_id: threeCityId.value,
      edu: edu.value,
      exp: exp.value,
      hy: hy.value,
      welfare: welfare.value,
      report: report.value,
      uptime: uptime.value,
      sex: sex.value,
      min_salary: salaryBound.value?.min_salary,
      max_salary: salaryBound.value?.max_salary,
      urgent: urgent.value ? true : undefined,
      rec: rec.value ? true : undefined,
      cert: cert.value ? true : undefined,
      order: order.value || undefined,
    }),
)
const listRaw = computed(() => data.value?.list || [])

const { data: cats } = await useAsyncData(
  () => `job-cats-${locale.value}`,
  () => api.get<CatNode[]>('/v1/wap/categories', { kind: 'job' }).catch(() => [] as CatNode[]),
)
const jobRoots = computed(() => catTree(cats.value || [], 40))
const jobLevel2 = computed(() => jobRoots.value.find((c) => c.id === job1.value)?.children || [])
const jobLevel3 = computed(() => jobLevel2.value.find((c) => c.id === job1Son.value)?.children || [])

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
const { data: edus } = await useAsyncData(
  () => `dict-edu-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/educations').catch(() => [] as DictItem[]),
)
const { data: exps } = await useAsyncData(
  () => `dict-exp-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/experiences').catch(() => [] as DictItem[]),
)
const { data: salaries } = await useAsyncData(
  () => `dict-salary-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/salaries').catch(() => [] as DictItem[]),
)
const { data: industries } = await useAsyncData(
  () => `dict-hy-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/industries').catch(() => [] as DictItem[]),
)
const { data: welfares } = await useAsyncData(
  () => `dict-welfare-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/welfares').catch(() => [] as DictItem[]),
)
const { data: reports } = await useAsyncData(
  () => `dict-report-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/reports').catch(() => [] as DictItem[]),
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
const { data: adsTop } = await useAsyncData('ads-507', () =>
  api.get<Array<{ image_n?: string; html?: string }>>('/v1/wap/ads', { slot: '507', limit: 1 }).catch(() => []),
)
const { data: adsSide } = await useAsyncData('ads-7', () =>
  api.get<Array<{ image_n?: string; html?: string }>>('/v1/wap/ads', { slot: '7', limit: 5 }).catch(() => []),
)

const jobItems = computed(() => jobRoots.value.map((c) => ({ id: c.id, name: c.name })))
const job2Items = computed(() => jobLevel2.value.map((c) => ({ id: c.id, name: c.name })))
const job3Items = computed(() => jobLevel3.value.map((c) => ({ id: c.id, name: c.name })))
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
const dictName = (items: DictItem[] | null | undefined, id?: number) =>
  items?.find((x) => x.id === id)?.name || ''
const uptimeItems = computed<DictItem[]>(() => [
  { id: 1, name: t('common_01940') },
  { id: 3, name: t('wap_00432') },
  { id: 7, name: t('wap_00433') },
  { id: 30, name: t('admin_user_00175') },
  { id: 90, name: t('wap_00431') },
])
const sexItems = computed<DictItem[]>(() => [
  { id: 1, name: t('common_02092') },
  { id: 2, name: t('common_02069') },
])
const { data: recSide } = await useAsyncData(
  () => `jobs-rec-side-${locale.value}`,
  () => api.get<{ list: JobLike[] }>('/v1/wap/jobs', { rec: true, page_size: 30 }).catch(() => ({ list: [] as JobLike[] })),
)
const recBatch = ref(0)
const recVisible = computed(() => {
  const all = recSide.value?.list || []
  if (all.length <= 10) return all
  const start = (recBatch.value * 10) % all.length
  const out: JobLike[] = []
  for (let i = 0; i < 10; i++) out.push(all[(start + i) % all.length])
  return out
})
function exchangeRec() {
  const n = recSide.value?.list?.length || 0
  if (n > 10) recBatch.value += 1
}
const { data: bidJobs } = await useAsyncData(
  () =>
    `jobs-bid-${locale.value}-${page.value}-${keyword.value}-${job1.value}-${job1Son.value}-${jobPost.value}-${provinceId.value}-${cityId.value}-${threeCityId.value}-${edu.value}-${exp.value}-${salaryId.value}-${hy.value}-${welfare.value}-${report.value}-${uptime.value}-${sex.value}-${urgent.value}-${rec.value}-${cert.value}`,
  () =>
    page.value > 1
      ? Promise.resolve({ list: [] as JobLike[] })
      : api
          .get<{ list: JobLike[] }>('/v1/wap/jobs', {
            page: 1,
            page_size: 20,
            keyword: keyword.value || undefined,
            job1: job1.value,
            job1_son: job1Son.value,
            job_post: jobPost.value,
            province_id: provinceId.value,
            city_id: cityId.value,
            three_city_id: threeCityId.value,
            edu: edu.value,
            exp: exp.value,
            hy: hy.value,
            welfare: welfare.value,
            report: report.value,
            uptime: uptime.value,
            sex: sex.value,
            min_salary: salaryBound.value?.min_salary,
            max_salary: salaryBound.value?.max_salary,
            urgent: urgent.value ? true : undefined,
            rec: rec.value ? true : undefined,
            cert: cert.value ? true : undefined,
            bid: true,
          })
          .catch(() => ({ list: [] as JobLike[] })),
)
const bidList = computed(() => (page.value > 1 ? [] : bidJobs.value?.list || []))
const list = computed(() => {
  const ids = new Set(bidList.value.map((j) => j.id))
  return ids.size ? listRaw.value.filter((j) => !ids.has(j.id)) : listRaw.value
})

const selected = computed(() => {
  const rows: Array<{ param: string; name: string; extra?: Record<string, undefined> }> = []
  if (keyword.value) rows.push({ param: 'keyword', name: `${t('admin_tool_00574')}：${keyword.value}` })
  if (job1.value) {
    const n = dictName(jobItems.value, job1.value)
    if (n) rows.push({ param: 'job1', name: `${t('admin_user_company_00377')}：${n}`, extra: { job1_son: undefined, job_post: undefined } })
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
    if (n) rows.push({ param: 'province_id', name: `${t('member_user_00198')}：${n}`, extra: { city_id: undefined, three_city_id: undefined } })
  }
  if (cityId.value) {
    const n = dictName(cities.value, cityId.value)
    if (n) rows.push({ param: 'city_id', name: n, extra: { three_city_id: undefined } })
  }
  if (threeCityId.value) {
    const n = dictName(districts.value, threeCityId.value)
    if (n) rows.push({ param: 'three_city_id', name: n })
  }
  const hyN = dictName(industries.value, hy.value)
  if (hyN) rows.push({ param: 'hy', name: `${t('admin_user_company_00373')}：${hyN}` })
  const eduN = dictName(edus.value, edu.value)
  if (eduN) rows.push({ param: 'edu', name: `${t('home.education_suffix')}：${eduN}` })
  const expN = dictName(exps.value, exp.value)
  if (expN) rows.push({ param: 'exp', name: `${t('wap_user_00240')}：${expN}` })
  const salaryN = dictName(salaries.value, salaryId.value)
  if (salaryN) rows.push({ param: 'salary', name: `${t('member_user_00106')}：${salaryN}` })
  const welN = dictName(welfares.value, welfare.value)
  if (welN) rows.push({ param: 'welfare', name: welN })
  const reportN = dictName(reports.value, report.value)
  if (reportN) rows.push({ param: 'report', name: `${t('wap_com_00279')}：${reportN}` })
  const upN = dictName(uptimeItems.value, uptime.value)
  if (upN) rows.push({ param: 'uptime', name: `${t('wap_00326')}：${upN}` })
  const sexN = dictName(sexItems.value, sex.value)
  if (sexN) rows.push({ param: 'sex', name: `${t('wap_com_00303')}：${sexN}` })
  if (cert.value) rows.push({ param: 'cert', name: t('common_02393') })
  return rows
})

useSeoMeta({ title: keyword.value ? `${keyword.value} - ${t('common.job')}` : t('default_00246') })

const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))

function goPage(p: number) {
  return navigateTo({ query: { ...route.query, page: p } })
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
            <span>{{ $t('default_00246') }}</span>
          </div>
        </div>
        <div class="clear" />
        <div v-if="adsTop?.length" class="yun_jobbanner">
          <img v-for="(ad, i) in adsTop" :key="i" :src="ad.image_n" alt="" />
        </div>
        <div class="clear" />
        <form action="/jobs" method="get">
          <div class="jobsearch_newbox">
            <div class="yun_job_search">
              <div class="yun_job_search_cont searchContButton">
                <div class="yun_job_search_textcont">
                  <input class="Search_jobs_text" name="keyword" :value="keyword" :placeholder="$t('default_00348')" />
                </div>
                <input class="Search_jobs_submit yun_bg_color jobsSubmit" type="submit" :value="$t('common.search')" />
              </div>
              <div v-if="hotSearches?.length" class="jobs_tag">
                {{ $t('wap_00385') }}：
                <NuxtLink
                  v-for="k in hotSearches"
                  :key="k.keyword"
                  :to="`/jobs?keyword=${encodeURIComponent(k.keyword)}`"
                  class="jos_tag_a"
                >{{ k.keyword }}</NuxtLink>
              </div>
            </div>
          </div>
          <div class="clear" />
          <input v-for="[k, v] in hiddenFilters" :key="k" type="hidden" :name="k" :value="v" />
          <div class="Search_jobs_box">
          <FilterRow
            v-if="!job1"
            :label="$t('common.job')"
            param="job1"
            :items="jobItems"
            :current="job1"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="job1 && !job1Son"
            :label="$t('common_01972')"
            param="job1_son"
            :items="job2Items"
            :current="job1Son"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="job1Son"
            :label="$t('admin_00223')"
            param="job_post"
            :items="job3Items"
            :current="jobPost"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <CityFilterBox
            :label="$t('member_com_00378')"
            path="/jobs"
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
            extra-class="search_more"
            :label="$t('member_user_00106')"
            param="salary"
            :items="salaries || []"
            :current="salaryId"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <div class="searchmorelist">
            <div class="Search_jobs_form_list search_more">
              <div class="Search_jobs_name">{{ $t('common.more') }}：</div>
              <div class="Search_jobs_sub" style="width: 1090px">
                <MoreFilterSelect
                  v-if="(welfares || []).length"
                  :label="$t('wap_com_00167')"
                  param="welfare"
                  :items="welfares || []"
                  :current="welfare"
                  path="/jobs"
                  :all-label="$t('common.all')"
                />
                <MoreFilterSelect
                  :label="$t('wap_com_00283')"
                  param="edu"
                  :items="edus || []"
                  :current="edu"
                  path="/jobs"
                  :all-label="$t('common.all')"
                />
                <MoreFilterSelect
                  :label="$t('wap_com_00287')"
                  param="exp"
                  :items="exps || []"
                  :current="exp"
                  path="/jobs"
                  :all-label="$t('common.all')"
                />
                <MoreFilterSelect
                  v-if="sexSwitch"
                  :label="$t('wap_com_00332')"
                  param="sex"
                  :items="sexItems"
                  :current="sex"
                  path="/jobs"
                  :all-label="$t('common.all')"
                />
                <MoreFilterSelect
                  v-if="(reports || []).length"
                  :label="$t('wap_com_00279')"
                  param="report"
                  :items="reports || []"
                  :current="report"
                  path="/jobs"
                  :all-label="$t('common.all')"
                />
                <MoreFilterSelect
                  :label="$t('default_00360')"
                  param="hy"
                  :items="industries || []"
                  :current="hy"
                  path="/jobs"
                  :all-label="$t('common.all')"
                  wide
                />
                <MoreFilterSelect
                  :label="$t('wap_00326')"
                  param="uptime"
                  :items="uptimeItems"
                  :current="uptime"
                  path="/jobs"
                  :all-label="$t('common.all')"
                />
              </div>
            </div>
          </div>
          <div v-if="selected.length" class="Search_close_box">
            <div>
              <div class="Search_clear">
                <NuxtLink to="/jobs">{{ $t('default_00059') }}</NuxtLink>
              </div>
              <span class="Search_close_box_s">{{ $t('default_00058') }}</span>
            </div>
            <NuxtLink
              v-for="s in selected"
              :key="s.param"
              :to="{ path: '/jobs', query: mergeQuery(route.query, { [s.param]: undefined, ...(s.extra || {}) }) }"
              class="Search_jobs_c_a disc_fac"
            >{{ s.name }}</NuxtLink>
          </div>
          <div class="clear" />
        </div>
        </form>
        <div class="search_h1_box" style="overflow: hidden; position: relative">
          <div class="search_h1_box_title">
            <ul class="search_h1_box_list">
              <li :class="{ search_job_all: !urgent && !rec && !order && !cert }">
                <NuxtLink :to="{ path: '/jobs', query: mergeQuery(route.query, { urgent: undefined, rec: undefined, order: undefined, cert: undefined }) }">{{
                  $t('common_02394')
                }}</NuxtLink>
                <i class="search_h1_box_list_icon" />
              </li>
              <li :class="{ search_Filter_current: order === 'lastdate' }">
                <NuxtLink :to="{ path: '/jobs', query: mergeQuery(route.query, { order: order === 'lastdate' ? undefined : 'lastdate' }) }">
                  <span>{{ $t('wap_00326') }}</span><i class="search_Filter_icon" />
                </NuxtLink>
              </li>
              <li :class="{ search_Filter_current: order === 'sdate' }">
                <NuxtLink :to="{ path: '/jobs', query: mergeQuery(route.query, { order: order === 'sdate' ? undefined : 'sdate' }) }">
                  <span>{{ $t('admin_user_weipin_00030') }}</span><i class="search_Filter_icon" />
                </NuxtLink>
              </li>
              <li :class="{ search_h1_box_cur: urgent }" class="job_jp_t">
                <NuxtLink
                  :to="{ path: '/jobs', query: mergeQuery(route.query, { urgent: urgent ? undefined : '1' }) }"
                  class="job_zt"
                >
                  {{ $t('member_com_00326') }} <i class="job_jp_chk" />
                </NuxtLink>
              </li>
              <li :class="{ search_h1_box_cur: rec }" class="job_tj_t">
                <NuxtLink
                  :to="{ path: '/jobs', query: mergeQuery(route.query, { rec: rec ? undefined : '1' }) }"
                  class="job_zt"
                >
                  {{ $t('home.recommended_jobs') }} <i class="job_tj_chk" />
                </NuxtLink>
              </li>
              <li :class="{ search_h1_box_cur: cert }">
                <NuxtLink
                  :to="{ path: '/jobs', query: mergeQuery(route.query, { cert: cert ? undefined : '1' }) }"
                  class="job_zt"
                >
                  <i class="job_tj_chk" /><em>{{ $t('common_02395') }}</em>
                </NuxtLink>
              </li>
            </ul>
            <div v-if="freeTel" class="search_h1_box_t fr">{{ $t('default_00363') }}{{ freeTel }}</div>
          </div>
        </div>
        <div class="left_job_all fl">
          <div class="job_left_sidebar">
            <p v-if="error" class="muted" style="padding: 30px 0">{{ failMsg }}</p>
            <template v-else>
              <JobCard v-for="job in bidList" :key="'bid-' + job.id" :job="job" variant="search" />
              <JobCard v-for="job in list" :key="job.id" :job="job" variant="search" />
              <template v-if="!list.length && !bidList.length">
                <EmptyState
                  :title="$t('default_00362')"
                  :hint="$t('common_02399')"
                />
                <JobCard
                  v-for="job in recVisible"
                  :key="'rec-' + job.id"
                  :job="job"
                  variant="search"
                />
              </template>
            </template>
            <Pager :page="page" :page-size="20" :total="data?.total || 0" @update:page="goPage" />
          </div>
        </div>
        <div v-if="(adsSide && adsSide.length) || recVisible.length" class="yun_job_list_right">
          <div v-if="adsSide?.length" class="yun_job_list_right_banner">
            <img v-for="(ad, i) in adsSide" :key="i" :src="ad.image_n" alt="" />
          </div>
          <div v-if="recVisible.length" class="job_recommendation">
            <div class="job_recommendation_title">
              <span class="job_recommendation_span"><i class="job_recommendation_span_line" />{{ $t('default_00249') }}</span>
              <a
                v-if="(recSide?.list || []).length > 10"
                href="javascript:;"
                class="job_right_box_more png"
                @click.prevent="exchangeRec"
              >{{ $t('common_02400') }}</a>
            </div>
            <ul class="job_recommendation_list">
              <li v-for="job in recVisible" :key="job.id">
                <NuxtLink :to="`/jobs/${job.id}`" class="job_recommendation_jobname">{{ job.name }}</NuxtLink>
                <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`" class="job_recommendation_Comname">{{ job.com_name }}</NuxtLink>
                <div class="job_recommendation_msg">
                  <span><em class="job_right_box_list_c">{{ formatSalary(job, $t('common.negotiable'), Number(settings.resume_salarytype || 1), $t('common_01943')) }}</em></span>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="site-h5">
    <div class="job_header" style="position: relative">
      <form class="job_header_center" action="/jobs" method="get" style="padding: 0.16rem 0.32rem">
        <input
          class="searchnew"
          name="keyword"
          :value="keyword"
          :placeholder="$t('wap_user_00254')"
          style="width: 100%; height: 0.8rem; border-radius: 0.4rem; padding: 0 0.32rem; background: #f5f5f5"
        />
      </form>
    </div>
    <div class="job_header_nav resumeAdeFlex">
      <div class="job_header_nav_left category">
        <ul>
          <li :class="{ active: !urgent }">
            <NuxtLink :to="{ path: '/jobs', query: mergeQuery(route.query, { urgent: undefined }) }">{{
              $t('common.latest')
            }}</NuxtLink>
          </li>
          <li :class="{ active: urgent }">
            <NuxtLink :to="{ path: '/jobs', query: mergeQuery(route.query, { urgent: '1' }) }">{{
              $t('wap_com_00250')
            }}</NuxtLink>
          </li>
          <li>
            <NuxtLink to="/map">{{ $t('wap_00223') }}</NuxtLink>
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
            label: $t('wap_00576'),
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
              { label: $t('wap_user_00016'), param: 'salary', items: salaries || [] },
              { label: $t('company_00007'), param: 'welfare', items: welfares || [] },
              { label: $t('home.experience_suffix'), param: 'exp', items: exps || [] },
              { label: $t('home.education_suffix'), param: 'edu', items: edus || [] },
              { label: $t('wap_com_00303'), param: 'sex', items: sexItems },
              { label: $t('wap_00326'), param: 'uptime', items: uptimeItems },
              { label: $t('admin_user_company_00373'), param: 'hy', items: industries || [] },
            ],
          },
        ]"
      />
    </div>
    <div class="main_part" style="padding-top: 0.2rem">
      <div v-if="adsTop?.length" class="jobzd_banner">
        <img v-for="(ad, i) in adsTop" :key="i" :src="ad.image_n" alt="" style="width: 100%" />
      </div>
      <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
      <template v-else>
        <JobCard v-for="job in bidList" :key="'bid-' + job.id" :job="job" variant="search" />
        <JobCard v-for="job in list" :key="job.id" :job="job" variant="search" />
        <template v-if="!list.length && !bidList.length">
          <EmptyState :title="$t('home.no_job_data')" />
          <JobCard
            v-for="job in recVisible"
            :key="'rec-' + job.id"
            :job="job"
            variant="search"
          />
        </template>
      </template>
      <Pager :page="page" :page-size="20" :total="data?.total || 0" @update:page="goPage" />
    </div>
  </div>
</template>
