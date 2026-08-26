<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { userItems } = useMemberNav()
const { data, error } = await useAsyncData('me-user', () => api.post('/v1/wap/me', {}))
useSeoMeta({ title: t('member_user_00183') })
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await navigateTo('/login')
}

const links = [
  { to: '/user/resume', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/user/applications', icon: '/legacy/h5/images/job_issue.png' },
  { to: '/user/interviews', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/user/favorites', icon: '/legacy/h5/images/icon_collect.png' },
  { to: '/user/follows', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/user/messages', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/user/views', icon: '/legacy/h5/images/icon_communication.png' },
  { to: '/user/expects', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/user/resume-tpls', icon: '/legacy/h5/images/Please_resume.png' },
  { to: '/user/privacy', icon: '/legacy/h5/images/sz.png' },
  { to: '/user/password', icon: '/legacy/h5/images/sz.png' },
  { to: '/user/binding', icon: '/legacy/h5/images/sz.png' },
  { to: '/user/integral', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/user/pay', icon: '/legacy/h5/images/financial_management.png' },
  { to: '/advice', icon: '/legacy/h5/images/fk.png' },
]
function labelOf(to: string) {
  return userItems.value.find((i) => i.to === to)?.label || t('common.more')
}
</script>

<template>
  <section v-if="error" class="site-inner">
    <h1>{{ $t('member_user_00183') }}</h1>
    <p class="muted">{{ $t('wap_00376') }}</p>
    <NuxtLink to="/login">{{ $t('ui.go_login') }}</NuxtLink>
  </section>
  <div v-else>
    <div class="site-pc">
      <div class="yun_m_index_date_box">
        <div class="yun_m_index_date_box_c">
          <div class="yun_m_index_date_list">
            <NuxtLink to="/user/interviews">
              <i class="yun_m_index_date_icon1" />
              <div class="yun_m_index_datename">{{ $t('wap_user_00216') }}</div>
            </NuxtLink>
          </div>
          <div class="yun_m_index_date_list">
            <NuxtLink to="/user/applications">
              <i class="yun_m_index_date_icon2" />
              <div class="yun_m_index_datename">{{ $t('wap_user_00270') }}</div>
            </NuxtLink>
          </div>
          <div class="yun_m_index_date_list">
            <NuxtLink to="/user/favorites">
              <i class="yun_m_index_date_icon3" />
              <div class="yun_m_index_datename">{{ $t('member_user_00103') }}</div>
            </NuxtLink>
          </div>
          <div class="yun_m_index_date_list yun_m_index_date_list_end">
            <NuxtLink to="/user/follows">
              <i class="yun_m_index_date_icon4" />
              <div class="yun_m_index_datename">{{ $t('wap_00385') }}</div>
            </NuxtLink>
          </div>
        </div>
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
              <div class="userheader_datum_job_data">{{ $t('wap_00544') }}</div>
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
