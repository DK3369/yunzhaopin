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
  }, t('common.negotiable')),
)
const welfare = computed(() => {
  const w = dict.value.welfare_names || job.value.welfare || job.value.job_welfare
  if (Array.isArray(w)) return w.map(String).filter(Boolean)
  if (typeof w === 'string') return w.split(/[,，]/).map((s) => s.trim()).filter(Boolean)
  return [] as string[]
})
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
      <div class="job_ceil">
        <div class="w1200">
          <div class="job_ceil_box">
            <div class="job_ceil_cont">
              <span class="job_ceil_jobname">{{ job.name }}</span>
              <span class="job_ceil_jobxz">{{ salary }}</span>
              <a href="javascript:;" class="job_ceil_jobsc" @click.prevent="toggleFav">{{
                fav ? $t('wap_00378') : $t('wap_00379')
              }}</a>
              <a href="javascript:;" class="job_ceil_jobsc" @click.prevent="report">{{ $t('common.submit') }}</a>
              <a href="javascript:;" class="job_ceil_jobtd" @click.prevent="apply">{{ $t('wap_com_00235') }}</a>
            </div>
          </div>
        </div>
      </div>
      <div class="job_details_top">
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
              <div v-if="welfare.length" class="job_details_welfare">
                <span v-for="w in welfare" :key="w" class="job_details_welfare_n">{{ w }}</span>
              </div>
              <p v-if="applyMsg" class="muted">{{ applyMsg }}</p>
            </div>
          </div>
        </div>
      </div>
      <div class="w1200">
        <div class="job_details_left">
          <div class="job_details_left_box">
            <div class="job_details_touch">
              <div class="job_details_user">
                <div class="job_details_userpic">
                  <img
                    :src="mediaUrl(String(company.logo_n || company.logo || job.com_logo || ''), PLACEHOLDER_LOGO)"
                    alt=""
                  />
                </div>
                <div>
                  <span class="job_details_touch_username">{{ contact?.linkman || job.com_name }}</span>
                </div>
              </div>
              <div class="job_details_touch_tel">
                {{ $t('common.phone') }}：
                <span class="job_details_touch_tel_n">{{ contact?.linktel || '****' }}</span>
                <a href="javascript:;" class="job_details_touch_tel_bth" @click.prevent="showTel">{{
                  $t('common.phone')
                }}</a>
              </div>
            </div>
            <h2>{{ $t('common.job') }}</h2>
            <div v-html="String(job.description || job.content || '')" />
            <h2 v-if="similar?.length">{{ $t('home.recommended_jobs') }}</h2>
            <div v-if="similar?.length">
              <JobCard v-for="row in similar" :key="row.id" :job="row" variant="search" />
            </div>
          </div>
        </div>
        <div class="Compply_right_sidebar">
          <div class="Compply_right_qy">
            <div class="Compply_logo">
              <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`">
                <img
                  :src="mediaUrl(String(company.logo_n || company.logo || job.com_logo || ''), PLACEHOLDER_LOGO)"
                  alt=""
                />
              </NuxtLink>
            </div>
            <div class="Compply_right_name">
              <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`">{{ job.com_name || company.name }}</NuxtLink>
            </div>
            <p class="Compply_right_name_all">{{ company.hy_n }}</p>
          </div>
          <div v-if="sameCom?.length" class="Compply_right_post">
            <ul class="Compply_right_post_other">
              <li v-for="row in sameCom || []" :key="row.id">
                <NuxtLink :to="`/jobs/${row.id}`" class="Compply_right_post_other_name">{{ row.name }}</NuxtLink>
              </li>
            </ul>
          </div>
        </div>
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
          <div v-if="welfare.length" class="job_describe_bottom">
            <div class="job_describe_cengter_header">{{ $t('common.more') }}</div>
            <div class="job_describe_bottom_welfare">
              <span v-for="w in welfare" :key="w">{{ w }}</span>
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
            <JobCard v-for="row in similar" :key="row.id" :job="row" variant="search" />
          </div>
        </div>
      </div>
      <div class="yun_czfoot">
        <div class="yun_czfootfixed">
          <div class="yun_czfoot_c">
            <div class="yun_czfoot_l">
              <NuxtLink to="/" class="yun_czfoot_s">
                <div class="yun_czfoot_s_p yun_czfoot_hmicon">{{ $t('common.home') }}</div>
              </NuxtLink>
              <a href="javascript:;" class="yun_czfoot_s" @click.prevent="toggleFav">
                <div class="yun_czfoot_s_p yun_czfoot_scicon">{{ $t('member_user_00103') }}</div>
              </a>
            </div>
            <a href="javascript:;" class="yun_czfoot_s" @click.prevent="apply">
              <div class="yun_czfoot_s_p yun_czfoot_jlicon">{{ applyMsg || $t('wap_com_00235') }}</div>
            </a>
            <a href="javascript:;" class="yun_czfoot_s" @click.prevent="showTel">
              <div class="yun_czfoot_s_p">{{ $t('common.phone') }}</div>
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div v-else class="site-inner">
    <h1>{{ $t('common.job') }}</h1>
    <p class="muted">{{ error ? $t('ui.load_failed') : $t('home.no_recruiting_jobs') }}</p>
  </div>
</template>
