<script setup lang="ts">
import { formatSalary, type CompanyLike, type JobLike } from '~/utils/site'

type ResumeRow = { uid: number; display_name?: string; expect_name?: string; lastupdate_n?: string }
type ArticleRow = { id: number; title: string; hits?: number }
type HotRow = { keyword: string; hits?: number; scope?: string }

const { t } = useI18n()
const api = useApi()
const { siteName } = useSiteChrome()

async function takeList<T>(p: Promise<{ list?: T[] } | T[] | null | undefined>): Promise<T[]> {
  const r = await p.catch(() => null)
  if (!r) return []
  if (Array.isArray(r)) return r
  return r.list || []
}

const { data } = await useAsyncData('site-rank-top', async () => {
  const [recJobs, companies, latestJobs, resumes, keywords, articles, urgentJobs] = await Promise.all([
    takeList<JobLike>(api.get('/v1/wap/jobs', { rec: true, page: 1, page_size: 10 })),
    takeList<CompanyLike>(api.get('/v1/wap/companies', { page: 1, page_size: 10 })),
    takeList<JobLike>(api.get('/v1/wap/jobs', { page: 1, page_size: 10 })),
    takeList<ResumeRow>(api.get('/v1/wap/resumes', { page: 1, page_size: 10 })),
    takeList<HotRow>(api.get('/v1/wap/hot-searches', { scope: 'job', limit: 10 })),
    takeList<ArticleRow>(api.get('/v1/wap/articles', { page: 1, page_size: 20 })),
    takeList<JobLike>(api.get('/v1/wap/jobs', { urgent: true, page: 1, page_size: 10 })),
  ])
  const articleRank = [...articles].sort((a, b) => Number(b.hits || 0) - Number(a.hits || 0)).slice(0, 10)
  return { recJobs, companies, latestJobs, resumes, keywords, articles: articleRank, urgentJobs }
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
          <NuxtLink :to="{ path: '/jobs', query: { keyword: row.keyword } }" style="width: 110px">
            {{ row.keyword }}
          </NuxtLink>
          <em>{{ row.hits }}</em>
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
