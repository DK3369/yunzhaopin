<script setup lang="ts">
import { listFailMsg, mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '~/utils/site'

const route = useRoute()
const { t, te, locale } = useI18n()
const { settings, me } = useSiteChrome()
const uid = Number(route.params.uid)
const tab = computed(() => String(route.query.tab || 'jobs'))
const api = useApi()
const comMessageOn = computed(() => String(settings.value.com_message || '') === '1')
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
    api
      .get<{ list: JobLike[]; total?: number }>('/v1/wap/companies/jobs', {
        uid,
        page: 1,
        page_size: 5,
      })
      .catch(() => ({ list: [] as JobLike[], total: 0 })),
)
const extraJobs = ref<JobLike[]>([])
const jobPage = ref(1)
const jobList = computed(() => [...(jobs.value?.list || []), ...extraJobs.value])
const jobTotal = computed(() => Number(jobs.value?.total || jobList.value.length))
async function loadMoreJobs() {
  jobPage.value += 1
  const r = await api.get<{ list: JobLike[]; total?: number }>('/v1/wap/companies/jobs', {
    uid,
    page: jobPage.value,
    page_size: 5,
  })
  extraJobs.value = [...extraJobs.value, ...(r.list || [])]
  if (r.total != null) {
    /* keep first-page total */
  }
}
const failMsg = computed(() => listFailMsg(error.value, t('common_00376'), t('common_00376')))
const following = ref(false)
const followMsg = ref('')
const revealed = ref<{ linktel?: string; linkphone?: string; linkman?: string } | null>(null)
const contact = computed(
  () => (company.value.contact || {}) as Record<string, unknown>,
)
const linkCode = computed(() => Number(contact.value.link_code || 0))
const mapHref = computed(() => {
  const x = String(company.value.x || '')
  const y = String(company.value.y || '')
  if (x && y) return `/map?x=${encodeURIComponent(x)}&y=${encodeURIComponent(y)}`
  return ''
})
const moneyLabel = computed(() => {
  const n = Number(company.value.money || 0)
  if (!n) return ''
  const unit = Number(company.value.moneytype) === 1 ? t('wap_js_00004') : t('wap_js_00002')
  return `${t('company_00023')}${n}${unit}`
})
const telDisplay = computed(
  () =>
    revealed.value?.linktel
    || revealed.value?.linkphone
    || String(contact.value.linktel_n || contact.value.linkphone_n || ''),
)
const linkMsg = computed(() => {
  const raw = String(contact.value.link_msg || '')
  if (!raw) return ''
  if (/^[a-z][a-z0-9_]*_\d+$/i.test(raw) || /^[a-z][a-z0-9_.]+$/i.test(raw)) {
    const key = raw as never
    return te(key) ? t(key) : raw
  }
  return raw
})
async function showTel() {
  if (linkCode.value === 6) {
    await navigateTo('/login')
    return
  }
  if (linkCode.value === 7) {
    await navigateTo('/user/resume')
    return
  }
  if (linkCode.value === 8) {
    await goCompanyJobs()
    return
  }
  try {
    await api.post('/v1/wap/jobs/tel-click', { id: 0, com_id: uid }).catch(() => undefined)
    const r = await api.get<{
      linktel?: string
      linkphone?: string
      linkman?: string
      revealed?: boolean
      link_code?: number
      prvlinktel?: string
      prvtime?: string
      link_msg?: string
    }>('/v1/wap/companies/contact', { uid, isgetprv: linkCode.value === 10 ? 1 : 0 })
    const code = Number(r.link_code || 0)
    if (code === 10 && r.prvlinktel) {
      revealed.value = { linktel: r.prvlinktel, linkphone: r.prvlinktel, linkman: r.linkman }
      if (r.prvtime) followMsg.value = r.prvtime
      return
    }
    if (code === 11) {
      followMsg.value = r.link_msg && te(r.link_msg as never) ? t(r.link_msg as never) : t('common_00332')
      return
    }
    if (r.revealed && (r.linktel || r.linkphone)) {
      revealed.value = { linktel: r.linktel, linkphone: r.linkphone, linkman: r.linkman }
    }
  } catch {
    await navigateTo('/login')
  }
}
async function goCompanyJobs() {
  if (tab.value !== 'jobs') {
    await navigateTo({ query: { ...route.query, tab: 'jobs' } })
  }
  if (import.meta.client) {
    document.getElementById('company_job_list')?.scrollIntoView({ behavior: 'smooth' })
  }
}
const { data: news } = await useAsyncData(
  () => `company-news-${locale.value}-${uid}`,
  () =>
    api
      .post<{ list: Array<Record<string, unknown>> }>('/v1/wap/companies/news', { uid, page: 1, page_size: 8 })
      .catch(() => ({ list: [] as Array<Record<string, unknown>> })),
)
const { data: products } = await useAsyncData(
  () => `company-products-${locale.value}-${uid}`,
  () =>
    api
      .post<{ list: Array<Record<string, unknown>> }>('/v1/wap/companies/products', {
        uid,
        page: 1,
        page_size: 8,
      })
      .catch(() => ({ list: [] as Array<Record<string, unknown>> })),
)
const newsList = computed(() => news.value?.list || [])
const productList = computed(() => products.value?.list || [])
const { data: msgs } = await useAsyncData(
  () => `company-msgs-${locale.value}-${uid}`,
  () =>
    api
      .post<{ list: Array<Record<string, unknown>> }>('/v1/wap/companies/messages', { uid })
      .catch(() => ({ list: [] as Array<Record<string, unknown>> })),
)
const msgList = computed(() => msgs.value?.list || [])
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
    await api.post('/v1/wap/companies/messages/post', {
      uid,
      content: askContent.value,
      captcha_cid: askCaptcha.value?.cid,
      authcode: askCode.value,
    })
    askContent.value = ''
    askCode.value = ''
    askMsg.value = t('common.confirm')
    await loadAskCaptcha()
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00376')
    await loadAskCaptcha()
  }
}
onMounted(async () => {
  if (comMessageOn.value) await loadAskCaptcha()
})
watch(
  () => company.value.isatn,
  (v) => {
    following.value = Number(v) === 1
  },
  { immediate: true },
)
async function toggleFollow() {
  followMsg.value = ''
  try {
    const r = await api.post<{ following?: boolean }>('/v1/mcenter/follows', {
      target_kind: 2,
      target_uid: uid,
    })
    following.value = Boolean(r.following)
  } catch (e: unknown) {
    followMsg.value = e instanceof Error ? e.message : t('common.no')
    await navigateTo('/login')
  }
}
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
                <i v-if="Number(company.yyzz_status) === 1" class="job_details_cominfo_rz job_details_cominfo_rz_zz" />
                <i v-if="Number(company.moblie_status) === 1" class="job_details_cominfo_rz job_details_cominfo_rz_sj" />
                <i v-if="Number(company.email_status) === 1" class="job_details_cominfo_rz job_details_cominfo_rz_yx" />
              </h1>
              <div class="com_details_info">
                <template v-if="company.city_one">{{ company.city_one }}</template>
                <template v-if="company.city_two"> - {{ company.city_two }}</template>
                <span v-if="company.hy_n" class="com_details_line">|</span>{{ company.hy_n }}
                <span v-if="company.pr_n" class="com_details_line">|</span>{{ company.pr_n }}
                <span v-if="company.mun_n" class="com_details_line">|</span>{{ company.mun_n }}
                <span v-if="company.sdate" class="com_details_line">|</span>
                <template v-if="company.sdate">{{ company.sdate }}</template>
                <span v-if="moneyLabel" class="com_details_line">|</span>
                <template v-if="moneyLabel">{{ moneyLabel }}</template>
                <span v-if="company.pre != null" class="com_details_line">|</span>
                <template v-if="company.pre != null">{{ company.pre }}% {{ $t('company_00006') }}</template>
              </div>
              <div class="com_details_data_box">
                <div class="com_details_data_box_c">
                  <div class="com_details_data">
                    <div class="com_details_data_n">{{ company.zp_num ?? jobs?.list?.length ?? 0 }}</div>
                    <div class="com_details_dataname">{{ $t('wap_00190') }}</div>
                    <i class="com_details_data_line" />
                  </div>
                  <div class="com_details_data">
                    <div class="com_details_data_n">{{ company.invite_resume ?? 0 }}</div>
                    <div class="com_details_dataname">{{ $t('company_00009') }}</div>
                    <i class="com_details_data_line" />
                  </div>
                  <div v-if="company.pre != null" class="com_details_data">
                    <div class="com_details_data_n">{{ company.pre }}%</div>
                    <div class="com_details_dataname">{{ $t('company_00006') }}</div>
                    <i class="com_details_data_line" />
                  </div>
                  <div class="com_details_data">
                    <div class="com_details_data_n">{{ company.login_date_n || $t('admin_user_00139') }}</div>
                    <div class="com_details_dataname">{{ $t('admin_yunying_00131') }}</div>
                  </div>
                </div>
              </div>
            </div>
            <div class="com_details_opt">
              <div class="com_details_opt_fxbox">
                <a href="javascript:;" class="com_details_opt_gz" :class="{ company_att: following }" @click.prevent="toggleFollow">
                  {{ following ? $t('wap_js_00140') : `+ ${$t('common_01949')}` }}
                </a>
              </div>
              <p v-if="followMsg" class="muted">{{ followMsg }}</p>
              <p v-if="Number(company.claimable) === 1">
                <NuxtLink :to="`/claim?uid=${uid}`">{{ $t('resume_00011') }}</NuxtLink>
              </p>
            </div>
          </div>
        </div>
      </div>
      <div class="w1200">
        <div class="com_details_left">
          <div v-if="welfare.length" class="com_show_leftbox">
            <div class="com_details_tit">
              <span class="com_details_tit_s">{{ $t('company_00007') }}</span>
              <i class="com_details_tit_line yun_bg_color" />
            </div>
            <div class="com_welfare">
              <span v-for="w in welfare" :key="w" class="com_welfare_s">{{ w }}</span>
            </div>
          </div>
          <div class="com_show_leftbox">
            <div class="com_details_tit">
              <span class="com_details_tit_s">{{ $t('wap_com_00168') }}</span>
              <i class="com_details_tit_line yun_bg_color" />
            </div>
            <div class="com_show_leftcont">
              <div v-if="company.content" class="con_show_introduction company_img_auto" v-html="String(company.content)" />
              <div v-else class="firm_ment">
                <div class="firm_tips_no">{{ $t('wap_00028') }}</div>
              </div>
            </div>
          </div>
          <div v-if="shows.length" class="com_show_leftbox">
            <div class="com_details_tit">
              <span class="com_details_tit_s">{{ $t('company_00008') }}</span>
              <i class="com_details_tit_line yun_bg_color" />
            </div>
            <div class="com_show_image" id="layer-pic">
              <div v-for="s in shows" :key="String(s.id)" class="com_show_image_list">
                <img :src="mediaUrl(String(s.picurl || ''), PLACEHOLDER_LOGO)" width="260" height="160" alt="" />
              </div>
            </div>
          </div>
          <div class="com_show_leftbox">
            <div class="com_details_tit">
              <span class="com_details_tit_s">{{ $t('wap_00462') }}</span>
              <i class="com_details_tit_line yun_bg_color" />
            </div>
            <div class="firm_det_link">
              <span v-if="contact.linkman" class="firm_mes1">{{ $t('common_02051') }}：{{ contact.linkman }}</span>
              <span v-if="company.linkjob" class="firm_mes1">{{ $t('common_01637') }}：{{ company.linkjob }}</span>
              <div v-if="linkCode === 10" class="firm_login_con">
                {{ revealed?.linktel || linkMsg || $t('common_01934') }}
                <a href="javascript:;" class="job_details_touch_tel_bth" @click.prevent="showTel">{{
                  $t('default_00233')
                }}</a>
              </div>
              <div v-else-if="linkCode === 11" class="firm_login_con">{{ linkMsg || $t('common_00332') }}</div>
              <div v-else-if="linkCode === 9" class="firm_login_con">
                {{ linkMsg || $t('common_02372') }}
              </div>
              <div v-else-if="linkCode > 1 && linkCode < 6" class="firm_login_con">{{ linkMsg }}</div>
              <div v-else class="firm_mes1">
                {{ $t('common.phone') }}：{{ telDisplay || '****' }}
                <template v-if="linkCode === 6">
                  <NuxtLink to="/login" class="firm_login_dl">{{ $t('common.login') }}</NuxtLink>
                </template>
                <template v-else-if="linkCode === 7">
                  <em>{{ linkMsg || $t('default_00203') }}</em>
                  <NuxtLink to="/user/resume" class="firm_login_dl">{{ $t('wap_user_00197') }}</NuxtLink>
                </template>
                <template v-else-if="linkCode === 8">
                  <em>{{ $t('default_00204') }}</em>
                  <a href="javascript:;" class="job_details_touch_tel_bth" @click.prevent="goCompanyJobs">{{
                    $t('wap_00190')
                  }}</a>
                </template>
                <a v-else href="javascript:;" class="job_details_touch_tel_bth" @click.prevent="showTel">{{
                  $t('default_00233')
                }}</a>
              </div>
              <span v-if="company.address" class="firm_mes1" style="width: 100%">
                {{ $t('wap_00040') }}：
                <NuxtLink v-if="mapHref" :to="mapHref">{{ company.address }}</NuxtLink>
                <template v-else>{{ company.address }}</template>
              </span>
              <span v-if="company.website" class="firm_mes1">{{ $t('wap_com_00162') }}：{{ company.website }}</span>
            </div>
          </div>
          <div v-if="comMessageOn" class="com_show_leftbox">
            <div class="com_details_tit">
              <span class="com_details_tit_s">{{ $t('wap_00271') }}</span>
              <i class="com_details_tit_line yun_bg_color" />
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
          <div v-if="newsList.length" class="com_show_leftbox">
            <div class="com_details_tit">
              <span class="com_details_tit_s">{{ $t('company_00019') }}</span>
              <i class="com_details_tit_line yun_bg_color" />
            </div>
            <ul class="black_newslist">
              <li v-for="n in newsList" :key="String(n.id)">
                <NuxtLink :to="`/companies/${uid}/news/${n.id}`">{{ n.title }}</NuxtLink>
              </li>
            </ul>
          </div>
          <div v-if="productList.length" class="com_show_leftbox">
            <div class="com_details_tit">
              <span class="com_details_tit_s">{{ $t('company_00020') }}</span>
              <i class="com_details_tit_line yun_bg_color" />
            </div>
            <div class="com_show_image">
              <div v-for="p in productList" :key="String(p.id)" class="com_show_image_list">
                <NuxtLink :to="`/companies/${uid}/products/${p.id}`">
                  <img :src="mediaUrl(String(p.cover_n || p.cover || ''), PLACEHOLDER_LOGO)" width="200" height="127" alt="" />
                  <p>{{ p.title }}</p>
                </NuxtLink>
              </div>
            </div>
          </div>
          <div class="com_show_leftbox">
            <div class="com_details_tit">
              <span class="com_details_tit_s">{{ $t('wap_00190') }}</span>
              <i class="com_details_tit_line yun_bg_color" />
            </div>
            <div id="company_job_list" class="comshow_job">
              <JobCard v-for="job in jobList" :key="job.id" :job="job" variant="firm" />
              <div v-if="!jobList.length" class="firm_tips_no">{{ $t('common_02402') }}</div>
              <a
                v-if="jobList.length < jobTotal"
                href="javascript:;"
                class="job_details_touch_tel_bth"
                @click.prevent="loadMoreJobs"
              >{{ $t('common.more') }}</a>
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
            <div class="com_details_data_box">
              <div class="com_details_data_box_c">
                <div class="com_details_data">
                  <div class="com_details_data_n">{{ company.zp_num ?? jobs?.list?.length ?? 0 }}</div>
                  <div class="com_details_dataname">{{ $t('wap_00190') }}</div>
                </div>
                <div class="com_details_data">
                  <div class="com_details_data_n">{{ company.invite_resume ?? 0 }}</div>
                  <div class="com_details_dataname">{{ $t('company_00009') }}</div>
                </div>
                <div v-if="company.pre != null" class="com_details_data">
                  <div class="com_details_data_n">{{ company.pre }}%</div>
                  <div class="com_details_dataname">{{ $t('company_00006') }}</div>
                </div>
                <div class="com_details_data">
                  <div class="com_details_data_n">{{ company.login_date_n || $t('admin_user_00139') }}</div>
                  <div class="com_details_dataname">{{ $t('admin_yunying_00131') }}</div>
                </div>
              </div>
            </div>
            <p v-if="company.zp_num != null" class="muted">{{ $t('wap_00185') }} {{ company.zp_num }}</p>
            <p v-if="moneyLabel" class="muted">{{ moneyLabel }}</p>
            <p v-if="company.address" class="muted">
              <NuxtLink v-if="mapHref" :to="mapHref">{{ company.address }}</NuxtLink>
              <template v-else>{{ company.address }}</template>
            </p>
            <a href="javascript:;" class="com_details_opt_gz" @click.prevent="toggleFollow">
              {{ following ? $t('wap_js_00140') : $t('common_01949') }}
            </a>
            <p v-if="Number(company.claimable) === 1">
              <NuxtLink :to="`/claim?uid=${uid}`">{{ $t('resume_00011') }}</NuxtLink>
            </p>
          </div>
        </div>
      </div>
      <div class="phpyuncomnav">
        <ul>
          <li>
            <NuxtLink :to="{ query: { tab: 'jobs' } }" :class="{ colorshow: tab !== 'about' }">{{
              $t('wap_00190')
            }}</NuxtLink>
            <span class="phpyunjobn">{{ company.zp_num ?? jobTotal }}</span>
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
            <NuxtLink v-if="mapHref" :to="mapHref" class="newcom_add_dz">{{ company.address }}</NuxtLink>
            <div v-else class="newcom_add_dz">{{ company.address }}</div>
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
        <div class="job_describe_cengter_header">{{ $t('wap_00462') }}</div>
        <div class="newcom_add">
          <div v-if="contact.linkman">{{ contact.linkman }}</div>
          <div v-if="linkCode === 10">
            {{ revealed?.linktel || linkMsg || $t('common_01934') }}
            <a href="javascript:;" @click.prevent="showTel">{{ $t('default_00233') }}</a>
          </div>
          <div v-else-if="linkCode === 11">{{ linkMsg || $t('common_00332') }}</div>
          <div v-else-if="linkCode === 9">
            {{ linkMsg || $t('common_02372') }}
          </div>
          <div v-else-if="linkCode > 1 && linkCode < 6">{{ linkMsg }}</div>
          <div v-else>
            {{ $t('common.phone') }}：{{ telDisplay || '****' }}
            <template v-if="linkCode === 6">
              <NuxtLink to="/login">{{ $t('common.login') }}</NuxtLink>
            </template>
            <template v-else-if="linkCode === 7">
              <NuxtLink to="/user/resume">{{ $t('wap_user_00197') }}</NuxtLink>
            </template>
            <template v-else-if="linkCode === 8">
              <a href="javascript:;" @click.prevent="goCompanyJobs">{{ $t('wap_00190') }}</a>
            </template>
            <a v-else href="javascript:;" @click.prevent="showTel">{{ $t('default_00233') }}</a>
          </div>
        </div>
        <div v-if="comMessageOn" class="job_describe_bottom">
          <div class="job_describe_cengter_header">{{ $t('wap_00271') }}</div>
          <div v-for="m in msgList" :key="String(m.id)">
            <p>{{ m.content }}</p>
            <p class="muted">{{ m.reply || $t('wap_01589') }}</p>
          </div>
          <p v-if="!msgList.length" class="muted">{{ $t('wap_01555') }}</p>
          <textarea v-model="askContent" class="mt10" :placeholder="$t('default_00201')" />
          <div style="display: flex; gap: 0.16rem; align-items: center; margin-top: 0.16rem">
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
        <div v-if="newsList.length" class="job_describe_bottom">
          <div class="job_describe_cengter_header">{{ $t('company_00019') }}</div>
          <div v-for="n in newsList" :key="String(n.id)">
            <NuxtLink :to="`/companies/${uid}/news/${n.id}`">{{ n.title }}</NuxtLink>
          </div>
        </div>
        <div v-if="productList.length" class="job_describe_bottom">
          <div class="job_describe_cengter_header">{{ $t('company_00020') }}</div>
          <div v-for="p in productList" :key="String(p.id)">
            <NuxtLink :to="`/companies/${uid}/products/${p.id}`">{{ p.title }}</NuxtLink>
          </div>
        </div>
        <div class="job_describe_cengter_header">{{ $t('wap_com_00168') }}</div>
        <div class="phpyunabout" v-html="String(company.content || '')" />
      </div>
      <div v-else id="company_job_list">
        <JobCard v-for="job in jobList" :key="job.id" :job="job" variant="com" />
        <div v-if="!jobList.length" class="wap_member_no">{{ $t('home.no_recruiting_jobs') }}</div>
        <a v-if="jobList.length < jobTotal" href="javascript:;" @click.prevent="loadMoreJobs">{{ $t('common.more') }}</a>
      </div>
    </div>
  </article>
  <article v-else class="site-inner">
    <h1>{{ $t('common.company') }}</h1>
    <p class="muted">{{ error ? failMsg : $t('wap_js_00113') }}</p>
  </article>
</template>
