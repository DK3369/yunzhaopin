<script setup lang="ts">
import type { JobLike, CatNode } from '~/utils/site'
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
const edu = computed(() => numQuery(route.query.edu))
const exp = computed(() => numQuery(route.query.exp))
const salaryId = computed(() => numQuery(route.query.salary))
const urgent = computed(() => route.query.urgent === '1')
const salaryBound = computed(() => (salaryId.value ? SALARY_BOUNDS[salaryId.value] : undefined))
const api = useApi()

const { data } = await useAsyncData(
  () =>
    `jobs-${locale.value}-${page.value}-${keyword.value}-${job1.value}-${job1Son.value}-${jobPost.value}-${provinceId.value}-${cityId.value}-${edu.value}-${exp.value}-${salaryId.value}-${urgent.value}`,
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
      edu: edu.value,
      exp: exp.value,
      min_salary: salaryBound.value?.min_salary,
      max_salary: salaryBound.value?.max_salary,
      urgent: urgent.value ? true : undefined,
    }),
)
const list = computed(() => data.value?.list || [])

const { data: cats } = await useAsyncData(
  () => `job-cats-${locale.value}`,
  () => api.get<CatNode[]>('/v1/wap/categories', { kind: 'job' }).catch(() => [] as CatNode[]),
)
const jobRoots = computed(() => catTree(cats.value || [], 40))
const jobLevel2 = computed(() => jobRoots.value.find((c) => c.id === job1.value)?.children || [])
const jobLevel3 = computed(() => jobLevel2.value.find((c) => c.id === job1Son.value)?.children || [])

const { data: provinces } = await useAsyncData(
  () => `regions-cn-1-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/regions', { country: 'CN', level: 1 }).catch(() => [] as DictItem[]),
)
const { data: cities } = await useAsyncData(
  () => `regions-child-${locale.value}-${provinceId.value || 0}`,
  () =>
    provinceId.value
      ? api.get<DictItem[]>('/v1/wap/regions/children', { id: provinceId.value }).catch(() => [] as DictItem[])
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
const { data: ads } = await useAsyncData('ads-504', () =>
  api.get<Array<{ image_n?: string; html?: string }>>('/v1/wap/ads', { slot: '504', limit: 3 }).catch(() => []),
)

const jobItems = computed(() => jobRoots.value.map((c) => ({ id: c.id, name: c.name })))
const job2Items = computed(() => jobLevel2.value.map((c) => ({ id: c.id, name: c.name })))
const job3Items = computed(() => jobLevel3.value.map((c) => ({ id: c.id, name: c.name })))
const cityLabel = computed(() => {
  const hit = [...(provinces.value || []), ...(cities.value || [])].find(
    (c) => c.id === cityId.value || c.id === provinceId.value,
  )
  return hit?.name || ''
})
const jobLabel = computed(() => {
  const hit = [...jobItems.value, ...job2Items.value, ...job3Items.value].find(
    (c) => c.id === jobPost.value || c.id === job1Son.value || c.id === job1.value,
  )
  return hit?.name || ''
})

useSeoMeta({ title: keyword.value ? `${keyword.value} - ${t('common.job')}` : t('default_00246') })

function goPage(p: number) {
  return navigateTo({ query: { ...route.query, page: p } })
}
</script>

<template>
  <div class="site-pc">
    <div class="yun_jobbody">
      <div class="yun_content">
        <div class="current_Location com_current_Location png">
          <div class="fl">
            {{ $t('common_01498') }}：
            <NuxtLink to="/">{{ $t('common.home') }}</NuxtLink> >
            <span>{{ $t('default_00246') }}</span>
          </div>
        </div>
        <div v-if="ads?.length" class="yun_jobbanner">
          <img v-for="(ad, i) in ads" :key="i" :src="ad.image_n" alt="" />
        </div>
        <form action="/jobs" method="get" class="jobsearch_newbox">
          <div class="yun_job_search">
            <div class="yun_job_search_cont searchContButton">
              <div class="yun_job_search_textcont">
                <input class="Search_jobs_text" name="keyword" :value="keyword" :placeholder="$t('default_00348')" />
              </div>
              <input class="Search_jobs_submit yun_bg_color jobsSubmit" type="submit" :value="$t('common.search')" />
            </div>
          </div>
        </form>
        <p>
          <NuxtLink :to="{ path: '/jobs', query: mergeQuery(route.query, { urgent: undefined }) }" :class="{ Search_jobs_sub_cur: !urgent }">{{
            $t('common.latest')
          }}</NuxtLink>
          ·
          <NuxtLink :to="{ path: '/jobs', query: mergeQuery(route.query, { urgent: '1' }) }" :class="{ Search_jobs_sub_cur: urgent }">{{
            $t('wap_com_00250')
          }}</NuxtLink>
        </p>
        <div class="Search_jobs_box">
          <FilterRow
            :label="$t('common.job')"
            param="job1"
            :items="jobItems"
            :current="job1"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="job1 && job2Items.length"
            :label="$t('common_01972')"
            param="job1_son"
            :items="job2Items"
            :current="job1Son"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="job1Son && job3Items.length"
            :label="$t('admin_00223')"
            param="job_post"
            :items="job3Items"
            :current="jobPost"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            :label="$t('member_com_00378')"
            param="province_id"
            :items="provinces || []"
            :current="provinceId"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="provinceId && (cities || []).length"
            :label="$t('common_02110')"
            param="city_id"
            :items="cities || []"
            :current="cityId"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            :label="$t('home.education_suffix')"
            param="edu"
            :items="edus || []"
            :current="edu"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            :label="$t('home.experience_suffix')"
            param="exp"
            :items="exps || []"
            :current="exp"
            path="/jobs"
            :all-label="$t('common.all')"
          />
          <FilterRow
            :label="$t('common.negotiable')"
            param="salary"
            :items="salaries || []"
            :current="salaryId"
            path="/jobs"
            :all-label="$t('common.all')"
          />
        </div>
        <div class="index_newjobbox index_zw_item">
          <ul>
            <JobCard v-for="job in list" :key="job.id" :job="job" />
          </ul>
          <p v-if="!list.length" class="muted" style="padding: 30px 0">{{ $t('home.no_job_data') }}</p>
        </div>
        <Pager :page="page" :page-size="20" :total="data?.total || 0" @update:page="goPage" />
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
          { key: 'province_id', label: $t('common_02110'), current: cityLabel, items: provinces || [] },
          { key: 'job1', label: $t('wap_00576'), current: jobLabel, items: jobItems },
          { key: 'edu', label: $t('wap_00238'), current: '', items: edus || [] },
        ]"
      />
    </div>
    <div class="main_part" style="padding-top: 0.2rem">
      <div v-if="ads?.length" class="jobzd_banner">
        <img v-for="(ad, i) in ads" :key="i" :src="ad.image_n" alt="" style="width: 100%" />
      </div>
      <JobCard v-for="job in list" :key="job.id" :job="job" />
      <p v-if="!list.length" class="muted" style="padding: 0.4rem">{{ $t('home.no_job_data') }}</p>
      <Pager :page="page" :page-size="20" :total="data?.total || 0" @update:page="goPage" />
    </div>
  </div>
</template>
