<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'
const api = useApi()
const { t } = useI18n()
const { comItems } = useMemberNav()
const { data, error } = await useAsyncData('me-com', () => api.post('/v1/wap/me', {}))
const { data: dash } = await useAsyncData('com-dash', () =>
  api
    .post<{
      applies_received: number
      applies_unread: number
      resume_downloads: number
    }>('/v1/mcenter/com-dashboard', {})
    .catch(() => null),
)
const { data: jobCounts } = await useAsyncData('com-job-counts', () =>
  api.post<{ total: number; online: number }>('/v1/mcenter/jobs/counts', {}).catch(() => ({ total: 0, online: 0 })),
)
useSeoMeta({ title: t('member_com_00290') })
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await navigateTo('/login')
}

const links = [
  { to: '/com/profile', icon: '/legacy/h5/images/company.png' },
  { to: '/com/gallery', icon: '/legacy/h5/images/company.png' },
  { to: '/com/jobs', icon: '/legacy/h5/images/manage_full-time.png' },
  { to: '/com/jobs/new', icon: '/legacy/h5/images/job_add.png' },
  { to: '/com/applications', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/com/talent', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/com/cert', icon: '/legacy/h5/images/company.png' },
  { to: '/com/messages', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/downloads', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/com/interviews', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/follows', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/fairs', icon: '/legacy/h5/images/diy_tit4_zph.png' },
  { to: '/com/orders', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/com/pay', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/com/stats', icon: '/legacy/h5/images/sz.png' },
  { to: '/com/password', icon: '/legacy/h5/images/sz.png' },
  { to: '/advice', icon: '/legacy/h5/images/fk.png' },
]
function labelOf(to: string) {
  return comItems.value.find((i) => i.to === to)?.label || t('common.more')
}
</script>

<template>
  <section v-if="error" class="site-inner">
    <h1>{{ $t('member_com_00290') }}</h1>
    <p class="muted">{{ isUnauthErr(error) ? $t('ui.please_login_com') : $t('ui.load_failed') }}</p>
    <NuxtLink to="/login">{{ $t('ui.go_login') }}</NuxtLink>
  </section>
  <div v-else>
    <div class="site-pc">
      <div class="membRighTops">
        <ul>
          <li class="membRighTops_mr">
            <NuxtLink to="/com/applications" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('wap_com_00105') }}</span>
              </div>
              <div class="membRiTopNum">
                <span>{{ dash?.applies_received ?? 0 }}</span>
              </div>
              <div v-if="dash?.applies_unread" class="membRiTopInx">
                <span>{{ dash.applies_unread }}</span>
              </div>
            </NuxtLink>
          </li>
          <li class="membRighTops_mr">
            <NuxtLink to="/com/jobs" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('wap_com_00106') }}</span>
              </div>
              <div class="membRiTopNum">
                <span>{{ jobCounts?.online ?? jobCounts?.total ?? 0 }}</span>
              </div>
            </NuxtLink>
          </li>
          <li class="membRighTops_mr">
            <NuxtLink to="/com/talent" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('wap_00576') }}</span>
              </div>
              <div class="membRiTopNum">
                <span>{{ dash?.resume_downloads ?? 0 }}</span>
              </div>
            </NuxtLink>
          </li>
        </ul>
      </div>
      <div class="yun_m_rightbox">
        <p class="muted">{{ data?.username || ('uid ' + data?.uid) }}</p>
        <button type="button" @click="logout">{{ $t('wap_user_00342') }}</button>
      </div>
    </div>
    <div class="site-h5">
      <div class="userheader">
        <div class="userheader_datum userheaderToubuds">
          <div class="userheader_datum_left">
            <div class="userheader_datum_job_name">
              <i>{{ data?.username || data?.uid }}</i>
            </div>
            <div class="userheader_datum_job_state">
              <div class="userheader_datum_job_data">{{ $t('common.company') }}</div>
            </div>
          </div>
        </div>
      </div>
      <div>
        <NuxtLink v-for="item in links" :key="item.to" :to="item.to">
          <div class="taskbar_enterprise">
            <div class="taskbar_datum">
              <div class="taskbar_datum_img">
                <img :src="item.icon" alt="" width="100%" height="100%" />
              </div>
              <div class="taskbar_datum_word">{{ labelOf(item.to) }}</div>
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
