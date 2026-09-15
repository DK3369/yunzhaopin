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

const { t, locale } = useI18n()
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

const { data } = await useAsyncData(
  () => `site-rank-top-${locale.value}`,
  async () => {
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
  },
)

function jobPay(job: JobLike) {
  const s = formatSalary(job, t('common.negotiable'))
  if (!s || s === t('common.negotiable')) return s
  return `￥${s}`
}

useSeoMeta({ title: () => t('default_00156') })
useHead({ link: [{ rel: 'canonical', href: '/top' }] })
</script>

<template>
  <div class="page-top">
    <div class="current_Location">
      {{ $t('common_02137') }}：
      <NuxtLink to="/">{{ siteName || $t('common.home') }}</NuxtLink>
      &gt; {{ $t('default_00156') }}
    </div>

    <div class="page-top__grid">
      <div class="post_Top_box">
        <div class="post_Top_box_title Top_box_line2">
          <strong>{{ $t('home.recommended_jobs') }}</strong>
        </div>
        <ul class="post_Top_box_list">
          <li v-if="!(data?.recJobs || []).length" class="page-top__empty">{{ $t('common_02409') }}</li>
          <li v-for="(row, i) in data?.recJobs || []" :key="row.id">
            <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
            <NuxtLink :to="`/jobs/${row.id}`" :title="row.name">{{ row.name }}</NuxtLink>
            <u>{{ row.job_city_one }}</u>
            <em>{{ jobPay(row) }}</em>
          </li>
        </ul>
      </div>

      <div class="post_Top_box">
        <div class="post_Top_box_title Top_box_line">
          <strong>{{ $t('default_00155') }}</strong>
        </div>
        <ul class="post_Top_box_list">
          <li v-if="!(data?.companies || []).length" class="page-top__empty">{{ $t('common_02409') }}</li>
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
          <li v-if="!(data?.latestJobs || []).length" class="page-top__empty">{{ $t('common_02409') }}</li>
          <li v-for="(row, i) in data?.latestJobs || []" :key="row.id">
            <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
            <NuxtLink :to="`/jobs/${row.id}`" :title="row.name">{{ row.name }}</NuxtLink>
            <u>{{ [row.job_city_one, row.job_city_two].filter(Boolean).join('-') }}</u>
            <em>{{ row.lastupdate_n }}</em>
          </li>
        </ul>
      </div>

      <div class="post_Top_box">
        <div class="post_Top_box_title Top_box_line2">
          <strong>{{ $t('home.latest_talents') }}</strong>
        </div>
        <ul class="post_Top_box_list">
          <li v-if="!(data?.resumes || []).length" class="page-top__empty">{{ $t('common_02409') }}</li>
          <li v-for="(row, i) in data?.resumes || []" :key="row.uid">
            <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
            <NuxtLink :to="`/resumes/${row.uid}`">{{ row.display_name }}</NuxtLink>
            <u>{{ row.expect_name }}</u>
            <em>{{ row.lastupdate_n }}</em>
          </li>
        </ul>
      </div>

      <div class="post_Top_box">
        <div class="post_Top_box_title Top_box_line3">
          <strong>{{ $t('admin_system_00443') }}</strong>
        </div>
        <ul class="post_Top_box_list">
          <li v-if="!(data?.keywords || []).length" class="page-top__empty">{{ $t('common_02409') }}</li>
          <li v-for="(row, i) in data?.keywords || []" :key="`${row.keyword}-${i}`">
            <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
            <NuxtLink :to="{ path: row.to || '/jobs', query: { keyword: row.keyword } }">
              {{ row.keyword }}
            </NuxtLink>
            <u>{{ row.type_name }}</u>
            <em>{{ row.num ?? row.hits }}</em>
          </li>
        </ul>
      </div>

      <div class="post_Top_box">
        <div class="post_Top_box_title Top_box_line3">
          <strong>{{ $t('default_00150') }}</strong>
        </div>
        <ul class="post_Top_box_list">
          <li v-if="!(data?.articles || []).length" class="page-top__empty">{{ $t('common_02409') }}</li>
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
          <li v-if="!(data?.urgentJobs || []).length" class="page-top__empty">{{ $t('common_02409') }}</li>
          <li v-for="(row, i) in data?.urgentJobs || []" :key="row.id">
            <span :class="{ Top_box_span: i < 3 }">{{ i + 1 }}</span>
            <NuxtLink :to="`/jobs/${row.id}`" :title="row.name">{{ row.name }}</NuxtLink>
            <u>{{ row.job_city_one }}</u>
            <em>{{ jobPay(row) }}</em>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style>
/* 排行榜皮写在页面里：H5 不加载 /legacy/pc.css（含 top.css），字号也不能跟 html rem。 */
.page-top {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 12px 24px;
  box-sizing: border-box;
  font-size: 14px;
  line-height: 1.5;
  color: #333;
}
.page-top .current_Location {
  padding: 15px 0;
  font-size: 14px;
  color: #625c5c;
}
.page-top .current_Location a {
  color: #000;
}
.page-top__grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 10px;
}
.page-top .post_Top_box {
  float: none !important;
  width: auto !important;
  height: auto !important;
  margin: 0 !important;
  background: #fff;
  border: 1px solid #ccc;
  padding: 10px;
  box-sizing: border-box;
  overflow: hidden;
}
.page-top .post_Top_box_title {
  width: 100%;
  height: 28px;
}
.page-top .post_Top_box_title strong {
  display: inline-block;
  color: #333;
  font-size: 16px;
  line-height: 24px;
  padding-left: 8px;
  font-weight: 700;
}
.page-top .Top_box_line {
  border-bottom: 2px solid #f196c1;
}
.page-top .Top_box_line strong {
  border-left: 4px solid #f196c1;
}
.page-top .Top_box_line2 {
  border-bottom: 2px solid #a6dead;
}
.page-top .Top_box_line2 strong {
  border-left: 4px solid #a6dead;
}
.page-top .Top_box_line3 {
  border-bottom: 2px solid #9cb6ff;
}
.page-top .Top_box_line3 strong {
  border-left: 4px solid #9cb6ff;
}
.page-top .post_Top_box_list {
  width: 100%;
  margin: 0;
  padding: 10px 0 0;
  list-style: none;
}
.page-top .post_Top_box_list li {
  display: flex;
  align-items: center;
  width: 100%;
  height: 30px;
  font-size: 14px;
}
.page-top .post_Top_box_list li span {
  width: 18px;
  height: 18px;
  font-size: 12px;
  text-align: center;
  line-height: 18px;
  flex-shrink: 0;
  background: #bbb;
  color: #fff;
  margin-right: 10px;
  font-weight: 700;
}
.page-top .post_Top_box_list li .Top_box_span {
  background: #f80;
}
.page-top .post_Top_box_list li a {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: #333;
  text-decoration: none;
}
.page-top .post_Top_box_list li u {
  flex-shrink: 0;
  max-width: 28%;
  margin-left: 6px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-style: normal;
  text-decoration: none;
  color: #666;
}
.page-top .post_Top_box_list li em {
  flex-shrink: 0;
  margin-left: 8px;
  color: #7c7c7c;
  font-size: 12px;
  font-style: normal;
}
.page-top .page-top__empty {
  height: auto;
  color: #999;
  font-size: 13px;
}
@media (min-width: 1200px) {
  .page-top {
    padding: 0 0 24px;
  }
  .page-top__grid {
    grid-template-columns: repeat(3, 1fr);
    gap: 10px 8px;
  }
  .page-top .post_Top_box {
    min-height: 330px;
  }
}
</style>
