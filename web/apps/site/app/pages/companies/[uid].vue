<script setup lang="ts">
import { listFailMsg, mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '~/utils/site'

const route = useRoute()
const { t, locale } = useI18n()
const uid = Number(route.params.uid)
const tab = computed(() => String(route.query.tab || 'jobs'))
const api = useApi()
const { data, error } = await useAsyncData(
  () => `company-${locale.value}-${uid}`,
  () => api.get('/v1/wap/companies/detail', { uid }),
)
const company = computed(() => (data.value || {}) as Record<string, unknown>)
const shows = computed(
  () => (Array.isArray(company.value.show) ? company.value.show : []) as Array<Record<string, unknown>>,
)
const welfare = computed(() => {
  const w = company.value.welfare_n || company.value.welfare
  if (Array.isArray(w)) return w.map(String).filter(Boolean)
  if (typeof w === 'string') return w.split(/[,，]/).map((s) => s.trim()).filter(Boolean)
  return [] as string[]
})
const { data: jobs } = await useAsyncData(
  () => `company-jobs-${locale.value}-${uid}`,
  () =>
    api.get<{ list: JobLike[] }>('/v1/wap/jobs', { page: 1, page_size: 20, uid }).catch(() => ({ list: [] as JobLike[] })),
)
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
useSeoMeta({
  title: () => String(company.value.name || t('common.company')),
  description: () => stripHtml(company.value.content || company.value.hy_n || company.value.name),
})
useHead({
  link: [{ rel: 'canonical', href: `/companies/${uid}` }],
  script: company.value.name
    ? [
        {
          type: 'application/ld+json',
          innerHTML: JSON.stringify({
            '@context': 'https://schema.org',
            '@type': 'Organization',
            name: company.value.name,
            description: stripHtml(company.value.content),
            identifier: String(uid),
            url: `/companies/${uid}`,
            logo: company.value.logo_n || company.value.logo || undefined,
          }),
        },
      ]
    : [],
})
</script>

<template>
  <article v-if="company.name">
    <div class="site-pc">
      <div class="com_details_top">
        <div class="w1200">
          <div class="com_details_current">
            {{ $t('common_01498') }}：<NuxtLink to="/">{{ $t('common.home') }}</NuxtLink> >
            <NuxtLink to="/companies">{{ $t('common.company') }}</NuxtLink> >
            <span>{{ company.name }}</span>
          </div>
          <div class="com_details_top_c">
            <div class="com_details_info_box">
              <div class="com_details_logo">
                <img :src="mediaUrl(String(company.logo_n || company.logo || ''), PLACEHOLDER_LOGO)" width="140" height="140" alt="" />
              </div>
              <h1 class="com_details_name">
                {{ company.name }}
                <img
                  v-if="Number(company.yyzz_status) === 1"
                  src="/legacy/pc/images/disc_icon10.png"
                  alt=""
                  class="png"
                  width="16"
                />
              </h1>
              <div class="com_details_info">
                {{ company.city_one }} <span v-if="company.city_two">- {{ company.city_two }}</span>
                <span v-if="company.hy_n" class="com_details_line">|</span>{{ company.hy_n }}
                <span v-if="company.pr_n" class="com_details_line">|</span>{{ company.pr_n }}
                <span v-if="company.mun_n" class="com_details_line">|</span>{{ company.mun_n }}
              </div>
              <p v-if="company.address" class="muted">{{ company.address }}</p>
              <p class="muted">
                <template v-if="company.sdate">{{ company.sdate }} · </template>
                <template v-if="company.zp_num != null">{{ $t('wap_00185') }} {{ company.zp_num }}</template>
                <template v-if="Number(company.isatn) === 1"> · {{ $t('wap_00378') }}</template>
              </p>
            </div>
            <p>
              <NuxtLink :to="{ query: { tab: 'jobs' } }" :class="{ Search_jobs_sub_cur: tab !== 'about' }">{{
                $t('wap_00190')
              }}</NuxtLink>
              ·
              <NuxtLink :to="{ query: { tab: 'about' } }" :class="{ Search_jobs_sub_cur: tab === 'about' }">{{
                $t('wap_00189')
              }}</NuxtLink>
            </p>
            <div v-if="tab === 'about'">
              <p v-if="company.address" class="muted">{{ $t('wap_00040') }}：{{ company.address }}</p>
              <div v-if="welfare.length" class="job_details_welfare">
                <span v-for="w in welfare" :key="w" class="job_details_welfare_n">{{ w }}</span>
              </div>
              <div v-if="shows.length" class="business_album">
                <img v-for="s in shows" :key="String(s.id)" :src="mediaUrl(String(s.picurl || ''), PLACEHOLDER_LOGO)" alt="" />
              </div>
              <div class="phpyunabout" v-html="String(company.content || '')" />
            </div>
            <div v-else>
              <JobCard v-for="job in jobs?.list || []" :key="job.id" :job="job" variant="search" />
              <p v-if="!(jobs?.list || []).length" class="muted">{{ $t('home.no_recruiting_jobs') }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="site-h5">
      <div class="top_card">
        <div class="top_card_top">
          <div class="top_card_top_logo">
            <img :src="mediaUrl(String(company.logo_n || company.logo || ''), PLACEHOLDER_LOGO)" alt="" width="100%" />
          </div>
          <div class="top_card_top_word">
            <div class="top_card_top_word_name">
              <div class="top_card_top_word_name_left">
                <div class="top_card_top_word_name_left_1">{{ company.name }}</div>
              </div>
            </div>
            <div class="newcom_info">
              <span v-if="company.mun_n">{{ company.mun_n }} ·</span>
              <span v-if="company.pr_n">{{ company.pr_n }} ·</span>
              <span>{{ company.hy_n }}</span>
            </div>
            <p v-if="company.zp_num != null" class="muted">{{ $t('wap_00185') }} {{ company.zp_num }}</p>
            <p v-if="company.address" class="muted">{{ company.address }}</p>
          </div>
        </div>
      </div>
      <div class="phpyuncomnav">
        <ul>
          <li>
            <NuxtLink :to="{ query: { tab: 'jobs' } }" :class="{ colorshow: tab !== 'about' }">{{
              $t('wap_00190')
            }}</NuxtLink>
            <span class="phpyunjobn">{{ company.zp_num ?? jobs?.list?.length ?? 0 }}</span>
          </li>
          <li>
            <NuxtLink :to="{ query: { tab: 'about' } }" :class="{ colorshow: tab === 'about' }">{{
              $t('wap_00189')
            }}</NuxtLink>
          </li>
        </ul>
      </div>
      <div v-if="tab === 'about'" class="company_generalize">
        <div v-if="company.address" class="job_describe_bottom">
          <div class="job_describe_cengter_header">{{ $t('wap_00040') }}</div>
          <div class="newcom_add">
            <div class="newcom_add_dz">{{ company.address }}</div>
          </div>
        </div>
        <div v-if="welfare.length" class="job_describe_bottom">
          <div class="job_describe_cengter_header">{{ $t('wap_com_00167') }}</div>
          <div class="job_describe_bottom_welfare">
            <ul>
              <li v-for="w in welfare" :key="w">{{ w }}</li>
            </ul>
          </div>
        </div>
        <div v-if="shows.length">
          <div class="job_describe_cengter_header">{{ $t('wap_com_00401') }}</div>
          <div class="business_album">
            <img v-for="s in shows" :key="String(s.id)" :src="mediaUrl(String(s.picurl || ''), PLACEHOLDER_LOGO)" alt="" />
          </div>
        </div>
        <div class="job_describe_cengter_header">{{ $t('wap_com_00168') }}</div>
        <div class="phpyunabout" v-html="String(company.content || '')" />
      </div>
      <div v-else id="company_job_list">
        <JobCard v-for="job in jobs?.list || []" :key="job.id" :job="job" variant="com" />
        <div v-if="!(jobs?.list || []).length" class="wap_member_no">{{ $t('home.no_recruiting_jobs') }}</div>
      </div>
    </div>
  </article>
  <article v-else class="site-inner">
    <h1>{{ $t('common.company') }}</h1>
    <p class="muted">{{ error ? failMsg : $t('ui.no_data') }}</p>
  </article>
</template>
