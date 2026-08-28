<script setup lang="ts">
import { catTree, listFailMsg, type CatNode } from '~/utils/site'
import type { DictItem } from '~/utils/query'

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
const api = useApi()
const { data, error } = await useAsyncData(
  () =>
    `resumes-${locale.value}-${page.value}-${keyword.value}-${education.value}-${exp.value}-${job1.value}-${job1Son.value}-${jobPost.value}-${provinceId.value}-${cityId.value}`,
  () =>
    api.get<{ list: Array<Record<string, unknown>>; total: number }>('/v1/wap/resumes', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
      education: education.value,
      exp: exp.value,
      job1: job1.value || jobPost.value || job1Son.value,
      province_id: provinceId.value,
      city_id: cityId.value,
    }),
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
useSeoMeta({ title: t('common.resume') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
const jobLabel = computed(() => {
  const hit = [...jobItems.value, ...job2Items.value, ...job3Items.value].find(
    (c) => c.id === jobPost.value || c.id === job1Son.value || c.id === job1.value,
  )
  return hit?.name || ''
})
const cityLabel = computed(() => {
  const hit = [...(provinces.value || []), ...(cities.value || [])].find(
    (c) => c.id === cityId.value || c.id === provinceId.value,
  )
  return hit?.name || ''
})
</script>

<template>
  <div class="site-pc">
    <div class="yun_jobbody">
      <div class="yun_content">
        <div class="current_Location com_current_Location png">
          <div class="fl">
            {{ $t('common_01498') }}：
            <NuxtLink to="/">{{ $t('common.home') }}</NuxtLink> >
            <span>{{ $t('common.resume') }}</span>
          </div>
        </div>
        <form action="/resumes" method="get" class="jobsearch_newbox">
          <div class="yun_job_search">
            <div class="yun_job_search_cont searchContButton">
              <div class="yun_job_search_textcont">
                <input class="Search_jobs_text" name="keyword" :value="keyword" :placeholder="$t('common.search')" />
              </div>
              <input class="Search_jobs_submit yun_bg_color jobsSubmit" type="submit" :value="$t('common.search')" />
            </div>
          </div>
        </form>
        <div class="Search_jobs_box">
          <FilterRow
            :label="$t('common.job')"
            param="job1"
            :items="jobItems"
            :current="job1"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="job1 && job2Items.length"
            :label="$t('common_01972')"
            param="job1_son"
            :items="job2Items"
            :current="job1Son"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="job1Son && job3Items.length"
            :label="$t('admin_00223')"
            param="job_post"
            :items="job3Items"
            :current="jobPost"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <FilterRow
            :label="$t('member_com_00378')"
            param="province_id"
            :items="provinces || []"
            :current="provinceId"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="provinceId && (cities || []).length"
            :label="$t('common_02110')"
            param="city_id"
            :items="cities || []"
            :current="cityId"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <FilterRow
            :label="$t('wap_com_00283')"
            param="education"
            :items="edus || []"
            :current="education"
            path="/resumes"
            :all-label="$t('common.all')"
          />
          <FilterRow
            :label="$t('wap_user_00240')"
            param="exp"
            :items="exps || []"
            :current="exp"
            path="/resumes"
            :all-label="$t('common.all')"
          />
        </div>
        <p v-if="error" class="muted">{{ failMsg }}</p>
        <template v-else>
          <ResumeCard v-for="r in list" :key="String(r.uid)" :row="r" />
          <EmptyState v-if="!list.length" />
        </template>
        <Pager
          :page="page"
          :page-size="20"
          :total="data?.total || 0"
          @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
        />
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
          <li class="active">
            <NuxtLink to="/resumes">{{ $t('common.latest') }}</NuxtLink>
          </li>
        </ul>
      </div>
      <H5FilterBar
        :all-label="$t('common.all')"
        :tabs="[
          { key: 'province_id', label: $t('common_02110'), current: cityLabel, items: provinces || [] },
          { key: 'job1', label: $t('wap_com_00021'), current: jobLabel, items: jobItems },
          { key: 'education', label: $t('wap_00238'), items: edus || [] },
        ]"
      />
    </div>
    <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
    <template v-else>
      <ResumeCard v-for="r in list" :key="String(r.uid)" :row="r" />
      <EmptyState v-if="!list.length" />
    </template>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </section>
</template>
