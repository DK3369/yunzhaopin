<script setup lang="ts">
import { dictReqLabel, formatSalary, formatUnixDate, mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '~/utils/site'

const route = useRoute()
const { t, te, locale } = useI18n()
const { siteName, settings, me } = useSiteChrome()
const id = Number(route.params.id)
const api = useApi()
const salaryType = computed(() => Number(settings.value.resume_salarytype || 1))
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
const appliedLocal = ref(false)
const alreadyApplied = computed(() => appliedLocal.value || Boolean(userContext.value.is_applied))
const alreadyInvited = computed(() => Number(userContext.value.invite_job || 0) > 0)
const jobClosed = computed(() => {
  const payload = (data.value || {}) as Record<string, unknown>
  return Boolean(payload.offline || payload.expired) || Number(job.value.status) === 1
})
const showSnum = computed(() => {
  const snum = Number(formatted.value.snum || job.value.snum || 0)
  const threshold = Number(settings.value.sy_sq_job_num || 0)
  return snum > threshold
})
const applyCta = computed(() => {
  if (alreadyApplied.value) return { kind: 'applied' as const, label: t('ui.already_applied') }
  if (alreadyInvited.value) return { kind: 'invited' as const, label: t('wap_00291') }
  return { kind: 'apply' as const, label: t('wap_com_00235') }
})
const comMessageOn = computed(() => String(settings.value.com_message || '') === '1')
const askContent = ref('')
const askCode = ref('')
const askCaptcha = ref<{ cid: string; image: string } | null>(null)
const askMsg = ref('')
async function loadAskCaptcha() {
  try {
    askCaptcha.value = await api.post('/v1/wap/captcha')
  } catch {
    askCaptcha.value = null
  }
}
async function postAsk() {
  askMsg.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  if (me.value.usertype !== 1) {
    askMsg.value = t('wap_00256')
    return
  }
  if (!askCaptcha.value) await loadAskCaptcha()
  try {
    await api.post('/v1/wap/jobs/messages/post', {
      id,
      content: askContent.value,
      captcha_cid: askCaptcha.value?.cid,
      authcode: askCode.value,
    })
    askContent.value = ''
    askCode.value = ''
    askMsg.value = t('common.confirm')
    await loadAskCaptcha()
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('ui.load_failed')
    await loadAskCaptcha()
  }
}
const favFromApi = computed(() => Boolean(userContext.value.is_favorited))
const eduLabel = computed(() => dictReqLabel(String(dict.value.edu_n || dict.value.job_edu || job.value.edu_n || ''), t('home.education_suffix')))
const expLabel = computed(() => dictReqLabel(String(dict.value.exp_n || dict.value.job_exp || job.value.exp_n || ''), t('home.experience_suffix')))
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
const sexSwitch = computed(() => String(settings.value.com_job_sexswitch || '') === '1')
const yqItems = computed(() => {
  const out: Array<{ label: string; value: string }> = []
  const zpNum = Number(job.value.zp_num || 0)
  if (zpNum > 0) out.push({ label: t('wap_com_00333'), value: String(zpNum) })
  else if (dict.value.number_n) out.push({ label: t('wap_com_00333'), value: String(dict.value.number_n) })
  if (dict.value.report_n) out.push({ label: t('wap_com_00279'), value: String(dict.value.report_n) })
  if (Number(job.value.is_graduate) === 1) out.push({ label: t('wap_com_00280'), value: t('common.yes') })
  if (dict.value.age_n) out.push({ label: t('wap_com_00284'), value: String(dict.value.age_n) })
  if (sexSwitch.value && dict.value.sex_n) out.push({ label: t('wap_com_00332'), value: String(dict.value.sex_n) })
  if (dict.value.marriage_n) out.push({ label: t('default_00366'), value: String(dict.value.marriage_n) })
  const langs = dict.value.langname
  if (Array.isArray(langs)) {
    for (const n of langs) {
      if (n) out.push({ label: t('wap_com_00292'), value: String(n) })
    }
  }
  return out
})
const contactInfo = computed(
  () =>
    ((data.value as { contact?: Record<string, unknown> } | null)?.contact || {}) as Record<
      string,
      unknown
    >,
)
const { data: adsBanner } = await useAsyncData('ads-509', () =>
  api.get<Array<{ image_n?: string }>>('/v1/wap/ads', { slot: '509', limit: 1 }).catch(() => []),
)
const { data: adsH5 } = await useAsyncData('ads-512', () =>
  api.get<Array<{ image_n?: string; image?: string; link?: string }>>('/v1/wap/ads', { slot: '512', limit: 1 }).catch(() => []),
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
const revealed = ref<{ linktel?: string; linkphone?: string; linkman?: string } | null>(null)
const ceilShow = ref(false)
const h5LinkOpen = ref(false)
const reportOpen = ref(false)
watch(
  favFromApi,
  (v) => {
    if (v) fav.value = true
  },
  { immediate: true },
)
const telDisplay = computed(
  () =>
    revealed.value?.linktel
    || revealed.value?.linkphone
    || String(contactInfo.value.linktel_n || contactInfo.value.linkphone_n || ''),
)
const linkCode = computed(() => Number(contactInfo.value.link_code || 0))
const applyStats = computed(() => ({
  snum: Number(formatted.value.snum || job.value.snum || 0),
  pre: Number(formatted.value.pre || 0),
  operatime: String(formatted.value.operatime_n || ''),
}))
const mapHref = computed(() => {
  const x = String(contactInfo.value.x || company.value.x || job.value.x || '')
  const y = String(contactInfo.value.y || company.value.y || job.value.y || '')
  if (x && y) return `/map?x=${encodeURIComponent(x)}&y=${encodeURIComponent(y)}`
  return ''
})
const linkMsg = computed(() => {
  const raw = String(contactInfo.value.link_msg || '')
  if (!raw) return ''
  if (/^[a-z][a-z0-9_]*_\d+$/i.test(raw) || /^[a-z][a-z0-9_.]+$/i.test(raw)) {
    const key = raw as never
    return te(key) ? t(key) : raw
  }
  return raw
})
onMounted(async () => {
  const onScroll = () => {
    const s = window.scrollY || document.documentElement.scrollTop
    ceilShow.value = s > 400
  }
  window.addEventListener('scroll', onScroll, { passive: true })
  onScroll()
  onUnmounted(() => window.removeEventListener('scroll', onScroll))
  try {
    const r = await api.post<{ exists?: boolean; favorited?: boolean }>('/v1/mcenter/favorites/exists', {
      kind: 1,
      target_id: id,
    })
    fav.value = Boolean(r.exists || r.favorited)
  } catch {
    /* guest */
  }
  if (comMessageOn.value) await loadAskCaptcha()
})
async function apply() {
  applyMsg.value = ''
  if (jobClosed.value) {
    applyMsg.value = t('wap_com_00242')
    return
  }
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  if (me.value.usertype !== 1) {
    applyMsg.value = t('wap_00256')
    return
  }
  try {
    await api.post('/v1/mcenter/apply', { job_id: id })
    appliedLocal.value = true
    applyMsg.value = t('common.confirm')
  } catch (e: unknown) {
    applyMsg.value = e instanceof Error ? e.message : t('common.no')
  }
}
async function shareJob() {
  applyMsg.value = ''
  try {
    const r = await api.get<{
      plain_text?: string
      share_url?: string
      job_name?: string
    }>('/v1/wap/jobs/share-text', { id })
    const text = String(r.plain_text || '')
    const url = String(r.share_url || (import.meta.client ? window.location.href : ''))
    if (import.meta.client && navigator.share) {
      await navigator.share({ title: String(r.job_name || job.value.name || ''), text, url })
      return
    }
    if (import.meta.client && navigator.clipboard && text) {
      await navigator.clipboard.writeText(text)
      applyMsg.value = t('common.confirm')
      return
    }
    applyMsg.value = text || t('ui.load_failed')
  } catch (e: unknown) {
    applyMsg.value = e instanceof Error ? e.message : t('ui.load_failed')
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
  h5LinkOpen.value = true
  try {
    const r = await api.get<{
      linktel?: string
      linkphone?: string
      linkman?: string
      link_code?: number
      link_msg?: string
      revealed?: boolean
      prvlinktel?: string
      prvtime?: string
    }>('/v1/wap/jobs/contact', { id, isgetprv: linkCode.value === 10 ? 1 : 0 })
    await api.post('/v1/wap/jobs/tel-click', { id }).catch(() => undefined)
    const code = Number(r.link_code || 0)
    if (code === 10 && r.prvlinktel) {
      revealed.value = { linktel: r.prvlinktel, linkphone: r.prvlinktel, linkman: r.linkman }
      applyMsg.value = r.prvtime || ''
      return
    }
    if (code === 11) {
      applyMsg.value = r.link_msg && te(r.link_msg as never) ? t(r.link_msg as never) : t('common_00332')
      return
    }
    if (r.revealed && (r.linktel || r.linkphone)) {
      revealed.value = { linktel: r.linktel, linkphone: r.linkphone, linkman: r.linkman }
      return
    }
    if (code === 6) {
      await navigateTo('/login')
      return
    }
    const raw = String(r.link_msg || '')
    applyMsg.value = raw && (te(raw as never) ? t(raw as never) : raw)
  } catch (e: unknown) {
    applyMsg.value = e instanceof Error ? e.message : t('common.phone')
  }
}
async function report() {
  reportOpen.value = true
}
const salary = computed(() =>
  formatSalary(
    {
      id,
      name: String(job.value.name || ''),
      min_salary: Number(job.value.minsalary || job.value.min_salary || 0),
      max_salary: Number(job.value.maxsalary || job.value.max_salary || 0),
    },
    t('common.negotiable'),
    salaryType.value,
    t('common_01943'),
  ),
)
const welfare = computed(() => {
  const w = dict.value.welfare_names || job.value.welfare || job.value.job_welfare
  if (Array.isArray(w)) return w.map(String).filter(Boolean)
  if (typeof w === 'string') return w.split(/[,，]/).map((s) => s.trim()).filter(Boolean)
  return [] as string[]
})
const comAddress = computed(() => String(contactInfo.value.address || company.value.address || job.value.address || ''))
const loginDateN = computed(() => formatUnixDate(Number(company.value.login_date || 0)))
const isUrgent = computed(() => Number(userContext.value.job_urgent) === 1)
const isRec = computed(() => Number(userContext.value.job_rec) === 1)
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
      <div
        class="job_ceil"
        id="float"
        :style="ceilShow ? { position: 'fixed', top: '0px', display: 'block' } : undefined"
      >
        <div class="job_ceil_box">
          <div class="job_ceil_box_bg" />
          <div class="job_ceil_box_c">
            <div class="yun_content">
              <div class="job_ceil_cont">
                <span class="job_ceil_jobname">{{ job.name }}</span>
                <span class="job_ceil_jobxz">{{ salary }}</span>
                <a href="javascript:;" class="job_ceil_jobsc" @click.prevent="toggleFav">{{
                  fav ? $t('wap_00378') : $t('wap_00379')
                }}</a>
                <template v-if="!jobClosed">
                <a
                  v-if="applyCta.kind !== 'apply'"
                  class="job_ceil_jobtd_ysq"
                >{{ applyCta.label }}</a>
                <a
                  v-else
                  href="javascript:;"
                  class="job_ceil_jobtd"
                  @click.prevent="apply"
                >{{ applyCta.label }}</a>
                </template>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="clear" />
      <div class="job_details_top">
        <div class="w1200">
          <div class="job_details_current">
            {{ $t('common_01498') }}：<NuxtLink to="/">{{ $t('common.home') }}</NuxtLink> >
            <NuxtLink to="/jobs">{{ $t('default_00236') }}</NuxtLink> > {{ $t('wap_00287') }}
          </div>
          <div class="job_details_topbox">
            <div class="job_details_topleft">
              <h1 class="job_details_name">{{ job.name }}</h1>
              <span class="job_details_salary_n">{{ salary }}</span>
              <span v-if="isUrgent" class="showtg_icon showjp" />
              <span v-if="isRec" class="showtg_icon showzt" />
              <div class="job_details_info">
                <template v-if="cityLabel">{{ cityLabel }}</template>
                <template v-if="expLabel">
                  <span class="job_details_line">|</span>{{ expLabel }}
                </template>
                <template v-if="eduLabel">
                  <span class="job_details_line">|</span>{{ eduLabel }}
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
              <img
                v-if="jobClosed"
                src="/legacy/pc/images/stamp.png"
                :alt="$t('wap_com_00242')"
              />
              <template v-else>
              <div class="job_details_top_operation">
                <a href="javascript:;" class="job_details_top_operation_sc" @click.prevent="toggleFav">{{
                  fav ? $t('wap_00378') : $t('wap_00379')
                }}</a>
                <a
                  v-if="applyCta.kind !== 'apply'"
                  class="job_details_top_operation_ysq"
                >{{ applyCta.label }}</a>
                <a
                  v-else
                  href="javascript:;"
                  class="job_details_top_operation_sq"
                  @click.prevent="apply"
                >{{ applyCta.label }}</a>
              </div>
              <div class="job_details_top_extension">
                <div class="job_details_top_extension_zl">
                  <a href="javascript:;" class="job_details_top_extension_jb" @click.prevent="report">{{
                    $t('wap_com_00350')
                  }}</a>
                </div>
              </div>
              </template>
            </div>
          </div>
          <div v-if="adsBanner?.length" class="yun_jobbanner">
            <img v-for="(ad, i) in adsBanner" :key="i" :src="ad.image_n" alt="" />
          </div>
        </div>
      </div>
      <div class="clear" />
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
                  <span class="job_details_touch_username">{{
                    revealed?.linkman || contactInfo.linkman || job.com_name
                  }}</span>
                  <span v-if="company.linkjob" class="job_details_touch_userjob">{{ company.linkjob }}</span>
                </div>
                <div v-if="loginDateN" class="job_details_touch_userlogintime">
                  {{ $t('default_00364') }}{{ loginDateN }}
                </div>
                <div v-if="showSnum" class="job_details_touch_userdata">
                  <div class="job_details_touch_userdata_list">
                    <span class="job_details_touch_userdata_n">{{ applyStats.snum }}{{ $t('common_02052') }}</span>{{ $t('member_com_00152') }}
                  </div>
                  <div class="job_details_touch_userdata_list">
                    <i class="job_details_touch_userdata_list_line" />
                    <span class="job_details_touch_userdata_n">{{ applyStats.pre }}%</span>{{ $t('default_00227') }}
                  </div>
                  <div v-if="applyStats.operatime" class="job_details_touch_userdata_list">
                    <i class="job_details_touch_userdata_list_line" />
                    <span class="job_details_touch_userdata_n">{{ applyStats.operatime }}</span>{{ $t('default_00224') }}
                  </div>
                </div>
              </div>
              <template v-if="!jobClosed">
              <div v-if="linkCode === 10 || linkCode === 11" class="job_details_touch_tel">
                <em class="job_details_touch_tel_tip">{{
                  revealed?.linktel || linkMsg || (linkCode === 11 ? $t('common_00332') : $t('common_01934'))
                }}</em>
                <a
                  v-if="linkCode === 10"
                  href="javascript:;"
                  class="job_details_touch_tel_bth"
                  @click.prevent="showTel"
                >{{ $t('default_00233') }}</a>
              </div>
              <div v-else-if="linkCode === 9" class="job_details_touch_tel">
                <em class="job_details_touch_tel_tip">{{ linkMsg || $t('common_02372') }}</em>
              </div>
              <div v-else-if="linkCode > 1 && linkCode < 6" class="job_details_touch_tel">
                <em class="job_details_touch_tel_tip">{{ linkMsg }}</em>
                <a
                  v-if="applyCta.kind === 'apply'"
                  href="javascript:;"
                  class="job_details_touch_tel_bth"
                  @click.prevent="apply"
                >{{ applyCta.label }}</a>
                <em v-else class="job_details_touch_tel_tip">{{ applyCta.label }}</em>
              </div>
              <div v-else class="job_details_touch_tel">
                {{ $t('common.phone') }}：
                <span class="job_details_touch_tel_n">{{ telDisplay || '****' }}</span>
                <template v-if="linkCode === 6">
                  <em class="job_details_touch_tel_tip">{{ linkMsg }}{{ $t('wap_00264') }}</em>
                  <NuxtLink to="/login" class="job_details_touch_tel_bth">{{ $t('default_00234') }}</NuxtLink>
                </template>
                <template v-else-if="linkCode === 7">
                  <em class="job_details_touch_tel_tip">{{ linkMsg || $t('default_00203') }}</em>
                  <NuxtLink to="/user/resume" class="job_details_touch_tel_bth">{{ $t('wap_user_00197') }}</NuxtLink>
                </template>
                <template v-else-if="linkCode === 8">
                  <em class="job_details_touch_tel_tip">{{ $t('default_00204') }}</em>
                  <a
                    v-if="applyCta.kind === 'apply'"
                    href="javascript:;"
                    class="job_details_touch_tel_bth"
                    @click.prevent="apply"
                  >{{ applyCta.label }}</a>
                  <em v-else class="job_details_touch_tel_tip">{{ applyCta.label }}</em>
                </template>
                <template v-else>
                  <a href="javascript:;" class="job_details_touch_tel_bth" @click.prevent="showTel">{{
                    $t('default_00233')
                  }}</a>
                  <span class="job_details_touch_tel_say">{{ $t('member_com_00024') }}{{ siteName }}{{ $t('wap_00240') }}</span>
                </template>
              </div>
              </template>
              <span v-if="comAddress" class="job_details_touch_add">
                {{ $t('wap_js_00082') }}：{{ comAddress }}
                <NuxtLink v-if="mapHref" :to="mapHref" class="job_details_touch_tel_bth">{{ $t('wap_00223') }}</NuxtLink>
              </span>
            </div>
            <div class="job_details_tit" style="margin-top: 20px">
              <span class="job_details_tit_s">{{ $t('wap_com_00289') }}</span>
              <i class="job_details_tit_line" />
            </div>
            <div class="job_details_describe">
              <span v-for="item in yqItems" :key="item.label + item.value" class="job_details_describe_yq">
                {{ item.label ? `${item.label}：` : '' }}{{ item.value }}
              </span>
              <div v-html="String(job.description || job.content || '')" />
              <div v-if="shenming" class="job_details_tip">{{ $t('common_02136') }}：{{ shenming }}</div>
            </div>
          </div>
          <div class="job_details_left_box">
            <div class="job_details_tit">
              <span class="job_details_tit_s">{{ $t('common_02373') }}</span>
              <i class="job_details_tit_line" />
            </div>
            <div class="job_details_com_otherjob">
              <ul>
                <li v-for="row in sameCom || []" :key="row.id">
                  <div class="job_details_com_otherjob_l">
                    <div class="job_details_com_otherjob_name">
                      <NuxtLink :to="`/jobs/${row.id}`" :title="row.name">{{ row.name }}</NuxtLink>
                    </div>
                    <div class="job_details_com_otherjob_info">
                      <template v-if="row.exp_n">{{ dictReqLabel(String(row.exp_n), $t('home.experience_suffix')) }}</template>
                      <span v-if="row.exp_n && row.edu_n" class="job_details_line">|</span>
                      <template v-if="row.edu_n">{{ dictReqLabel(String(row.edu_n), $t('home.education_suffix')) }}</template>
                    </div>
                  </div>
                  <div class="job_details_com_otherjob_c">
                    <div class="job_details_com_otherjob_xz">{{ formatSalary(row, $t('common.negotiable'), salaryType, $t('common_01943')) }}</div>
                    <div class="job_details_com_otherjob_city">{{ row.job_city_one }} - {{ row.job_city_two }}</div>
                  </div>
                  <div class="job_details_com_otherjob_r">
                    <div class="job_details_com_otherjob_time">{{ row.lastupdate_n }}</div>
                    <NuxtLink :to="`/jobs/${row.id}`" class="job_details_com_otherjob_sq">{{ $t('wap_00574') }}</NuxtLink>
                  </div>
                </li>
              </ul>
              <div v-if="!(sameCom || []).length" class="evaluate_pj_no">{{ $t('default_00216') }}</div>
            </div>
          </div>
          <div class="job_details_left_box">
            <div class="job_details_tit">
              <span class="job_details_tit_s">{{ $t('default_00218') }}</span>
              <i class="job_details_tit_line" />
              <NuxtLink to="/jobs" class="job_details_more">{{ $t('default_00367') }}</NuxtLink>
            </div>
            <div class="job_details_like">
              <ul>
                <li v-for="row in similarList" :key="row.id">
                  <div class="job_details_likejobname">
                    <NuxtLink :to="`/jobs/${row.id}`" :title="row.name">{{ row.name }}</NuxtLink>
                  </div>
                  <div class="job_details_likejobxz">{{ formatSalary(row, $t('common.negotiable'), salaryType, $t('common_01943')) }}</div>
                  <div class="job_details_likecomname">
                    <NuxtLink v-if="row.uid" :to="`/companies/${row.uid}`">{{ row.com_name }}</NuxtLink>
                  </div>
                  <NuxtLink :to="`/jobs/${row.id}`" class="job_details_likesq">{{ $t('wap_00574') }}</NuxtLink>
                </li>
              </ul>
            </div>
          </div>
        </div>
        <div class="job_details_right">
          <div v-if="Number(company.fact_status) === 1" class="yunnew_hyboxpv">
            <div class="yunnew_hybox">
              <i class="yunnew_hyboxicon" />
              <span class="yunnew_hyboxname">{{ $t('wap_00275') }}</span>
            </div>
          </div>
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
            <div class="job_details_cominfo_tb">
              <i v-if="Number(company.yyzz_status) === 1" class="job_details_cominfo_rz job_details_cominfo_rz_zz" />
              <i v-if="Number(company.moblie_status) === 1" class="job_details_cominfo_rz job_details_cominfo_rz_sj" />
              <i v-if="Number(company.email_status) === 1" class="job_details_cominfo_rz job_details_cominfo_rz_yx" />
            </div>
            <div class="Compply_right_js">
              <ul>
                <li v-if="hyLabel">
                  <span class="Compply_right_span_c"><i class="Compply_right_icon Compply_right_icon_hy" />{{ hyLabel }}</span>
                </li>
                <li v-if="prLabel">
                  <span class="Compply_right_span_c"><i class="Compply_right_icon Compply_right_icon_xz" />{{ prLabel }}</span>
                </li>
                <li v-if="munLabel">
                  <span class="Compply_right_span_c"><i class="Compply_right_icon Compply_right_icon_rs" />{{ munLabel }}</span>
                </li>
              </ul>
            </div>
            <div v-if="company.content" class="job_details_cominfo_p" v-html="String(company.content)" />
            <div class="job_details_cominfo_more_bth">
              <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`" class="job_details_cominfo_more">{{
                $t('common_01689')
              }} ></NuxtLink>
            </div>
            <div v-if="comMessageOn" class="job_details_right_box">
              <div class="job_details_tit">
                <span class="job_details_tit_s">{{ $t('common_02375') }}</span>
                <i class="job_details_tit_line" />
              </div>
              <p v-if="!msgList.length" class="job_details_comask_p">{{ $t('default_00213') }}</p>
              <div v-for="m in msgList" :key="'pc' + String(m.id)" class="yun_newedition_asklist">
                <div class="yun_newedition_showask">{{ m.content }}</div>
                <div class="yun_newedition_showand">{{ m.reply || $t('wap_01553') }}</div>
              </div>
              <div class="job_hr_ly_box" style="padding-top: 10px">
                <textarea v-model="askContent" class="comapply_Leave_fb_text" :placeholder="$t('default_00201')" />
                <div class="affirm affirm_yz" style="display: flex; gap: 8px; align-items: center; margin-top: 8px">
                  <input v-model="askCode" class="zx_yx_input" :placeholder="$t('wap_user_00143')" maxlength="4" />
                  <img
                    v-if="askCaptcha?.image"
                    :src="askCaptcha.image"
                    class="zx_yx_input_img"
                    alt=""
                    @click="loadAskCaptcha"
                  />
                  <button type="button" class="comapply_Leave_fb_sub" @click="postAsk">{{ $t('wap_00280') }}</button>
                </div>
                <p v-if="askMsg" class="muted">{{ askMsg }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="site-h5">
      <div class="min_body">
        <div class="job_describe">
          <div class="job_describe_top">
            <div class="new_jobshowtop">
              <div v-if="jobClosed" class="job_yxj">
                <img src="/legacy/h5/images/stamp.png" :alt="$t('wap_com_00242')" />
              </div>
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
              <span v-if="showSnum">{{ $t('wap_01587') }} {{ applyStats.snum }}{{ $t('common_02052') }}</span>
              <a href="javascript:;" @click.prevent="toggleFav">{{
                fav ? $t('wap_00378') : $t('wap_00379')
              }}</a>
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
              <li v-for="item in yqItems" :key="item.label + item.value">
                {{ item.label ? `${item.label}：` : '' }}{{ item.value }}
              </li>
            </ul>
            <div class="newjob_js" v-html="String(job.description || job.content || '')" />
          </div>
        </div>
        <div v-if="adsH5?.length" class="jobshow_ad">
          <a v-for="(ad, i) in adsH5" :key="'512-' + i" :href="ad.link || undefined">
            <img v-if="ad.image_n || ad.image" :src="ad.image_n || ad.image" alt="" style="width: 100%" />
          </a>
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
        <div v-if="comMessageOn" class="company_questions">
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
          <div v-if="!msgList.length" class="jobshow_tw_box">
            <p class="muted">{{ $t('wap_01555') }}</p>
          </div>
          <div class="job_tcktextarea" style="padding: 0.24rem">
            <textarea v-model="askContent" class="mt10" :placeholder="$t('default_00201')" />
            <div class="job_tckyzmbox" style="display: flex; gap: 0.16rem; align-items: center; margin-top: 0.16rem">
              <input v-model="askCode" :placeholder="$t('wap_user_00143')" maxlength="6" />
              <img
                v-if="askCaptcha?.image"
                :src="askCaptcha.image"
                alt=""
                style="height: 0.8rem"
                @click="loadAskCaptcha"
              />
            </div>
            <button type="button" class="job_tckbth" @click="postAsk">{{ $t('wap_00280') }}</button>
            <p v-if="askMsg" class="muted">{{ askMsg }}</p>
          </div>
        </div>
        <div class="recommend_post" style="margin-top: 0">
          <div class="recommend_post_header" style="margin: 0.4rem 0">{{ $t('wap_00282') }}</div>
          <div class="recommend_post_card_box">
            <div v-for="row in similarList" :key="row.id" class="recommend_post_card">
              <NuxtLink :to="`/jobs/${row.id}`" :title="row.name">
                <div class="recommend_post_card_top">
                  <div class="recommend_post_card_name">{{ row.name }}</div>
                  <div class="recommend_post_card_money">{{ formatSalary(row, $t('common.negotiable'), salaryType, $t('common_01943')) }}</div>
                </div>
                <div class="newjob_info">
                  <span>{{ row.job_city_one }}{{ row.job_city_two ? `-${row.job_city_two}` : '' }}</span>
                  <template v-if="row.edu_n">
                    <i class="newjob_info_line" /><span>{{ dictReqLabel(String(row.edu_n), $t('home.education_suffix')) }}</span>
                  </template>
                  <template v-if="row.exp_n">
                    <i class="newjob_info_line" /><span>{{ dictReqLabel(String(row.exp_n), $t('home.experience_suffix')) }}</span>
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
      <p v-if="applyMsg" class="muted" style="padding: 0.24rem">{{ applyMsg }}</p>
      <div v-if="!jobClosed" class="yun_czfoot">
        <div class="yun_czfootfixed">
          <div class="yun_czfoot_c">
            <div class="yun_czfoot_l">
              <NuxtLink to="/" class="yun_czfoot_s">
                <div class="yun_czfoot_s_p yun_czfoot_hmicon">{{ $t('common.home') }}</div>
              </NuxtLink>
              <a href="javascript:;" class="yun_czfoot_s" @click.prevent="shareJob">
                <div class="yun_czfoot_s_p yun_czfoot_scicon">{{ $t('common.share') }}</div>
              </a>
            </div>
            <a
              v-if="applyCta.kind !== 'apply'"
              class="yun_czfoot_s"
            >
              <div class="yun_czfoot_s_p yun_czfoot_ytdicon">{{ applyCta.label }}</div>
            </a>
            <a
              v-else
              href="javascript:;"
              class="yun_czfoot_s"
              @click.prevent="apply"
            >
              <div class="yun_czfoot_s_p yun_czfoot_jlicon">{{ applyCta.label }}</div>
            </a>
            <a href="javascript:;" class="yun_czfoot_s" @click.prevent="showTel">
              <div class="yun_czfoot_s_p">{{ $t('common.phone') }}</div>
            </a>
          </div>
        </div>
      </div>
      <div v-if="h5LinkOpen" class="new_jobshow_telbox" style="position: fixed; left: 0.4rem; right: 0.4rem; bottom: 1.4rem; z-index: 80; background: #fff; border-radius: 0.16rem; padding: 0.32rem; box-shadow: 0 4px 16px rgba(0,0,0,.12)">
        <div class="new_jobshow_leftname">{{ $t('member_com_00024') }}</div>
        <div v-if="revealed?.linktel || revealed?.linkphone">
          <div>{{ revealed?.linkman || contactInfo.linkman }}</div>
          <a v-if="revealed?.linktel" :href="`tel:${revealed.linktel}`">{{ revealed.linktel }}</a>
          <a v-else-if="revealed?.linkphone" :href="`tel:${revealed.linkphone}`">{{ revealed.linkphone }}</a>
        </div>
        <div v-else-if="linkCode === 10 || linkCode === 11" class="new_jobshow_tel">
          {{ revealed?.linktel || linkMsg || (linkCode === 11 ? $t('common_00332') : $t('common_01934')) }}
          <a v-if="linkCode === 10 && !revealed?.linktel" href="javascript:;" @click.prevent="showTel">{{
            $t('default_00233')
          }}</a>
        </div>
        <div v-else-if="linkCode === 9" class="new_jobshow_tel">
          {{ linkMsg || $t('common_02372') }}
        </div>
        <div v-else class="new_jobshow_tel">{{ applyMsg || linkMsg || telDisplay }}</div>
        <a href="javascript:;" class="new_jobshow_telbth" @click.prevent="h5LinkOpen = false">{{ $t('common.close') }}</a>
      </div>
    </div>
    <ReportSheet
      v-if="reportOpen"
      :target-kind="1"
      :target-id="id"
      @close="reportOpen = false"
      @done="applyMsg = $t('common.confirm')"
    />
  </div>
  <div v-else class="site-inner">
    <h1>{{ $t('common.job') }}</h1>
    <p class="muted">{{ error ? $t('ui.load_failed') : $t('default_00033') }}</p>
  </div>
</template>
