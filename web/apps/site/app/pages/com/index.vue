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
      job_msg_unanswered?: number
      unread_messages?: number
    }>('/v1/mcenter/com-dashboard', {})
    .catch(() => null),
)
const { data: fans } = await useAsyncData('com-fans-n', () =>
  api.post<{ total: number }>('/v1/mcenter/fans', { page: 1, page_size: 1 }).catch(() => ({ total: 0 })),
)
const { data: looks } = await useAsyncData('com-looks-n', () =>
  api.post<{ total: number }>('/v1/mcenter/look-jobs/list', { page: 1, page_size: 1 }).catch(() => ({ total: 0 })),
)
const { data: gzh } = await useAsyncData('com-gzh', () =>
  api.post<{ subscribe?: number }>('/v1/mcenter/wechat/subscribe', {}).catch(() => ({ subscribe: 1 })),
)
const gzhNeed = computed(() => Number(gzh.value?.subscribe || 0) !== 1)
const { wxQr } = useSiteChrome()
useSeoMeta({ title: t('member_com_00290') })
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await refreshNuxtData('auth-me')
  await navigateTo('/login')
}

const links = [
  { to: '/com/profile', icon: '/legacy/h5/images/company.png' },
  { to: '/com/gallery', icon: '/legacy/h5/images/company.png' },
  { to: '/com/jobs', icon: '/legacy/h5/images/manage_full-time.png' },
  { to: '/com/jobs/new', icon: '/legacy/h5/images/job_add.png' },
  { to: '/com/parts', icon: '/legacy/h5/images/job_training.png' },
  { to: '/com/applications', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/com/looks', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/fans', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/talent', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/com/cert', icon: '/legacy/h5/images/company.png' },
  { to: '/com/messages', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/job-messages', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/downloads', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/com/interviews', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/follows', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/fairs', icon: '/legacy/h5/images/diy_tit4_zph.png' },
  { to: '/com/orders', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/com/pay', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/com/stats', icon: '/legacy/h5/images/sz.png' },
  { to: '/com/password', icon: '/legacy/h5/images/sz.png' },
  { to: '/com/set', icon: '/legacy/h5/images/sz.png' },
  { to: '/com/otherservice', icon: '/legacy/h5/images/sz.png' },
  { to: '/com/binding', icon: '/legacy/h5/images/sz.png' },
  { to: '/com/finder', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/com/news', icon: '/legacy/h5/images/company.png' },
  { to: '/com/products', icon: '/legacy/h5/images/company.png' },
  { to: '/com/banners', icon: '/legacy/h5/images/company.png' },
  { to: '/com/addresses', icon: '/legacy/h5/images/company.png' },
  { to: '/com/interview-tpls', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/integral', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/com/hrs', icon: '/legacy/h5/images/company.png' },
  { to: '/com/recommend', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/com/specials', icon: '/legacy/h5/images/diy_tit4_zph.png' },
  { to: '/redeem/orders', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/com/broadcasts', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/com/warnings', icon: '/legacy/h5/images/fk.png' },
  { to: '/com/member-right', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/advice', icon: '/legacy/h5/images/fk.png' },
]
function labelOf(to: string) {
  return comItems.value.find((i) => i.to === to)?.label || t('common.more')
}
</script>

<template>
  <section v-if="error" class="site-inner">
    <h1>{{ $t('member_com_00290') }}</h1>
    <p class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <NuxtLink to="/login">{{ $t('ui.go_login') }}</NuxtLink>
  </section>
  <div v-else>
    <div class="site-pc">
      <p v-if="gzhNeed" class="muted" style="padding: 8px 0">
        {{ $t('common_00655') }}
        <img v-if="wxQr" :src="wxQr" alt="" width="80" height="80" />
      </p>
      <div class="membRighTops">
        <ul>
          <li class="membRighTops_mr">
            <NuxtLink to="/com/applications" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('member_com_00152') }}</span>
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
            <NuxtLink to="/com/fans" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('wap_com_00407') }}</span>
              </div>
              <div class="membRiTopNum">
                <span>{{ fans?.total ?? 0 }}</span>
              </div>
            </NuxtLink>
          </li>
          <li class="membRighTops_mr">
            <NuxtLink to="/com/looks" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('wap_user_00276') }}</span>
              </div>
              <div class="membRiTopNum">
                <span>{{ looks?.total ?? 0 }}</span>
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
      <p v-if="gzhNeed" class="muted" style="padding: 0.16rem 0.24rem">
        {{ $t('common_00655') }}
        <img v-if="wxQr" :src="wxQr" alt="" width="80" height="80" />
      </p>
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
              <div class="taskbar_datum_word">
                {{ labelOf(item.to) }}
                <span v-if="item.to === '/com/applications' && dash?.applies_unread" class="yun_m_n">{{ dash.applies_unread }}</span>
                <span v-else-if="item.to === '/com/job-messages' && dash?.job_msg_unanswered" class="yun_m_n">{{ dash.job_msg_unanswered }}</span>
                <span v-else-if="item.to === '/com/messages' && dash?.unread_messages" class="yun_m_n">{{ dash.unread_messages }}</span>
              </div>
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
