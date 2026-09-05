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
  api.post<{ score?: number }>('/v1/mcenter/resume/completion', {}).catch(() => null),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('user-home-sign', () =>
  api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const msg = ref('')
useSeoMeta({ title: t('member_user_00183') })

const expectList = computed(() => {
  const raw = expects.value
  if (Array.isArray(raw)) return raw
  return raw?.list || []
})
const defExpect = computed(() => {
  const id = Number(resume.value?.def_job || 0)
  return expectList.value.find((e: { id?: number }) => Number(e.id) === id) || expectList.value[0] || null
})
const integrity = computed(() => Number(completion.value?.score || 0))
function ageOf(birthday?: string) {
  if (!birthday) return 0
  const y = Number(String(birthday).slice(0, 4))
  if (!y) return 0
  return new Date().getFullYear() - y
}
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
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
  { to: '/user/parts', icon: '/legacy/h5/images/job_training.png', key: 'wap_user_00220' },
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
  <div v-else>
    <div class="site-pc">
      <div class="yun_m_index_date_box">
        <div class="yun_m_index_date_box_c">
          <div class="yun_m_index_date_list">
            <NuxtLink to="/user/interviews">
              <i class="yun_m_index_date_icon1" />
              <span v-if="dash?.unread_messages" class="yun_m_n">{{ dash.unread_messages }}</span>
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
        </div>
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
              <span class="user_resume_wzd_name">{{ $t('wap_00328') }}：</span>
              <div class="user_resume_wzd_b"><span class="user_resume_wzd_c" :style="{ width: `${integrity}%` }" /></div>
              <span class="user_resume_wzd_r">{{ integrity }}%</span>
            </div>
            <div class="user_resume_p user_resume_pd">{{ resume?.lastupdate_n }}</div>
          </div>
          <div class="user_resume_cz">
            <div class="user_resume_cz_p">
              <button type="button" class="user_resume_cz_a" @click="refreshMyResume">{{ $t('wap_user_00199') }}</button>
            </div>
            <div class="user_resume_cz_p">
              <NuxtLink :to="`/resumes/${resume?.uid || data?.uid}`" class="user_resume_cz_a">{{ $t('wap_user_00217') }}</NuxtLink>
            </div>
            <div class="user_resume_cz_p">
              <NuxtLink to="/user/recommend" class="user_resume_cz_a">{{ $t('wap_user_00211') }}</NuxtLink>
            </div>
            <div class="user_resume_cz_p">
              <NuxtLink to="/user/resume" class="user_resume_cz_a">{{ $t('wap_00269') }}</NuxtLink>
            </div>
          </div>
        </div>
        <div v-else class="member_right_no_job">
          <div class="member_right_no_job_box">
            <div class="member_right_no_jobr">
              {{ $t('member_user_00128') }}
              <NuxtLink to="/user/resume" class="member_right_no_jobr_bth">{{ $t('wap_user_00197') }}</NuxtLink>
            </div>
          </div>
        </div>
      </div>
      <p v-if="msg" class="muted">{{ msg }}</p>
      <div class="yun_m_rightbox">
        <p class="muted">{{ data?.username || ('uid ' + data?.uid) }}</p>
        <button type="button" @click="logout">{{ $t('wap_user_00342') }}</button>
      </div>
    </div>
    <div class="site-h5">
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
                <span>{{ integrity }}%</span>
              </div>
            </div>
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
        </ul>
      </div>
      <p v-if="msg" class="muted">{{ msg }}</p>
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
</template>
