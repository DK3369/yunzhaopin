<script setup lang="ts">
import { formatSalary, type CompanyLike, type JobLike } from '~/utils/site'

type ResumeRow = { uid: number; display_name?: string; expect_name?: string; lastupdate_n?: string }
type ArticleRow = { id: number; title: string; hits?: number }
type HotRow = { keyword: string; hits?: number; num?: number; type_name?: string; to?: string }

type RankingsData = {
  rec_jobs?: JobLike[]
  companies?: CompanyLike[]
  latest_jobs?: JobLike[]
  resumes?: ResumeRow[]
  keywords?: HotRow[]
  articles?: ArticleRow[]
  urgent_jobs?: JobLike[]
}

const { t } = useI18n()
const api = useApi()
const { siteName } = useSiteChrome()
const { applyToQuery } = useSubSite()

const emptyRankings: RankingsData = {
  rec_jobs: [],
  companies: [],
  latest_jobs: [],
  resumes: [],
  keywords: [],
  articles: [],
  urgent_jobs: [],
}

const { data } = await useAsyncData('site-rank-top', async () => {
  const row = await api.get<RankingsData>('/v1/wap/rankings', applyToQuery({})).catch(() => emptyRankings)
  return {
    recJobs: row.rec_jobs || [],
    companies: row.companies || [],
    latestJobs: row.latest_jobs || [],
    resumes: row.resumes || [],
    keywords: row.keywords || [],
    articles: row.articles || [],
    urgentJobs: row.urgent_jobs || [],
  }
})

function jobPay(job: JobLike) {
  const s = formatSalary(job, t('common.negotiable'))
  if (!s || s === t('common.negotiable')) return s
  return `￥${s}`
}

useSeoMeta({ title: () => t('default_00156') })
useHead({ link: [{ rel: 'canonical', href: '/top' }] })
</script>

<template>
  <div class="yun_content">
    <div class="current_Location icon">
      {{ $t('common_02137') }}：
      <NuxtLink to="/">{{ siteName || $t('common.home') }}</NuxtLink>
      &gt; {{ $t('default_00156') }}
    </div>
    <div class="clear" />

    <div class="post_Top_box fr">
      <div class="post_Top_box_title Top_box_line2">
        <strong>{{ $t('home.recommended_jobs') }}</strong>
      </div>
      <ul class="post_Top_box_list">
        <li v-for="(row, i) in data?.recJobs || []" :key="row.id">
          <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
          <NuxtLink :to="`/jobs/${row.id}`" :title="row.name" style="width: 110px">{{ row.name }}</NuxtLink>
          <u class="fl">{{ row.job_city_one }}</u>
          <em>{{ jobPay(row) }}</em>
        </li>
      </ul>
    </div>

    <div class="post_Top_box fr" style="margin-right: 0">
      <div class="post_Top_box_title Top_box_line">
        <strong>{{ $t('default_00155') }}</strong>
      </div>
      <ul class="post_Top_box_list">
        <li v-for="(row, i) in data?.companies || []" :key="row.uid">
          <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
          <NuxtLink :to="`/companies/${row.uid}`" :title="row.name || row.shortname || ''">
            {{ row.name || row.shortname }}
          </NuxtLink>
          <em>{{ row.lastupdate_n }}</em>
        </li>
      </ul>
    </div>

    <div class="post_Top_box">
      <div class="post_Top_box_title Top_box_line">
        <strong>{{ $t('home.latest_jobs') }}</strong>
      </div>
      <ul class="post_Top_box_list">
        <li v-for="(row, i) in data?.latestJobs || []" :key="row.id">
          <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
          <NuxtLink :to="`/jobs/${row.id}`" :title="row.name" style="width: 115px">{{ row.name }}</NuxtLink>
          <u class="fl">{{ [row.job_city_one, row.job_city_two].filter(Boolean).join('-') }}</u>
          <em>{{ row.lastupdate_n }}</em>
        </li>
      </ul>
    </div>

    <div class="post_Top_box">
      <div class="post_Top_box_title Top_box_line2">
        <strong>{{ $t('home.latest_talents') }}</strong>
      </div>
      <ul class="post_Top_box_list">
        <li v-for="(row, i) in data?.resumes || []" :key="row.uid">
          <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
          <NuxtLink :to="`/resumes/${row.uid}`" style="width: 100px">{{ row.display_name }}</NuxtLink>
          <u class="fl">{{ row.expect_name }}</u>
          <em>{{ row.lastupdate_n }}</em>
        </li>
      </ul>
    </div>

    <div class="post_Top_box" style="margin-right: 0">
      <div class="post_Top_box_title Top_box_line3">
        <strong>{{ $t('admin_system_00443') }}</strong>
      </div>
      <ul class="post_Top_box_list">
        <li v-for="(row, i) in data?.keywords || []" :key="`${row.keyword}-${i}`">
          <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
          <NuxtLink :to="{ path: row.to || '/jobs', query: { keyword: row.keyword } }" style="width: 110px">
            {{ row.keyword }}
          </NuxtLink>
          <u class="fl">{{ row.type_name }}</u>
          <em>{{ row.num ?? row.hits }}</em>
        </li>
      </ul>
    </div>

    <div class="post_Top_box">
      <div class="post_Top_box_title Top_box_line3">
        <strong>{{ $t('default_00150') }}</strong>
      </div>
      <ul class="post_Top_box_list">
        <li v-for="(row, i) in data?.articles || []" :key="row.id">
          <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
          <NuxtLink :to="`/articles/${row.id}`">{{ row.title }}</NuxtLink>
          <em>{{ row.hits }}</em>
        </li>
      </ul>
    </div>

    <div class="post_Top_box">
      <div class="post_Top_box_title Top_box_line">
        <strong>{{ $t('member_com_00326') }}</strong>
      </div>
      <ul class="post_Top_box_list">
        <li v-for="(row, i) in data?.urgentJobs || []" :key="row.id">
          <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
          <NuxtLink :to="`/jobs/${row.id}`" :title="row.name" style="width: 110px">{{ row.name }}</NuxtLink>
          <u class="fl">{{ row.job_city_one }}</u>
          <em>{{ jobPay(row) }}</em>
        </li>
      </ul>
    </div>
    <div class="clear" />
  </div>
</template>
