<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { userItems } = useMemberNav()
const { data, error } = await useAsyncData('me-user', () => api.post('/v1/wap/me', {}))
const { data: dash } = await useAsyncData('user-dash', () =>
  api
    .post<{
      interview_count: number
      apply_count: number
      favorite_count: number
      view_count: number
      unread_messages: number
      wkyqnum: number
      commsgnum: number
      sxnum: number
      sysnum: number
    }>('/v1/mcenter/dashboard', {})
    .catch(() => null),
)
const { data: follows } = await useAsyncData('user-follow-n', () =>
  api.post<{ total: number }>('/v1/mcenter/follows/list', { kind: 2, page: 1, page_size: 1 }).catch(() => ({ total: 0 })),
)
const { data: resume, refresh: refreshResume } = await useAsyncData('user-home-resume', () =>
  api
    .post<{
      name?: string
      photo?: string
      birthday?: string
      education_n?: string
      exp_n?: string
      def_job?: number
      lastupdate_n?: string
      uid?: number
    }>('/v1/mcenter/resume/list', {})
    .catch(() => null),
)
const { data: expects } = await useAsyncData('user-home-expects', () =>
  api.post('/v1/mcenter/resume/expects/list', {}).catch(() => []),
)
const { data: completion } = await useAsyncData('user-home-score', () =>
  api.post<{ score?: number; missing?: string[] }>('/v1/mcenter/resume/completion', {}).catch(() => null),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('user-home-sign', () =>
  api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const { data: gzh } = await useAsyncData('user-gzh', () =>
  api.post<{ subscribe?: number }>('/v1/mcenter/wechat/subscribe', {}).catch(() => ({ subscribe: 1 })),
)
const { data: recJobs } = await useAsyncData('user-home-rec-jobs', () =>
  api.post('/v1/mcenter/recommend/jobs', { limit: 8 }).catch(() => []),
)
type HomeRecJob = { id: number; name?: string; com_name?: string; uid?: number; min_salary?: number; max_salary?: number }
const recJobList = computed((): HomeRecJob[] => {
  const raw = recJobs.value
  if (Array.isArray(raw)) return raw as HomeRecJob[]
  if (raw && typeof raw === 'object' && 'list' in raw) return ((raw as { list?: HomeRecJob[] }).list || []) as HomeRecJob[]
  return []
})
const gzhNeed = computed(() => Number(gzh.value?.subscribe || 0) !== 1)
const { wxQr } = useSiteChrome()
const msg = ref('')
useSeoMeta({ title: t('member_user_00183') })

type HomeExpect = {
  id?: number
  name?: string
  job_classid_n?: string
  job_name?: string
  jobstatus_n?: string
  report_n?: string
  hits?: number
}
const expectList = computed((): HomeExpect[] => {
  const raw = expects.value
  if (Array.isArray(raw)) return raw as HomeExpect[]
  if (raw && typeof raw === 'object' && 'list' in raw) return ((raw as { list?: HomeExpect[] }).list || []) as HomeExpect[]
  return []
})
const defExpect = computed(() => {
  const id = Number(resume.value?.def_job || 0)
  return expectList.value.find((e) => Number(e.id) === id) || expectList.value[0] || null
})
const extraExpects = computed(() => {
  const defId = Number(defExpect.value?.id || 0)
  return expectList.value.filter((e) => Number(e.id) !== defId)
})
const integrity = computed(() => Number(completion.value?.score || 0))
const missingBits = computed(() => completion.value?.missing || [])
function missingLabel(k: string) {
  const map: Record<string, string> = {
    basic_info: t('wap_00269'),
    photo: t('wap_user_00204'),
    expect: t('wap_00460'),
    education: t('wap_00459'),
    work: t('wap_00457'),
    skill_or_language_or_project: t('wap_00450'),
  }
  return map[k] || k
}
function ageOf(birthday?: string) {
  if (!birthday) return 0
  const y = Number(String(birthday).slice(0, 4))
  if (!y) return 0
  return new Date().getFullYear() - y
}
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await refreshNuxtData('auth-me')
  await navigateTo('/login')
}
async function refreshMyResume() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/refresh', {})
    msg.value = t('wap_user_00198')
    await refreshResume()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function sign() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sign', {})
    msg.value = t('common.success')
    await refreshSign()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

const h5Links = [
  { to: '/user/resume', icon: '/legacy/h5/images/resume_index.png', key: 'wap_user_00204' },
  { to: '/user/privacy', icon: '/legacy/h5/images/ys.png', key: 'wap_user_00215' },
  { to: '/user/otherservice', icon: '/legacy/h5/images/job_training.png', key: 'wap_user_00196' },
  { to: '/user/finance', icon: '/legacy/h5/images/financial_management.png', key: 'wap_user_00213' },
  { to: '/user/set', icon: '/legacy/h5/images/sz.png', key: 'wap_user_00214' },
  { to: '/advice', icon: '/legacy/h5/images/fk.png', key: 'wap_user_00203' },
]
function labelOf(to: string, key: string) {
  return userItems.value.find((i) => i.to === to)?.label || t(key)
}
</script>

<template>
  <section v-if="error" class="site-inner">
    <h1>{{ $t('member_user_00183') }}</h1>
    <p class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <NuxtLink to="/login">{{ $t('ui.go_login') }}</NuxtLink>
  </section>
  <div v-else class="member-page member-page-user">
    <div class="site-pc">
      <div v-if="gzhNeed" class="yun_wtbd_tip">
        <div class="yun_wtbd_tip_tit">{{ $t('wap_user_00205') }}</div>
        <div class="yun_wtbd_tip_p">
          {{ $t('wap_user_00191') }}
          <a v-if="wxQr" href="javascript:;" class="wxtitle yun_wtbd_tip_bth">{{ $t('member_user_00150') }}</a>
          <img v-if="wxQr" :src="wxQr" alt="" width="80" height="80" />
        </div>
      </div>
        <div class="yun_m_index_date_box">
        <div class="yun_m_index_date_box_c">
          <div class="yun_m_index_date_list">
            <NuxtLink to="/user/interviews">
              <i class="yun_m_index_date_icon1" />
              <span v-if="dash?.wkyqnum" class="yun_m_n">{{ dash.wkyqnum }}</span>
              <div class="yun_m_index_datename">{{ $t('wap_user_00216') }}</div>
              <div class="yun_m_index_date_n">
                <span class="yun_m_index_d_c">{{ dash?.interview_count ?? 0 }}</span>
              </div>
            </NuxtLink>
          </div>
          <div class="yun_m_index_date_list">
            <NuxtLink to="/user/applications">
              <i class="yun_m_index_date_icon2" />
              <div class="yun_m_index_datename">{{ $t('member_user_00149') }}</div>
              <div class="yun_m_index_date_n">{{ dash?.apply_count ?? 0 }}</div>
            </NuxtLink>
          </div>
          <div class="yun_m_index_date_list">
            <NuxtLink to="/user/favorites">
              <i class="yun_m_index_date_icon3" />
              <div class="yun_m_index_datename">{{ $t('member_user_00103') }}</div>
              <div class="yun_m_index_date_n">{{ dash?.favorite_count ?? 0 }}</div>
            </NuxtLink>
          </div>
          <div class="yun_m_index_date_list yun_m_index_date_list_end">
            <NuxtLink to="/user/follows">
              <i class="yun_m_index_date_icon4" />
              <div class="yun_m_index_datename">{{ $t('wap_01142') }}</div>
              <div class="yun_m_index_date_n">{{ follows?.total ?? 0 }}</div>
            </NuxtLink>
          </div>
        </div>
      </div>
      <div class="yun_m_index_resume">
        <div class="yun_m_index_resume_tit">
          <div class="yun_m_index_resume_span">{{ $t('wap_user_00204') }}</div>
        <div v-if="resume?.name || defExpect" class="user_resume_box">
          <div class="user_resume_photo">
            <NuxtLink to="/user/resume">
              <img v-if="resume?.photo" :src="mediaUrl(resume.photo)" alt="" />
            </NuxtLink>
          </div>
          <div class="user_resume_info">
            <div class="user_resume_name">
              {{ resume?.name || data?.username }}
              <span v-if="defExpect?.name" class="user_resume_job">{{ defExpect.name }}</span>
            </div>
            <div class="user_resume_p">
              <template v-if="ageOf(resume?.birthday)">{{ ageOf(resume?.birthday) }}{{ $t('common_02074') }}</template>
              <span v-if="resume?.exp_n" class="user_resume_line">|</span>
              <template v-if="resume?.exp_n">{{ resume.exp_n }}{{ $t('home.experience_suffix') }}</template>
              <span v-if="resume?.education_n" class="user_resume_line">|</span>
              <template v-if="resume?.education_n">{{ resume.education_n }}{{ $t('home.education_suffix') }}</template>
            </div>
            <div v-if="defExpect" class="user_resume_p2">{{ defExpect.jobstatus_n }} {{ defExpect.report_n }}</div>
          </div>
          <div class="user_resume_c">
            <div class="user_resume_wzd">
              <NuxtLink to="/user/resume">
                <span class="user_resume_wzd_name">{{ $t('wap_00328') }}：</span>
                <div class="user_resume_wzd_b"><span class="user_resume_wzd_c" :style="{ width: `${integrity}%` }" /></div>
                <span class="user_resume_wzd_r">{{ integrity }}%</span>
              </NuxtLink>
              <p v-if="missingBits.length" class="muted">
                <NuxtLink to="/user/resume">{{ missingBits.map(missingLabel).join(' · ') }}</NuxtLink>
              </p>
            </div>
            <div class="user_resume_p user_resume_pd">{{ resume?.lastupdate_n }}</div>
            <div v-if="defExpect?.hits != null" class="user_resume_p">{{ $t('member_com_00268') }}：{{ defExpect.hits }}</div>
          </div>
          <div class="user_resume_cz">
            <div class="user_resume_cz_p">
              <a href="javascript:;" class="user_resume_cz_a user_resume_cz_icon2" @click="refreshMyResume">{{ $t('wap_user_00199') }}</a>
            </div>
            <div class="user_resume_cz_p">
              <NuxtLink :to="`/resumes/${resume?.uid || data?.uid}`" class="user_resume_cz_a user_resume_cz_icon4">{{ $t('wap_user_00217') }}</NuxtLink>
            </div>
            <div class="user_resume_cz_p">
              <NuxtLink to="/user/resume" class="user_resume_cz_a user_resume_cz_icon1">{{ $t('wap_user_00207') }} <span class="user_resume_cz_yzd">{{ $t('wap_user_00335') }}</span></NuxtLink>
            </div>
            <div class="user_resume_cz_p">
              <NuxtLink to="/user/resume" class="user_resume_cz_a user_resume_cz_icon3">{{ $t('wap_00269') }}</NuxtLink>
            </div>
          </div>
        </div>
        <div v-if="missingBits.length && (resume?.name || defExpect)" class="user_resume_boxtip">
          <div class="user_resume_boxtip_c">
            <div class="user_resume_boxtip_h1">{{ missingBits.map(missingLabel).join(' · ') }}</div>
            <div class="user_resume_boxtip_p">{{ $t('common_01975') }}</div>
            <NuxtLink to="/user/resume" class="user_resume_boxtip_bth">{{ $t('wap_user_00197') }}</NuxtLink>
          </div>
        </div>
        <div v-else-if="!(resume?.name || defExpect)" class="member_right_no_job">
          <div class="member_right_no_job_box">
            <div class="yun_m_index_job_icon" />
            <div class="member_right_no_jobr">
              {{ $t('member_user_00128') }}
              <NuxtLink to="/user/resume" class="member_right_no_jobr_bth">{{ $t('wap_user_00197') }}</NuxtLink>
            </div>
          </div>
        </div>
        <div v-for="row in extraExpects" :key="'ex-' + row.id" class="member_index_resume_box">
          <div class="member_index_resume_t">
            <div class="member_index_resume_t_left">
              <div class="member_index_resume_t_name fltL">
                <div class="member_index_resume_t_name_l member_index_resume_t_name_w80 fltL">{{ $t('member_com_00013') }}</div>
                {{ row.name }}
              </div>
              <div class="member_index_resume_job fltL">
                <span class="member_index_resume_t_name_l member_index_resume_t_name_w80 fltL">{{ $t('wap_user_00015') }}</span>
                <span class="member_index_resume_jobname">{{ row.job_classid_n || row.job_name || row.name }}</span>
              </div>
            </div>
            <div class="member_index_resume_t_cz fltR">
              <div class="member_index_resume_t_cz_b">
                <NuxtLink to="/user/resume" class="member_index_resume_t_cz_bth">{{ $t('wap_00269') }}</NuxtLink>
                <NuxtLink :to="`/resumes/${resume?.uid || data?.uid}`" class="member_index_resume_t_cz_bth mt15">{{ $t('wap_user_00217') }}</NuxtLink>
              </div>
            </div>
          </div>
        </div>
        </div>
      </div>
      <div class="member_right_box_banner fltL" />
      <div class="yun_m_index_job mt20 fltL">
        <div class="yun_m_index_job_tit"><span class="yun_m_index_job_tit_s">{{ $t('member_user_00134') }}</span></div>
        <div v-if="!resume?.name && !defExpect" class="member_right_no_job">
          <div class="member_right_no_job_box">
            <div class="yun_m_index_job_icon" />
            <div class="member_right_no_jobr">
              {{ $t('member_user_00128') }}
              <NuxtLink to="/user/resume" class="member_right_no_jobr_bth">{{ $t('wap_user_00197') }}</NuxtLink>
            </div>
          </div>
        </div>
        <div v-else id="joblist" class="member_right_job_box">
          <div v-if="!recJobList.length" id="nojoblist" class="member_right_no_job">
            <div class="member_right_no_job_box">
              <div class="yun_m_index_job_icon" />
              <div class="yun_m_index_job_tip">{{ $t('member_user_00137') }}</div>
            </div>
          </div>
          <div v-for="job in recJobList" :key="'rj-' + job.id" class="yun_m_index_joblist">
            <NuxtLink :to="`/jobs/${job.id}`" class="yun_m_index_jobname">{{ job.name }}</NuxtLink>
            <span v-if="job.min_salary || job.max_salary" class="yun_m_index_jobxz">{{ job.min_salary }}-{{ job.max_salary }}</span>
            <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`" class="yun_m_index_jobcom">{{ job.com_name }}</NuxtLink>
          </div>
        </div>
      </div>
      <p v-if="msg" class="muted">{{ msg }}</p>
    </div>
    <div class="site-h5">
      <div v-if="missingBits.length" class="heiseVipDao">
        <div class="vip_nav">
          <div class="vip_nav_img">
            <img src="/legacy/h5/images/inform.png" alt="" width="100%" height="100%" />
          </div>
          <i class="vip_nav_word">{{ missingBits.map(missingLabel).join(' · ') }}</i>
          <NuxtLink to="/user/resume" class="vip_nav_remind">{{ $t('wap_user_00197') }}</NuxtLink>
        </div>
      </div>
      <p v-if="gzhNeed" class="muted" style="padding: 0.16rem 0.24rem">
        {{ $t('common_00655') }}
        <img v-if="wxQr" :src="wxQr" alt="" width="80" height="80" />
      </p>
      <div class="userheader">
        <div class="userheader_nav">
          <div class="userheader_nav_calendar" @click="signSt?.signed_today ? undefined : sign()">
            <img
              :src="signSt?.signed_today ? '/legacy/h5/images/comtop2.png' : '/legacy/h5/images/comtop22.png'"
              alt=""
              width="100%"
              height="100%"
            />
          </div>
          <NuxtLink to="/user/set" class="userheader_nav_set">
            <img src="/legacy/h5/images/comtop4.png" alt="" width="100%" height="100%" />
          </NuxtLink>
        </div>
        <div class="userheader_datum userheaderToubuds">
          <NuxtLink to="/user/resume" class="userheader_datum_logo">
            <img v-if="resume?.photo" :src="mediaUrl(resume.photo)" alt="" width="100%" height="100%" />
          </NuxtLink>
          <div class="userheader_datum_left">
            <div class="userheader_datum_job_name">
              <i>{{ resume?.name || data?.username || data?.uid }}</i>
              <div v-if="integrity" class="userheader_datum_job_name_number">
                <img src="/legacy/h5/images/dskke.png" alt="" />
                <span>{{ integrity }}%</span>
              </div>
            </div>
            <p v-if="missingBits.length" class="muted">
              <NuxtLink to="/user/resume">{{ missingBits.map(missingLabel).join(' · ') }}</NuxtLink>
            </p>
            <div class="userheader_datum_job_state">
              <div v-if="resume?.exp_n || resume?.education_n" class="userheader_datum_job_data">
                {{ resume?.exp_n }}{{ resume?.education_n }}{{ ageOf(resume?.birthday) ? ageOf(resume?.birthday) + $t('common_02074') : '' }}
              </div>
              <div v-else class="userheader_datum_job_data">{{ $t('wap_user_00189') }}</div>
            </div>
          </div>
          <NuxtLink to="/user/resume" class="userheader_datum_right">
            <div class="userheader_datum_right_word">
              <span>{{ resume?.name ? $t('wap_user_00208') : $t('wap_user_00197') }}</span>
              <img src="/legacy/h5/images/comtop1.png" alt="" />
            </div>
          </NuxtLink>
        </div>
        <div class="userparticulars">
          <ul>
            <li>
              <NuxtLink to="/user/interviews">
                <i class="userparticulars_number">{{ dash?.interview_count ?? 0 }}</i>
                <i class="userparticulars_word">{{ $t('wap_user_00216') }}</i>
              </NuxtLink>
            </li>
            <li>
              <NuxtLink to="/user/applications">
                <i class="userparticulars_number">{{ dash?.apply_count ?? 0 }}</i>
                <i class="userparticulars_word">{{ $t('wap_00787') }}</i>
              </NuxtLink>
            </li>
            <li>
              <NuxtLink to="/user/favorites">
                <i class="userparticulars_number">{{ dash?.favorite_count ?? 0 }}</i>
                <i class="userparticulars_word">{{ $t('member_user_00103') }}</i>
              </NuxtLink>
            </li>
            <li>
              <NuxtLink to="/user/views">
                <i class="userparticulars_number">{{ dash?.view_count ?? 0 }}</i>
                <i class="userparticulars_word">{{ $t('wap_user_00276') }}</i>
              </NuxtLink>
            </li>
          </ul>
        </div>
      </div>
      <div class="user_nav_fast mt10">
        <ul>
          <li @click="refreshMyResume">
            <div class="user_nav_fast_img">
              <img src="/legacy/h5/images/jobhunter_refresh.png" alt="" width="100%" height="100%" />
            </div>
            <i class="user_nav_fast_word">{{ $t('wap_user_00199') }}</i>
          </li>
          <li>
            <NuxtLink :to="`/resumes/${resume?.uid || data?.uid}`">
              <div class="user_nav_fast_img">
                <img src="/legacy/h5/images/jobhunter_preview.png" alt="" width="100%" height="100%" />
              </div>
              <i class="user_nav_fast_word">{{ $t('wap_user_00217') }}</i>
            </NuxtLink>
          </li>
          <li>
            <NuxtLink to="/user/recommend">
              <div class="user_nav_fast_img">
                <img src="/legacy/h5/images/userpp.png" alt="" width="100%" height="100%" />
              </div>
              <i class="user_nav_fast_word">{{ $t('wap_user_00211') }}</i>
            </NuxtLink>
          </li>
          <li>
            <NuxtLink to="/user/resume">
              <div class="user_nav_fast_img">
                <img src="/legacy/h5/images/jobhunter_top.png" alt="" width="100%" height="100%" />
              </div>
              <i class="user_nav_fast_word">{{ $t('wap_user_00210') }}</i>
            </NuxtLink>
          </li>
        </ul>
      </div>
      <p v-if="msg" class="muted">{{ msg }}</p>
      <div class="taskbar">
      <div class="taskbar_box">
        <NuxtLink v-for="item in h5Links" :key="item.to" :to="item.to">
          <div class="taskbar_enterprise">
            <div class="taskbar_datum">
              <div class="taskbar_datum_img">
                <img :src="item.icon" alt="" width="100%" height="100%" />
              </div>
              <div class="taskbar_datum_word">{{ labelOf(item.to, item.key) }}</div>
            </div>
            <div class="taskbar_nav">
              <div class="taskbar_nav_word">{{ $t('common.more') }}</div>
              <div class="taskbar_nav_img">
                <img src="/legacy/h5/images/my_more.png" alt="" width="100%" height="100%" />
              </div>
            </div>
          </div>
        </NuxtLink>
        <div class="taskbar_enterprise_last" @click="logout">
          <div class="taskbar_datum">
            <div class="taskbar_datum_word">{{ $t('wap_user_00342') }}</div>
          </div>
        </div>
      </div>
      </div>
    </div>
  </div>
</template>
