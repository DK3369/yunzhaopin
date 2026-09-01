<script setup lang="ts">
import { catTree, formatSalary, listFailMsg, mediaUrl, PLACEHOLDER_LOGO, type CatNode, type CompanyLike, type JobLike } from '~/utils/site'

type Banner = { image_n?: string; image?: string; link?: string; title?: string; pic_content?: string }
type ArticleLike = {
  id: number
  title: string
  datetime_n?: string
  published_at_n?: string
  cover?: string
  picurl?: string
  category?: string
  name?: string
}
type FriendLink = { id: number; name: string; url: string; logo?: string; category?: string }

const api = useApi()
const { t } = useI18n()
const { siteName, me, h5Nav } = useSiteChrome()
const h5NavPage = ref(0)
const h5NavPages = computed(() => {
  const list = h5Nav.value || []
  const pages: typeof list[] = []
  for (let i = 0; i < list.length; i += 4) pages.push(list.slice(i, i + 4))
  return pages
})
watch(h5NavPages, (pages) => {
  if (h5NavPage.value >= pages.length) h5NavPage.value = 0
})

const { data: home, error } = await useAsyncData('home', async () => {
  const h = (await api.get('/v1/wap/home', { did: 0 })) as {
    hot_jobs?: JobLike[]
    rec_companies?: CompanyLike[]
    announcements?: unknown[]
    hot_keywords?: unknown[]
    new_articles?: ArticleLike[]
    featured_articles?: ArticleLike[]
    hot_articles?: ArticleLike[]
  }
  const [rec, latest, urgent] = await Promise.all([
    api.get<{ list: JobLike[] }>('/v1/wap/jobs', { rec: true, page_size: 8 }).catch(() => ({ list: [] as JobLike[] })),
    api.get<{ list: JobLike[] }>('/v1/wap/jobs', { page_size: 8 }).catch(() => ({ list: [] as JobLike[] })),
    api.get<{ list: JobLike[] }>('/v1/wap/jobs', { urgent: true, page_size: 8 }).catch(() => ({ list: [] as JobLike[] })),
  ])
  return {
    ...h,
    rec_jobs: rec.list || [],
    latest_jobs: latest.list || [],
    urgent_jobs: urgent.list || [],
  }
})
const { data: cats } = await useAsyncData('job-cats', () =>
  api.get<CatNode[]>('/v1/wap/categories', { kind: 'job' }).catch(() => [] as CatNode[]),
)
const { data: adsPc } = await useAsyncData('ads-3', () =>
  api.get<Banner[]>('/v1/wap/ads', { slot: '3', limit: 5 }).catch(() => [] as Banner[]),
)
const { data: adsH5 } = await useAsyncData('ads-50', () =>
  api.get<Banner[]>('/v1/wap/ads', { slot: '50', limit: 5 }).catch(() => [] as Banner[]),
)
const { data: adsMid } = await useAsyncData('ads-mid', async () => {
  const [slot13, slot14, slot15, slot72, slot73, slot92, slot503] = await Promise.all([
    api.get<Banner[]>('/v1/wap/ads', { slot: '13', limit: 3 }).catch(() => [] as Banner[]),
    api.get<Banner[]>('/v1/wap/ads', { slot: '14', limit: 3 }).catch(() => [] as Banner[]),
    api.get<Banner[]>('/v1/wap/ads', { slot: '15', limit: 3 }).catch(() => [] as Banner[]),
    api.get<Banner[]>('/v1/wap/ads', { slot: '72', limit: 1 }).catch(() => [] as Banner[]),
    api.get<Banner[]>('/v1/wap/ads', { slot: '73', limit: 1 }).catch(() => [] as Banner[]),
    api.get<Banner[]>('/v1/wap/ads', { slot: '92', limit: 5 }).catch(() => [] as Banner[]),
    api.get<Banner[]>('/v1/wap/ads', { slot: '503', limit: 3 }).catch(() => [] as Banner[]),
  ])
  return {
    slot13: slot13 || [],
    slot14: slot14 || [],
    slot15: slot15 || [],
    slot72: slot72 || [],
    slot73: slot73 || [],
    slot92: slot92 || [],
    slot503: slot503 || [],
  }
})
const { data: friendLinks } = await useAsyncData('home-links', () =>
  api.get<FriendLink[]>('/v1/wap/friend-links').catch(() => [] as FriendLink[]),
)
const { data: resumes, error: resumeError } = await useAsyncData('home-resumes', () =>
  api
    .get<{ list: Array<Record<string, unknown>> }>('/v1/wap/resumes', { page_size: 8, recg: true })
    .catch(() => ({ list: [] as Array<Record<string, unknown>> })),
)

const jobCats = computed(() => catTree(cats.value || [], 11))
const hotJobs = computed(() => (home.value?.hot_jobs || []) as JobLike[])
const latestJobList = computed(() => {
  const extra = (home.value as { latest_jobs?: JobLike[] } | null)?.latest_jobs || []
  return extra.length ? extra : hotJobs.value
})
const recJobList = computed(() => ((home.value as { rec_jobs?: JobLike[] } | null)?.rec_jobs || []) as JobLike[])
const urgentList = computed(() => ((home.value as { urgent_jobs?: JobLike[] } | null)?.urgent_jobs || []) as JobLike[])
const companies = computed(() => (home.value?.rec_companies || []) as CompanyLike[])
const announcements = computed(() => (home.value?.announcements || []) as Array<{ id: number; title: string }>)
const keywords = computed(() => (home.value?.hot_keywords || []) as Array<{ keyword: string }>)
const articles = computed(() => (home.value?.new_articles || []) as ArticleLike[])
const featuredArticles = computed(() => {
  const tagged = (home.value?.featured_articles || []) as ArticleLike[]
  if (tagged.length) return tagged.slice(0, 2)
  return articles.value.filter((a) => a.picurl || a.cover).slice(0, 2)
})
const hotArticles = computed(() => (home.value?.hot_articles || []) as ArticleLike[])
const resumeList = computed(() => resumes.value?.list || [])
const pcBanners = computed(() => (adsPc.value || []).filter((b) => b.image_n || b.image))
const h5Banners = computed(() => (adsH5.value || []).filter((b) => b.image_n || b.image))
const mid13 = computed(() => adsMid.value?.slot13 || [])
const mid14 = computed(() => adsMid.value?.slot14 || [])
const mid15 = computed(() => adsMid.value?.slot15 || [])
const ads72 = computed(() => (adsMid.value?.slot72 || []).filter((b) => b.image_n || b.image || b.pic_content))
const ads73 = computed(() => (adsMid.value?.slot73 || []).filter((b) => b.image_n || b.image || b.pic_content))
const ads92 = computed(() => (adsMid.value?.slot92 || []).filter((b) => b.image_n || b.image || b.pic_content))
const ads503 = computed(() => (adsMid.value?.slot503 || []).filter((b) => b.image_n || b.image || b.pic_content))
const hasMidAds = computed(() => mid13.value.length + mid14.value.length + mid15.value.length > 0)
const linkList = computed(() => (Array.isArray(friendLinks.value) ? friendLinks.value : []) as FriendLink[])
const linkPics = computed(() => linkList.value.filter((l) => String(l.category) === '2' && (l.logo || '').trim()))
const linkTexts = computed(() => linkList.value.filter((l) => String(l.category) !== '2'))
const hasLinks = computed(() => linkPics.value.length + linkTexts.value.length > 0)

const pcSlide = ref(0)
const h5Slide = ref(0)
const noticeSlide = ref(0)
const h5Tab = ref<'latest' | 'urgent' | 'rec'>('latest')
const h5JobList = computed(() => {
  if (h5Tab.value === 'urgent') return urgentList.value
  if (h5Tab.value === 'rec') return recJobList.value
  return latestJobList.value
})
const noticeItem = computed(() => announcements.value[noticeSlide.value] || announcements.value[0])

let pcTimer: ReturnType<typeof setInterval> | undefined
let h5Timer: ReturnType<typeof setInterval> | undefined
let noticeTimer: ReturnType<typeof setInterval> | undefined
onMounted(() => {
  pcTimer = setInterval(() => {
    if (pcBanners.value.length > 1) pcSlide.value = (pcSlide.value + 1) % pcBanners.value.length
  }, 4000)
  h5Timer = setInterval(() => {
    if (h5Banners.value.length > 1) h5Slide.value = (h5Slide.value + 1) % h5Banners.value.length
  }, 4000)
  noticeTimer = setInterval(() => {
    if (announcements.value.length > 1) noticeSlide.value = (noticeSlide.value + 1) % announcements.value.length
  }, 3500)
})
onBeforeUnmount(() => {
  if (pcTimer) clearInterval(pcTimer)
  if (h5Timer) clearInterval(h5Timer)
  if (noticeTimer) clearInterval(noticeTimer)
})

function adHref(ad: Banner) {
  return ad.link || undefined
}

const loginUser = ref('')
const loginPass = ref('')
const loginErr = ref('')
async function homeLogin() {
  loginErr.value = ''
  try {
    const profile = await $fetch<{ uid: number; usertype: number }>('/api/auth/login', {
      method: 'POST',
      body: { username: loginUser.value, password: loginPass.value },
    })
    await navigateTo(profile.usertype === 2 ? '/com' : '/user')
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    loginErr.value = ex.data?.statusMessage || ex.statusMessage || t('ui.login_failed')
  }
}

useSeoMeta({
  title: () => (siteName.value ? `${siteName.value} - ${t('common.home')}` : t('common.home')),
  description: () => `${t('common.job')} / ${t('common.company')} / ${t('common.article')}`,
})
useHead({
  link: [{ rel: 'canonical', href: '/' }],
})
</script>

<template>
  <!-- ========== PC 首页，对齐 default/index/index.htm ========== -->
  <div class="site-pc">
    <p v-if="error" class="w1200 muted" style="padding: 12px 0">{{ $t('ui.home_unavailable') }}</p>
    <div v-if="ads73.length" class="index_zs_banner index_zs_banner2">
      <a v-for="(ad, i) in ads73" :key="'73-' + i" :href="adHref(ad) || '/jobs'">
        <img v-if="ad.image_n || ad.image" :src="mediaUrl(ad.image_n || ad.image)" :alt="ad.title || ''" />
      </a>
    </div>
    <div v-if="ads72.length" class="index_zs_banner index_zs_banner1">
      <a v-for="(ad, i) in ads72" :key="'72-' + i" :href="adHref(ad) || '/jobs'">
        <img v-if="ad.image_n || ad.image" :src="mediaUrl(ad.image_n || ad.image)" :alt="ad.title || ''" />
      </a>
    </div>
    <div class="w1200">
      <div class="first_floor">
        <div class="first_floor_top">
          <div class="yunheader_60zwlb">
            <div class="leftNav">
              <div id="menuLst" class="menuLst">
                <ul>
                  <li v-for="(cat, idx) in jobCats" :key="cat.id" :class="'lst' + idx">
                    <b />
                    <NuxtLink class="link" :to="`/jobs?job1=${cat.id}`">{{ cat.name }}</NuxtLink>
                    <i />
                    <div class="lstCon">
                      <div class="lstConClass">
                        <dl v-for="son in cat.children || []" :key="son.id">
                          <dt>
                            <NuxtLink :to="`/jobs?job1=${cat.id}&job1_son=${son.id}`">{{ son.name }}</NuxtLink>
                          </dt>
                          <dd>
                            <NuxtLink
                              v-for="post in son.children || []"
                              :key="post.id"
                              :to="`/jobs?job1=${cat.id}&job1_son=${son.id}&job_post=${post.id}`"
                            >{{ post.name }}</NuxtLink>
                          </dd>
                          <dd style="display: block; clear: both; width: 100%; font-size: 0; line-height: 0" />
                        </dl>
                      </div>
                    </div>
                  </li>
                  <li v-if="!jobCats.length">
                    <NuxtLink class="link" to="/jobs">{{ $t('wap_com_00420') }}</NuxtLink>
                  </li>
                </ul>
              </div>
            </div>
          </div>

          <div class="index_frist_box">
            <div v-if="pcBanners.length" class="index_huandeng">
              <div class="banner-slides">
                <a
                  v-for="(b, i) in pcBanners"
                  :key="i"
                  :class="{ 'is-on': i === pcSlide }"
                  :href="b.link || '/jobs'"
                >
                  <img :src="mediaUrl(b.image_n || b.image)" :alt="b.title || ''" />
                </a>
              </div>
              <div v-if="pcBanners.length > 1" class="banner-dots">
                <span
                  v-for="(_, i) in pcBanners"
                  :key="i"
                  :class="{ 'is-on': i === pcSlide }"
                  @click="pcSlide = i"
                />
              </div>
            </div>
            <div class="yunheader_60jpbox">
              <div v-for="job in urgentList" :key="job.id" class="js_new">
                <NuxtLink :to="`/jobs/${job.id}`" class="yunheader_60jp" :title="job.name">
                  <i class="yunheader_60jpicon" />
                  <div class="yunheader_60jplogo">
                    <img :src="mediaUrl(job.com_logo, PLACEHOLDER_LOGO)" alt="" />
                  </div>
                  <div class="yunheader_60jpbane">{{ job.name }}</div>
                  <div class="yunheader_60jpxz">{{ formatSalary(job, $t('common.negotiable'), $t('ui.yuan')) }}</div>
                </NuxtLink>
                <div class="yunheader_60jpcom">{{ job.com_name }}</div>
              </div>
              <p v-if="!urgentList.length" class="muted" style="padding: 16px 8px">{{ $t('ui.no_jobs') }}</p>
            </div>
          </div>

          <div class="fastloginbox">
            <div class="hp_login hp_login_panel">
              <template v-if="me">
                <div class="hp_login_tit">
                  <span class="yun_Indexlogin_tit_s">{{ $t('common_02492') }}</span>
                </div>
                <p style="padding: 20px 10px 8px">{{ me.username }}</p>
                <NuxtLink :to="me.usertype === 2 ? '/com' : '/user'" class="hp_login_submit" style="display: block; text-align: center; text-decoration: none">{{ $t('default_00307') }}</NuxtLink>
              </template>
              <form v-else @submit.prevent="homeLogin">
                <div class="hp_login_tit">
                  <span class="yun_Indexlogin_tit_s">{{ $t('wap_00555') }}</span>
                </div>
                <div class="hp_login_hy">
                  <input v-model="loginUser" class="hp_login_hy_but" :placeholder="$t('admin_user_00140')" autocomplete="username" />
                </div>
                <div class="hp_login_hy">
                  <input v-model="loginPass" class="hp_login_hy_but" type="password" :placeholder="$t('wap_user_00371')" autocomplete="current-password" />
                </div>
                <button class="hp_login_submit" type="submit">{{ $t('common.login') }}</button>
                <p v-if="loginErr" class="muted" style="padding: 6px 4px 0">{{ loginErr }}</p>
                <div style="padding: 8px 4px; font-size: 12px">
                  <NuxtLink to="/register">{{ $t('ajax_00016') }}</NuxtLink>
                  <NuxtLink to="/forgetpw" style="float: right">{{ $t('wap_00680') }}</NuxtLink>
                </div>
              </form>
            </div>
            <div class="new_gg fl">
              <div class="new_gg_tit">
                {{ $t('common.site_notice') }}
                <NuxtLink to="/announcements" class="new_gg_titmore">{{ $t('common.more') }}</NuxtLink>
              </div>
              <ul>
                <li v-for="a in announcements.slice(0, 4)" :key="a.id">
                  <NuxtLink :to="`/announcements/${a.id}`">{{ a.title }}</NuxtLink>
                </li>
                <li v-if="!announcements.length"><span class="muted">{{ $t('wap_00129') }}</span></li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <div v-if="hasMidAds" class="index_banner fl">
        <div class="index_banner_1250 fl">
          <div v-for="ad in mid13" :key="'13-' + (ad.title || ad.image)" class="b_w1200 b_tip">
            <a v-if="ad.image_n || ad.image" :href="adHref(ad) || '/jobs'">
              <img :src="mediaUrl(ad.image_n || ad.image)" :alt="ad.title || ''" />
            </a>
            <div v-else-if="ad.pic_content" v-html="ad.pic_content" />
          </div>
          <div v-for="ad in mid14" :key="'14-' + (ad.title || ad.image)" class="b_w289 b_tip">
            <a v-if="ad.image_n || ad.image" :href="adHref(ad) || '/jobs'">
              <img :src="mediaUrl(ad.image_n || ad.image)" :alt="ad.title || ''" />
            </a>
            <div v-else-if="ad.pic_content" v-html="ad.pic_content" />
          </div>
          <div v-for="ad in mid15" :key="'15-' + (ad.title || ad.image)" class="b_w143 b_tip">
            <a v-if="ad.image_n || ad.image" :href="adHref(ad) || '/jobs'">
              <img :src="mediaUrl(ad.image_n || ad.image)" :alt="ad.title || ''" />
            </a>
            <div v-else-if="ad.pic_content" v-html="ad.pic_content" />
          </div>
        </div>
      </div>

      <div class="index_frame_right">
        <div class="yunheader_60_tit">
          <NuxtLink to="/companies" class="yunheader_60_tit_a">
            <i class="yunheader_60_tit_line" />{{ $t('home.famous_companies') }}<i class="yunheader_60_tit_rline" />
          </NuxtLink>
        </div>
        <div class="index_mq_box">
          <div class="index_mq_box_cont">
            <ul>
              <CompanyCard v-for="c in companies" :key="c.uid" :company="c" />
            </ul>
            <p v-if="!companies.length" class="muted" style="padding: 20px">{{ $t('common_02402') }}</p>
          </div>
        </div>
        <div class="yunheader_60lookmore"><NuxtLink to="/companies">{{ $t('common.view_more') }}</NuxtLink></div>
      </div>

      <div class="index_zl_box">
        <div class="yunheader_60_tit">
          <NuxtLink to="/jobs" class="yunheader_60_tit_a">
            <i class="yunheader_60_tit_line" />{{ $t('home.recommended_jobs') }}<i class="yunheader_60_tit_rline" />
          </NuxtLink>
        </div>
        <div class="index_newjobbox index_zw_item">
          <ul>
            <JobCard v-for="job in recJobList" :key="'r' + job.id" :job="job" />
          </ul>
          <p v-if="!recJobList.length" class="muted" style="padding: 20px">{{ $t('ui.no_jobs') }}</p>
        </div>
        <div class="yunheader_60lookmore"><NuxtLink to="/jobs">{{ $t('common.view_more') }}</NuxtLink></div>

        <div class="yunheader_60_tit">
          <NuxtLink to="/jobs" class="yunheader_60_tit_a">
            <i class="yunheader_60_tit_line" />{{ $t('home.latest_jobs') }}<i class="yunheader_60_tit_rline" />
          </NuxtLink>
        </div>
        <div class="index_newjobbox index_zw_item">
          <ul>
            <JobCard v-for="job in latestJobList" :key="'n' + job.id" :job="job" />
          </ul>
          <p v-if="!latestJobList.length" class="muted" style="padding: 20px">{{ $t('ui.no_jobs') }}</p>
        </div>
        <div class="yunheader_60lookmore"><NuxtLink to="/jobs">{{ $t('common.view_more') }}</NuxtLink></div>
      </div>

      <div v-if="ads92.length" class="index_banner fl">
        <div class="index_banner_1250 fl">
          <div v-for="(ad, i) in ads92" :key="'92-' + i" class="b_w1200 b_tip">
            <a v-if="ad.image_n || ad.image" :href="adHref(ad) || '/jobs'">
              <img :src="mediaUrl(ad.image_n || ad.image)" :alt="ad.title || ''" />
            </a>
            <div v-else-if="ad.pic_content" v-html="ad.pic_content" />
          </div>
        </div>
      </div>

      <div class="index_zl_box">
        <div class="yunheader_60_tit">
          <NuxtLink to="/resumes" class="yunheader_60_tit_a">
            <i class="yunheader_60_tit_line" />{{ $t('home.recommended_talents') }}<i class="yunheader_60_tit_rline" />
          </NuxtLink>
        </div>
        <div class="tjuser_list">
          <ul>
            <li v-for="r in resumeList" :key="String(r.uid || r.id)">
              <div class="tjuser_photo">
                <img :src="mediaUrl(String(r.photo || r.photo_n || ''), PLACEHOLDER_LOGO)" width="80" height="80" alt="" />
              </div>
              <div class="tjuser_name">
                <NuxtLink :to="`/resumes/${r.uid}`">{{ r.display_name || r.name || r.uname || $t('common_02430') }}</NuxtLink>
              </div>
              <div class="tjuser_nameinfo">
                {{ r.exp_n }}<i v-if="r.exp_n && (r.edu_n || r.education_n)" class="index_resume_userinfo_line">|</i>{{ r.edu_n || r.education_n }}
              </div>
              <div class="tjuser_yx">
                {{ $t('home.intention') }}<span class="index_resume_useryx_n">{{ r.expect_name || r.job_classid_n || r.expect || r.job_post_n || '' }}</span>
              </div>
            </li>
          </ul>
          <p v-if="resumeError" class="muted" style="padding: 20px">{{
            listFailMsg(resumeError, $t('ui.rate_limit'), $t('ui.load_failed'))
          }}</p>
          <p v-else-if="!resumeList.length" class="muted" style="padding: 20px">{{ $t('ui.no_public_resumes') }}</p>
        </div>
        <div class="yunheader_60lookmore"><NuxtLink to="/resumes">{{ $t('common.view_more') }}</NuxtLink></div>
      </div>

      <div class="index_zl_box">
        <div class="yunheader_60_tit">
          <NuxtLink to="/articles" class="yunheader_60_tit_a">
            <i class="yunheader_60_tit_line" />{{ $t('home.workplace_news') }}<i class="yunheader_60_tit_rline" />
          </NuxtLink>
        </div>
        <div class="yunheader_60_tit_p" data-no="1">{{ $t('home.workplace_headline') }}</div>
        <div class="index_news_box60">
          <div class="index_news_box">
            <div v-for="a in featuredArticles" :key="'p' + a.id" class="index_news_list">
              <div class="index_news_list_img">
                <NuxtLink :to="`/articles/${a.id}`">
                  <img :src="mediaUrl(a.picurl || a.cover, PLACEHOLDER_LOGO)" width="190" height="120" :alt="a.title" />
                </NuxtLink>
              </div>
              <div class="index_news_list_info">
                <div class="index_news_list_name">
                  <NuxtLink :to="`/articles/${a.id}`" :title="a.title">{{ a.title }}</NuxtLink>
                </div>
                <div class="index_news_list_lb">{{ a.category || a.name || '' }}</div>
                <div class="index_news_list_time">{{ a.datetime_n || a.published_at_n }}</div>
              </div>
            </div>
            <ul class="index_news_list_list">
              <li v-for="a in articles" :key="a.id">
                <NuxtLink :to="`/articles/${a.id}`">
                  <i class="index_news_list_icon" />{{ a.title }}
                </NuxtLink>
                <em>{{ a.datetime_n || a.published_at_n }}</em>
              </li>
            </ul>
            <p v-if="!articles.length && !featuredArticles.length" class="muted" style="padding: 20px">{{ $t('wap_00144') }}</p>
          </div>
          <div v-if="hotArticles.length" class="index_hotnews">
            <ul>
              <li v-for="(a, idx) in hotArticles" :key="'h' + a.id">
                <span class="index_hotnews_n" :class="idx < 3 ? 'hot' + (idx + 1) : ''">{{ idx + 1 }}</span>
                <NuxtLink :to="`/articles/${a.id}`" :title="a.title">{{ a.title }}</NuxtLink>
              </li>
            </ul>
          </div>
        </div>
        <div class="yunheader_60lookmore"><NuxtLink to="/articles">{{ $t('common.view_more') }}</NuxtLink></div>
      </div>

      <div v-if="hasLinks" class="index_zl_box">
        <div class="index_link_box fl">
          <div class="new_index_tit">
            <span class="new_index_tit_list new_index_tit_cur">{{ $t('default_00256') }}<i class="new_index_tit_line" /></span>
            <NuxtLink to="/links" class="new_index_tit_more">{{ $t('common.view_more') }}</NuxtLink>
          </div>
          <div>
            <div v-if="linkPics.length" class="index_link_box_banner">
              <a v-for="l in linkPics" :key="'img' + l.id" :href="l.url" target="_blank" rel="nofollow noopener">
                <img :src="mediaUrl(l.logo)" :alt="l.name" width="160" height="50" />
              </a>
            </div>
            <div v-if="linkTexts.length" class="index_link_box_p">
              <span v-for="l in linkTexts" :key="'txt' + l.id" class="index_link_box_p_name">
                <a :href="l.url" target="_blank" rel="nofollow noopener">{{ l.name }}</a>
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- ========== H5 首页，对齐 wap/index.htm ========== -->
  <div class="site-h5">
    <div class="index_body">
      <div class="banner">
        <div v-if="h5Banners.length" class="roll">
          <a
            v-for="(b, i) in h5Banners"
            :key="i"
            :class="{ 'is-on': i === h5Slide }"
            :href="b.link || '/jobs'"
          >
            <img class="h5-banner" :src="mediaUrl(b.image_n || b.image)" :alt="b.title || ''" />
          </a>
          <div v-if="h5Banners.length > 1" class="banner-dots h5-banner-dots">
            <span
              v-for="(_, i) in h5Banners"
              :key="i"
              :class="{ 'is-on': i === h5Slide }"
              @click="h5Slide = i"
            />
          </div>
        </div>
        <div class="job">
          <div class="swiper-container navbox_jgw" id="navswiper">
            <div
              class="swiper-wrapper"
              :style="{ transform: `translate3d(-${h5NavPage * 100}%,0,0)` }"
            >
              <div v-for="(page, pi) in h5NavPages" :key="pi" class="swiper-slide">
                <NuxtLink v-for="item in page" :key="String(item.id || item.to)" :to="item.to">
                  <div class="full-time">
                    <div class="full-time-logo">
                      <img :src="item.icon" alt="" style="width: 100%" />
                    </div>
                    <div class="full-time-word">{{ item.label }}</div>
                  </div>
                </NuxtLink>
              </div>
            </div>
            <div v-if="h5NavPages.length > 1" class="swiper-pagination navbox_fyq">
              <span
                v-for="(_, i) in h5NavPages"
                :key="i"
                class="swiper-pagination-bullet"
                :class="{ 'swiper-pagination-bullet-active': i === h5NavPage }"
                @click="h5NavPage = i"
              />
            </div>
          </div>
        </div>
        <div v-if="announcements.length" class="inform">
          <div class="inform-trumpet">
            <NuxtLink to="/announcements">
              <img src="/legacy/h5/images/home_icon_notice.png" alt="" style="width: 100%" />
            </NuxtLink>
          </div>
          <NuxtLink v-if="noticeItem" :to="`/announcements/${noticeItem.id}`" style="color: #666">
            <i class="inform-word conceal_word">{{ noticeItem.title }}</i>
          </NuxtLink>
        </div>
      </div>

      <div v-if="!me" class="indexlogin_bth">
        <div class="indexlogin_bth_c">
          <div class="indexlogin_list">
            <NuxtLink to="/login" class="indexlogin_listc indexlogin_listcr">
              <i class="indexlogin_icon" />
              <div class="indexlogin_name">{{ $t('common.publish_resume') }}</div>
              <div class="indexlogin_p">{{ $t('home.find_favorite_job') }}</div>
            </NuxtLink>
          </div>
          <div class="indexlogin_list">
            <NuxtLink to="/login" class="indexlogin_listc indexlogin_listcl">
              <i class="indexlogin_icon indexlogin_icon2" />
              <div class="indexlogin_name">{{ $t('common.publish_job') }}</div>
              <div class="indexlogin_p">{{ $t('home.hire_good_talent') }}</div>
            </NuxtLink>
          </div>
        </div>
      </div>

      <div v-if="keywords.length" class="new_mq">
        <i class="new_mq_name">{{ $t('home.hot_jobs') }}</i>
        <NuxtLink class="new_mq_more" to="/jobs">{{ $t('common.more_arrow') }}</NuxtLink>
        <div class="index_jobtagbox">
          <div v-for="k in keywords" :key="k.keyword" class="index_jobtaglist">
            <NuxtLink :to="`/jobs?keyword=${encodeURIComponent(k.keyword)}`">
              <span class="index_jobtag_n">{{ k.keyword }}</span>
            </NuxtLink>
          </div>
        </div>
      </div>

      <div class="new_mq">
        <i class="new_mq_name">{{ $t('home.famous_companies') }}</i>
        <NuxtLink class="new_mq_more" to="/companies">{{ $t('common.more_arrow') }}</NuxtLink>
        <div class="new_mq_new_show">
          <CompanyCard v-for="c in companies" :key="c.uid" :company="c" />
          <p v-if="!companies.length" class="muted" style="padding: 0.3rem">{{ $t('common_02402') }}</p>
        </div>
      </div>

      <div v-if="ads503.length" class="zd_banner">
        <a v-for="(ad, i) in ads503" :key="'503-' + i" :href="adHref(ad) || '/jobs'">
          <img v-if="ad.image_n || ad.image" :src="mediaUrl(ad.image_n || ad.image)" :alt="ad.title || ''" />
        </a>
      </div>

      <div class="tab">
        <div class="h5-job-tabs">
          <button type="button" :class="{ on: h5Tab === 'latest' }" @click="h5Tab = 'latest'">{{ $t('common.latest') }}</button>
          <button type="button" :class="{ on: h5Tab === 'urgent' }" @click="h5Tab = 'urgent'">{{ $t('wap_00222') }}</button>
          <button type="button" :class="{ on: h5Tab === 'rec' }" @click="h5Tab = 'rec'">{{ $t('wap_com_00251') }}</button>
        </div>
        <JobCard v-for="job in h5JobList" :key="h5Tab + job.id" :job="job" />
        <p v-if="!h5JobList.length" class="muted" style="padding: 0.4rem">{{ $t('ui.no_jobs') }}</p>
        <div class="yunheader_60lookmore" style="text-align: center; padding: 0.3rem 0 0.6rem">
          <NuxtLink to="/jobs">{{ $t('wap_00518') }}</NuxtLink>
        </div>
      </div>

      <div class="yun_newedition_footer">
        <div>
          <NuxtLink to="/advice">{{ $t('wap_user_00203') }}</NuxtLink>
          <span class="yun_newedition_footer_line">|</span>
          <NuxtLink to="/pages/about">{{ $t('wap_00218') }}</NuxtLink>
          <span class="yun_newedition_footer_line">|</span>
          <NuxtLink to="/pages/contact">{{ $t('wap_00220') }}</NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>
