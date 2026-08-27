<script setup lang="ts">
import { formatSalary, mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '~/utils/site'

const route = useRoute()
const { t, locale } = useI18n()
const { siteName, settings } = useSiteChrome()
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
const userContext = computed(
  () =>
    ((data.value as { user_context?: Record<string, unknown> } | null)?.user_context || {}) as Record<
      string,
      unknown
    >,
)
const formatted = computed(
  () =>
    ((data.value as { formatted?: Record<string, unknown> } | null)?.formatted || {}) as Record<
      string,
      unknown
    >,
)
const msgList = computed(
  () =>
    ((data.value as { msg_list?: Array<Record<string, unknown>> } | null)?.msg_list || []) as Array<
      Record<string, unknown>
    >,
)
const alreadyApplied = computed(() => Boolean(userContext.value.is_applied))
const eduLabel = computed(() => String(dict.value.edu_n || dict.value.job_edu || job.value.edu_n || ''))
const expLabel = computed(() => String(dict.value.exp_n || dict.value.job_exp || job.value.exp_n || ''))
const cityLabel = computed(() => {
  const parts = [
    dict.value.city_one || job.value.job_city_one || job.value.city_one,
    dict.value.city_two || job.value.job_city_two || job.value.city_two,
    dict.value.city_three,
  ]
    .map((v) => String(v || ''))
    .filter(Boolean)
  return parts.join('-')
})
const hyLabel = computed(() => String(dict.value.hy_n || company.value.hy_n || job.value.job_hy || job.value.hy_n || ''))
const munLabel = computed(() => String(dict.value.mun_n || ''))
const prLabel = computed(() => String(dict.value.pr_n || ''))
const hits = computed(() => Number(job.value.jobhits || 0))
const yqItems = computed(() => {
  const out: string[] = []
  if (dict.value.number_n) out.push(String(dict.value.number_n))
  if (dict.value.type_n) out.push(String(dict.value.type_n))
  if (dict.value.age_n) out.push(String(dict.value.age_n))
  if (dict.value.sex_n) out.push(String(dict.value.sex_n))
  if (dict.value.marriage_n) out.push(String(dict.value.marriage_n))
  const langs = dict.value.langname
  if (Array.isArray(langs)) out.push(...langs.map(String).filter(Boolean))
  else if (typeof langs === 'string' && langs) out.push(langs)
  return out
})
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
const comAddress = computed(() => String(company.value.address || job.value.address || ''))
const shenming = computed(() => String(settings.value.sy_shenming || ''))
const similarList = computed(() => similar.value || [])
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
              <a href="javascript:;" class="job_ceil_jobtd" @click.prevent="apply">{{
                alreadyApplied ? $t('ui.already_applied') : $t('wap_com_00235')
              }}</a>
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
              <div class="job_details_info">
                <template v-if="cityLabel">{{ cityLabel }}</template>
                <template v-if="expLabel">
                  <span class="job_details_line">|</span>{{ expLabel }}{{ $t('home.experience_suffix') }}
                </template>
                <template v-if="eduLabel">
                  <span class="job_details_line">|</span>{{ eduLabel }}{{ $t('home.education_suffix') }}
                </template>
              </div>
              <div v-if="welfare.length" class="job_details_welfare">
                <span v-for="w in welfare" :key="w" class="job_details_welfare_n">{{ w }}</span>
              </div>
              <div class="job_details_topright_data">
                <span v-if="formatted.lastupdate_n" class="job_details_topright_data_time">
                  {{ formatted.lastupdate_n }} {{ $t('wap_00225') }}
                </span>
                <template v-if="hits">
                  {{ $t('member_com_00268') }}：{{ hits }} {{ $t('common_02089') }}
                </template>
              </div>
              <p v-if="applyMsg" class="muted">{{ applyMsg }}</p>
            </div>
            <div class="job_details_topright">
              <div class="job_details_top_operation">
                <a href="javascript:;" class="job_details_top_operation_sc" @click.prevent="toggleFav">{{
                  fav ? $t('wap_00378') : $t('wap_00379')
                }}</a>
                <a href="javascript:;" class="job_details_top_operation_sq" @click.prevent="apply">{{
                  alreadyApplied ? $t('ui.already_applied') : $t('wap_com_00235')
                }}</a>
              </div>
              <div class="job_details_top_extension">
                <a href="javascript:;" class="job_details_top_extension_jb" @click.prevent="report">{{
                  $t('wap_com_00350')
                }}</a>
              </div>
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
                  $t('default_00233')
                }}</a>
              </div>
            </div>
            <h2>{{ $t('wap_00287') }}</h2>
            <ul v-if="yqItems.length" class="job_describe_yq">
              <li v-for="item in yqItems" :key="item">{{ item }}</li>
            </ul>
            <div v-html="String(job.description || job.content || '')" />
            <div v-if="msgList.length" class="job_details_left_box" style="margin-top: 16px">
              <h2>{{ $t('common.message') }}</h2>
              <p v-for="m in msgList" :key="String(m.id)" class="muted">
                <strong>{{ m.username }}</strong>
                {{ m.content }}
                <template v-if="m.reply"><br />{{ m.reply }}</template>
              </p>
            </div>
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
            <p class="Compply_right_name_all">{{ hyLabel }}</p>
            <ul class="Compply_right_js">
              <li v-if="hyLabel">
                <span class="Compply_right_span_c">{{ hyLabel }}</span>
              </li>
              <li v-if="prLabel">
                <span class="Compply_right_span_c">{{ prLabel }}</span>
              </li>
              <li v-if="munLabel">
                <span class="Compply_right_span_c">{{ munLabel }}</span>
              </li>
            </ul>
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
              <div v-if="cityLabel" class="job_describe_top_require_left">
                <i><img src="/legacy/h5/images/icon_orientation.png" alt="" style="width: 100%" /></i>
                <i>{{ cityLabel }}</i>
              </div>
              <div v-if="eduLabel" class="job_describe_top_require_center">
                <div class="job_describe_top_require_left">
                  <i><img src="/legacy/h5/images/icon_fixed.png" alt="" style="width: 100%" /></i>
                  <i>{{ eduLabel }}</i>
                </div>
              </div>
              <div v-if="expLabel" class="job_describe_top_require_right">
                <div class="job_describe_top_require_left">
                  <i><img src="/legacy/h5/images/icon_education.png" alt="" style="width: 100%" /></i>
                  <i>{{ expLabel }}</i>
                </div>
              </div>
            </div>
            <div class="newjob_show_sj">
              <span v-if="formatted.lastupdate_n">{{ $t('wap_00225') }} {{ formatted.lastupdate_n }}</span>
              <span v-if="hits">{{ $t('wap_user_00221') }} {{ hits }}</span>
            </div>
          </div>
          <div v-if="welfare.length" class="job_describe_bottom">
            <div class="job_describe_cengter_header">{{ $t('wap_00286') }}</div>
            <div class="job_describe_bottom_welfare">
              <ul>
                <li v-for="w in welfare" :key="w">{{ w }}</li>
              </ul>
            </div>
          </div>
          <div class="job_describe_cengter">
            <div class="job_describe_cengter_header">{{ $t('wap_00287') }}</div>
            <ul v-if="yqItems.length" class="job_describe_yq">
              <li v-for="item in yqItems" :key="item">{{ item }}</li>
            </ul>
            <div class="newjob_js" v-html="String(job.description || job.content || '')" />
          </div>
        </div>
        <div v-if="job.uid" class="corporate_information">
          <div class="corporate_information_header">{{ $t('wap_00270') }}</div>
          <NuxtLink :to="`/companies/${job.uid}`">
            <div class="corporate_information_message">
              <div class="corporate_information_message_logo">
                <img
                  :src="mediaUrl(String(company.logo_n || company.logo || job.com_logo || ''), PLACEHOLDER_LOGO)"
                  alt=""
                  width="100%"
                />
              </div>
              <div class="corporate_information_message_name">
                <div>{{ job.com_name || company.name }}</div>
                <div class="com_j_info">
                  <span v-if="munLabel">{{ munLabel }}</span>
                  <span v-if="prLabel">· {{ prLabel }} ·</span>
                  <span>{{ hyLabel }}</span>
                </div>
              </div>
            </div>
          </NuxtLink>
          <div v-if="comAddress" class="corporate_information_map_p">{{ comAddress }}</div>
        </div>
        <div v-if="shenming" class="wxtipbox">
          <div class="wxtip">
            <div class="wxtip_tit">{{ siteName }}{{ $t('wap_user_00205') }}</div>
          </div>
          <div>
            {{ shenming }}
            <span class="wxtip_bth" @click="report">{{ $t('wap_00283') }}</span>
          </div>
        </div>
        <div class="company_questions">
          <div class="company_questions_header">
            <div class="company_questions_header_left">{{ $t('wap_00271') }}</div>
          </div>
          <div v-for="m in msgList" :key="String(m.id)" class="company_questions_body">
            <div class="company_questions_body_top">
              <i class="company_questions_body_top_ask">{{ m.content }}</i>
            </div>
            <div class="company_questions_body_top">
              <i class="company_questions_body_top_answer">{{ m.reply || $t('wap_01589') }}</i>
            </div>
          </div>
        </div>
        <div class="recommend_post" style="margin-top: 0">
          <div class="recommend_post_header" style="margin: 0.4rem 0">{{ $t('wap_00282') }}</div>
          <div class="recommend_post_card_box">
            <div v-for="row in similarList" :key="row.id" class="recommend_post_card">
              <NuxtLink :to="`/jobs/${row.id}`" :title="row.name">
                <div class="recommend_post_card_top">
                  <div class="recommend_post_card_name">{{ row.name }}</div>
                  <div class="recommend_post_card_money">{{ formatSalary(row, $t('common.negotiable')) }}</div>
                </div>
                <div class="newjob_info">
                  <span>{{ row.job_city_one }}{{ row.job_city_two ? `-${row.job_city_two}` : '' }}</span>
                  <template v-if="row.edu_n">
                    <i class="newjob_info_line" /><span>{{ row.edu_n }}{{ $t('home.education_suffix') }}</span>
                  </template>
                  <template v-if="row.exp_n">
                    <i class="newjob_info_line" /><span>{{ row.exp_n }}{{ $t('home.experience_suffix') }}</span>
                  </template>
                </div>
                <div class="recommend_post_card_bottom">
                  <div class="recommend_post_card_bottom_left">
                    <div class="recommend_post_card_bottom_left_logo">
                      <img :src="mediaUrl(row.com_logo || row.logo, PLACEHOLDER_LOGO)" alt="" width="100%" />
                    </div>
                    <i class="recommend_post_card_bottom_left_word">{{ row.com_name }}</i>
                  </div>
                  <div class="recommend_post_card_bottom_right">{{ row.lastupdate_n }}</div>
                </div>
              </NuxtLink>
            </div>
            <div v-if="!similarList.length" class="company_questions">
              <div class="wap_member_no">{{ $t('wap_00253') }}</div>
            </div>
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
              <div class="yun_czfoot_s_p yun_czfoot_jlicon">{{
                applyMsg || (alreadyApplied ? $t('ui.already_applied') : $t('wap_com_00235'))
              }}</div>
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
