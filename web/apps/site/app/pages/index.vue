<script setup lang="ts">
import { catTree, formatSalary, listFailMsg, mediaUrl, PLACEHOLDER_BANNER, PLACEHOLDER_LOGO, type CatNode, type JobLike } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { siteName, me, h5Nav } = useSiteChrome()

const { data: home, error } = await useAsyncData('home', async () => {
  const h = (await api.get('/v1/wap/home', { did: 0 })) as {
    hot_jobs?: JobLike[]
    rec_companies?: Array<Record<string, unknown>>
    announcements?: unknown[]
    hot_keywords?: unknown[]
    new_articles?: unknown[]
  }
  const [rec, latest, urgent] = await Promise.all([
    api.get<{ list: JobLike[] }>('/v1/wap/jobs', { rec: true, page_size: 8 }).catch(() => ({ list: [] as JobLike[] })),
    api.get<{ list: JobLike[] }>('/v1/wap/jobs', { page_size: 8 }).catch(() => ({ list: [] as JobLike[] })),
    api.get<{ list: JobLike[] }>('/v1/wap/jobs', { urgent: true, page_size: 8 }).catch(() => ({ list: [] as JobLike[] })),
  ])
  if (!(h.rec_companies || []).length) {
    const c = await api.get<{ list: Array<Record<string, unknown>> }>('/v1/wap/companies', { page_size: 12 })
    h.rec_companies = c.list || []
  }
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
  api.get<Array<{ image_n?: string; image?: string; link?: string; title?: string }>>('/v1/wap/ads', { slot: '3', limit: 5 }).catch(() => []),
)
const { data: adsH5 } = await useAsyncData('ads-50', () =>
  api.get<Array<{ image_n?: string; image?: string; link?: string; title?: string }>>('/v1/wap/ads', { slot: '50', limit: 5 }).catch(() => []),
)
const { data: resumes, error: resumeError } = await useAsyncData('home-resumes', () =>
  api.get<{ list: Array<Record<string, unknown>> }>('/v1/wap/resumes', { page_size: 8 }),
)

const jobCats = computed(() => catTree(cats.value || [], 11))
const hotJobs = computed(() => (home.value?.hot_jobs || []) as JobLike[])
const recJobList = computed(() => {
  const extra = (home.value as { rec_jobs?: JobLike[] } | null)?.rec_jobs || []
  return extra.length ? extra : hotJobs.value
})
const latestJobList = computed(() => {
  const extra = (home.value as { latest_jobs?: JobLike[] } | null)?.latest_jobs || []
  return extra.length ? extra : hotJobs.value
})
const urgentList = computed(() => {
  const extra = (home.value as { urgent_jobs?: JobLike[] } | null)?.urgent_jobs || []
  return extra.length ? extra : hotJobs.value.slice(0, 8)
})
const companies = computed(() => home.value?.rec_companies || [])
const announcements = computed(() => home.value?.announcements || [])
const keywords = computed(() => home.value?.hot_keywords || [])
const articles = computed(() => home.value?.new_articles || [])
const resumeList = computed(() => resumes.value?.list || [])
const pcBanners = computed(() => adsPc.value || [])
const h5Banners = computed(() => adsH5.value || [])

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
  title: () => `${siteName.value} - ${t('common.home')}`,
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
            <div class="index_huandeng">
              <a v-if="pcBanners[0]" :href="pcBanners[0].link || '/jobs'">
                <img :src="mediaUrl(pcBanners[0].image_n || pcBanners[0].image, PLACEHOLDER_BANNER)" :alt="pcBanners[0].title || ''" />
              </a>
              <img v-else :src="PLACEHOLDER_BANNER" alt="" />
            </div>
            <div class="yunheader_60jpbox">
              <div v-for="job in urgentList" :key="job.id" class="js_new">
                <NuxtLink :to="`/jobs/${job.id}`" class="yunheader_60jp" :title="job.name">
                  <i class="yunheader_60jpicon" />
                  <div class="yunheader_60jplogo">
                    <img :src="mediaUrl(job.com_logo, PLACEHOLDER_LOGO)" alt="" />
                  </div>
                  <div class="yunheader_60jpbane">{{ job.name }}</div>
                  <div class="yunheader_60jpxz">{{ formatSalary(job) }}</div>
                </NuxtLink>
                <div class="yunheader_60jpcom">{{ job.com_name }}</div>
              </div>
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
          <p v-if="!recJobList.length" class="muted" style="padding: 20px">{{ $t('ui.no_hot_jobs') }}</p>
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
        </div>
        <div class="yunheader_60lookmore"><NuxtLink to="/jobs">{{ $t('common.view_more') }}</NuxtLink></div>
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
                {{ r.exp_n }}<i v-if="r.exp_n && r.edu_n" class="index_resume_userinfo_line">|</i>{{ r.edu_n }}
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
        <div class="index_news_box60">
          <div class="index_news_box">
            <ul class="index_news_list_list">
              <li v-for="a in articles" :key="a.id">
                <NuxtLink :to="`/articles/${a.id}`">
                  <i class="index_news_list_icon" />{{ a.title }}
                </NuxtLink>
                <em>{{ a.datetime_n || a.published_at_n }}</em>
              </li>
            </ul>
            <p v-if="!articles.length" class="muted" style="padding: 20px">{{ $t('wap_00144') }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- ========== H5 首页，对齐 wap/index.htm ========== -->
  <div class="site-h5">
    <div class="index_body">
      <div class="banner">
        <div class="roll">
          <a v-if="h5Banners[0]" :href="h5Banners[0].link || '/jobs'">
            <img class="h5-banner" :src="mediaUrl(h5Banners[0].image_n || h5Banners[0].image, PLACEHOLDER_BANNER)" alt="" />
          </a>
          <img v-else class="h5-banner" :src="PLACEHOLDER_BANNER" alt="" />
        </div>
        <div class="job">
          <div class="navbox_jgw">
            <NuxtLink v-for="item in h5Nav" :key="item.to" :to="item.to">
              <div class="full-time">
                <div class="full-time-logo">
                  <img :src="item.icon" alt="" style="width: 100%" />
                </div>
                <div class="full-time-word">{{ item.label }}</div>
              </div>
            </NuxtLink>
          </div>
        </div>
        <div v-if="announcements.length" class="inform">
          <div class="inform-trumpet">
            <NuxtLink to="/announcements">
              <img src="/legacy/h5/images/home_icon_notice.png" alt="" style="width: 100%" />
            </NuxtLink>
          </div>
          <NuxtLink :to="`/announcements/${announcements[0].id}`" style="color: #666">
            <i class="inform-word conceal_word">{{ announcements[0].title }}</i>
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

      <div class="tab">
        <JobCard v-for="job in latestJobList" :key="job.id" :job="job" />
        <p v-if="!latestJobList.length" class="muted" style="padding: 0.4rem">{{ $t('ui.no_hot_jobs') }}</p>
        <div class="yunheader_60lookmore" style="text-align: center; padding: 0.3rem 0 0.6rem">
          <NuxtLink to="/jobs">{{ $t('wap_00518') }}</NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>
