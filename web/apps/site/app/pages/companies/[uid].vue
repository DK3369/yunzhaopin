<script setup lang="ts">
import { mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '~/utils/site'

const route = useRoute()
const { t, locale } = useI18n()
const uid = Number(route.params.uid)
const tab = computed(() => String(route.query.tab || 'jobs'))
const api = useApi()
const { data } = await useAsyncData(
  () => `company-${locale.value}-${uid}`,
  () => api.get('/v1/wap/companies/detail', { uid }),
)
const company = computed(() => (data.value || {}) as Record<string, unknown>)
const { data: jobs } = await useAsyncData(
  () => `company-jobs-${locale.value}-${uid}`,
  () =>
    api.get<{ list: JobLike[] }>('/v1/wap/jobs', { page: 1, page_size: 20, uid }).catch(() => ({ list: [] as JobLike[] })),
)
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
      <div class="yun_content" style="padding: 20px 0 40px">
        <div style="display: flex; gap: 16px; align-items: center; margin-bottom: 16px">
          <img :src="mediaUrl(String(company.logo_n || company.logo || ''), PLACEHOLDER_LOGO)" width="80" height="80" alt="" />
          <div>
            <h1>{{ company.name }}</h1>
            <p class="muted">{{ company.hy_n }} · {{ company.city_two }}</p>
          </div>
        </div>
        <p>
          <NuxtLink :to="{ query: { tab: 'jobs' } }" :class="{ Search_jobs_sub_cur: tab !== 'about' }">{{
            $t('home.latest_jobs')
          }}</NuxtLink>
          ·
          <NuxtLink :to="{ query: { tab: 'about' } }" :class="{ Search_jobs_sub_cur: tab === 'about' }">{{
            $t('common.company')
          }}</NuxtLink>
        </p>
        <div v-if="tab === 'about'" v-html="String(company.content || '')" />
        <div v-else class="index_newjobbox">
          <ul>
            <JobCard v-for="job in jobs?.list || []" :key="job.id" :job="job" />
          </ul>
          <p v-if="!(jobs?.list || []).length" class="muted">{{ $t('home.no_recruiting_jobs') }}</p>
        </div>
      </div>
    </div>
    <div class="site-h5">
      <div style="background: #fff; padding: 0.32rem; display: flex; gap: 0.24rem">
        <img :src="mediaUrl(String(company.logo_n || company.logo || ''), PLACEHOLDER_LOGO)" style="width: 1.2rem; height: 1.2rem" alt="" />
        <div>
          <h1 style="font-size: 0.42rem">{{ company.name }}</h1>
          <p class="muted">{{ company.hy_n }} · {{ company.city_two }}</p>
        </div>
      </div>
      <div class="job_header_nav_left category" style="background: #fff">
        <ul>
          <li :class="{ active: tab !== 'about' }">
            <NuxtLink :to="{ query: { tab: 'jobs' } }">{{ $t('home.latest_jobs') }}</NuxtLink>
          </li>
          <li :class="{ active: tab === 'about' }">
            <NuxtLink :to="{ query: { tab: 'about' } }">{{ $t('common.company') }}</NuxtLink>
          </li>
        </ul>
      </div>
      <div v-if="tab === 'about'" style="background: #fff; margin-top: 0.2rem; padding: 0.32rem" v-html="String(company.content || '')" />
      <div v-else style="margin-top: 0.2rem">
        <JobCard v-for="job in jobs?.list || []" :key="job.id" :job="job" />
        <p v-if="!(jobs?.list || []).length" class="muted" style="padding: 0.4rem">{{ $t('home.no_recruiting_jobs') }}</p>
      </div>
    </div>
  </article>
  <article v-else class="site-inner">
    <h1>{{ $t('common.company') }}</h1>
    <p class="muted">{{ $t('home.no_job_data') }}</p>
  </article>
</template>
