<script setup lang="ts">
import { formatSalary, mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '~/utils/site'

const route = useRoute()
const { t, locale } = useI18n()
const id = Number(route.params.id)
const api = useApi()
const { data, error } = await useAsyncData(
  () => `job-${locale.value}-${id}`,
  () => api.get('/v1/wap/jobs/detail', { id }),
)
const job = computed(
  () => ((data.value as { job?: Record<string, unknown> } | null)?.job || {}) as Record<string, unknown>,
)
const dict = computed(
  () => ((data.value as { dict?: Record<string, unknown> } | null)?.dict || {}) as Record<string, unknown>,
)
const company = computed(
  () =>
    ((data.value as { company?: Record<string, unknown> } | null)?.company || {}) as Record<string, unknown>,
)
const { data: similar } = await useAsyncData(
  () => `job-similar-${locale.value}-${id}`,
  () => api.get<JobLike[]>('/v1/wap/jobs/similar', { id, limit: 8 }).catch(() => [] as JobLike[]),
)
const { data: sameCom } = await useAsyncData(
  () => `job-same-${locale.value}-${id}`,
  () => api.get<JobLike[]>('/v1/wap/jobs/same-company', { id, limit: 6 }).catch(() => [] as JobLike[]),
)
const fav = ref(false)
const applyMsg = ref('')
const contact = ref<{ linktel?: string; linkman?: string } | null>(null)
onMounted(async () => {
  try {
    const r = await api.post<{ exists?: boolean; favorited?: boolean }>('/v1/mcenter/favorites/exists', {
      kind: 1,
      target_id: id,
    })
    fav.value = Boolean(r.exists || r.favorited)
  } catch {
    /* guest */
  }
})
async function apply() {
  applyMsg.value = ''
  try {
    await api.post('/v1/mcenter/apply', { job_id: id })
    applyMsg.value = t('common.confirm')
  } catch (e: unknown) {
    applyMsg.value = e instanceof Error ? e.message : t('common.no')
  }
}
async function toggleFav() {
  try {
    const r = await api.post<{ favorited: boolean }>('/v1/mcenter/favorites', { kind: 1, target_id: id })
    fav.value = Boolean(r.favorited)
  } catch {
    await navigateTo('/login')
  }
}
async function showTel() {
  try {
    contact.value = await api.get('/v1/wap/jobs/contact', { id })
    await api.post('/v1/wap/jobs/tel-click', { id }).catch(() => undefined)
  } catch (e: unknown) {
    applyMsg.value = e instanceof Error ? e.message : t('common.phone')
  }
}
async function report() {
  try {
    await api.post('/v1/mcenter/reports', { target_kind: 1, target_id: id, reason_code: 'other', detail: '' })
    applyMsg.value = t('common.confirm')
  } catch {
    await navigateTo('/login')
  }
}
const salary = computed(() =>
  formatSalary({
    id,
    name: String(job.value.name || ''),
    min_salary: Number(job.value.minsalary || job.value.min_salary || 0),
    max_salary: Number(job.value.maxsalary || job.value.max_salary || 0),
  }),
)
const description = computed(() =>
  stripHtml(job.value.description || job.value.content || job.value.name || job.value.com_name),
)
const datePosted = computed(() => unixToIso(job.value.lastupdate || job.value.sdate))
const employmentType = computed(() => {
  const n = Number(job.value.type)
  if (n === 58) return 'PART_TIME'
  if (n === 59) return 'INTERN'
  if (n === 60) return 'TEMPORARY'
  return 'FULL_TIME'
})
useSeoMeta({
  title: () => String(job.value.name || t('common.job')),
  description: () => description.value,
})
useHead({
  link: [{ rel: 'canonical', href: `/jobs/${id}` }],
  script: job.value.name
    ? [
        {
          type: 'application/ld+json',
          innerHTML: JSON.stringify({
            '@context': 'https://schema.org',
            '@type': 'JobPosting',
            title: job.value.name,
            description: description.value || String(job.value.name),
            datePosted: datePosted.value,
            hiringOrganization: {
              '@type': 'Organization',
              name: job.value.com_name || job.value.name,
            },
            jobLocation: dict.value.city_two
              ? {
                  '@type': 'Place',
                  address: {
                    '@type': 'PostalAddress',
                    addressLocality: dict.value.city_two,
                    addressRegion: dict.value.city_one,
                    addressCountry: 'CN',
                  },
                }
              : undefined,
            employmentType: employmentType.value,
            identifier: String(id),
          }),
        },
      ]
    : [],
})
</script>

<template>
  <div v-if="job.name">
    <div class="site-pc">
      <div class="job_details_top job-detail-sticky">
        <div class="w1200">
          <div class="job_details_current">
            {{ $t('common_01498') }}：<NuxtLink to="/">{{ $t('common.home') }}</NuxtLink> >
            <NuxtLink to="/jobs">{{ $t('common.job') }}</NuxtLink> > {{ job.name }}
          </div>
          <div class="job_details_topbox">
            <div class="job_details_topleft">
              <h1 class="job_details_name">{{ job.name }}</h1>
              <span class="job_details_salary_n">{{ salary }}</span>
              <p class="muted" style="margin-top: 12px">
                {{ dict.city_two || job.city_two }} · {{ dict.job_edu || dict.edu_n }} · {{ dict.job_exp || dict.exp_n }}
              </p>
              <p>{{ job.com_name }}</p>
              <div style="margin-top: 16px; display: flex; gap: 12px">
                <button type="button" class="job_ceil_jobtd" @click="apply">{{ $t('wap_com_00235') }}</button>
                <button type="button" class="job_ceil_jobtd" @click="toggleFav">
                  {{ fav ? $t('common.yes') : $t('member_user_00103') }}
                </button>
                <button type="button" class="job_ceil_jobtd" @click="showTel">{{ $t('common.phone') }}</button>
                <button type="button" class="job_ceil_jobtd" @click="report">{{ $t('common.submit') }}</button>
              </div>
              <p v-if="applyMsg" class="muted">{{ applyMsg }}</p>
              <p v-if="contact?.linktel" class="muted">{{ contact.linkman }} {{ contact.linktel }}</p>
            </div>
          </div>
        </div>
      </div>
      <div class="yun_content" style="padding: 20px 0 40px; display: flex; gap: 24px">
        <div style="flex: 1">
          <h2 style="margin-bottom: 12px">{{ $t('common.job') }}</h2>
          <div v-html="String(job.description || job.content || '')" />
          <h2 v-if="similar?.length" style="margin: 24px 0 12px">{{ $t('home.recommended_jobs') }}</h2>
          <ul v-if="similar?.length" class="index_newjobbox">
            <JobCard v-for="row in similar" :key="row.id" :job="row" />
          </ul>
        </div>
        <aside style="width: 280px">
          <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`" style="display: flex; gap: 12px; margin-bottom: 16px">
            <img
              :src="mediaUrl(String(company.logo_n || company.logo || job.com_logo || ''), PLACEHOLDER_LOGO)"
              width="64"
              height="64"
              alt=""
            />
            <div>
              <strong>{{ job.com_name || company.name }}</strong>
              <p class="muted">{{ company.hy_n }}</p>
            </div>
          </NuxtLink>
          <h3 v-if="sameCom?.length">{{ $t('home.latest_jobs') }}</h3>
          <p v-for="row in sameCom || []" :key="row.id">
            <NuxtLink :to="`/jobs/${row.id}`">{{ row.name }}</NuxtLink>
          </p>
        </aside>
      </div>
    </div>

    <div class="site-h5">
      <div class="min_body">
        <div class="job_describe">
          <div class="job_describe_top">
            <div class="new_jobshowtop">
              <div class="new_jobshowname">{{ job.name }}</div>
              <span class="new_jobshowxz">{{ salary }}</span>
            </div>
            <div class="job_describe_top_require">
              <div class="job_describe_top_require_left">
                <i>{{ dict.city_two || job.city_two || dict.city_one }}</i>
              </div>
              <div v-if="dict.job_edu || dict.edu_n" class="job_describe_top_require_center">
                <div class="job_describe_top_require_left">
                  <i>{{ dict.job_edu || dict.edu_n }}</i>
                </div>
              </div>
              <div v-if="dict.job_exp || dict.exp_n" class="job_describe_top_require_right">
                <div class="job_describe_top_require_left">
                  <i>{{ dict.job_exp || dict.exp_n }}</i>
                </div>
              </div>
            </div>
          </div>
          <div class="job_describe_box" style="background: #fff; padding: 0.32rem; margin-top: 0.2rem">
            <div v-html="String(job.description || job.content || '')" />
          </div>
          <NuxtLink
            v-if="job.uid"
            :to="`/companies/${job.uid}`"
            class="index_company"
            style="display: flex; background: #fff; margin-top: 0.2rem; padding: 0.32rem"
          >
            <i class="index_company-logo">
              <img
                :src="mediaUrl(String(company.logo_n || company.logo || job.com_logo || ''), PLACEHOLDER_LOGO)"
                alt=""
                style="width: 100%"
              />
            </i>
            <i class="index_company-name">{{ job.com_name || company.name }}</i>
          </NuxtLink>
          <div v-if="similar?.length" style="background: #fff; margin-top: 0.2rem; padding: 0.2rem">
            <JobCard v-for="row in similar" :key="row.id" :job="row" />
          </div>
        </div>
      </div>
      <div
        style="position: fixed; left: 0; right: 0; bottom: 1.82rem; background: #fff; padding: 0.2rem 0.32rem; z-index: 90; display: flex; gap: 0.16rem"
      >
        <button type="button" class="login_bth" style="flex: 1; height: 1rem; background: #2778f8; color: #fff; border: 0" @click="apply">
          {{ applyMsg || $t('wap_com_00235') }}
        </button>
        <button type="button" style="width: 1.6rem; height: 1rem" @click="toggleFav">{{ $t('member_user_00103') }}</button>
        <button type="button" style="width: 1.6rem; height: 1rem" @click="showTel">{{ $t('common.phone') }}</button>
      </div>
    </div>
  </div>
  <div v-else class="site-inner">
    <h1>{{ $t('common.job') }}</h1>
    <p class="muted">{{ error ? $t('home.no_job_data') : $t('home.no_recruiting_jobs') }}</p>
  </div>
</template>
