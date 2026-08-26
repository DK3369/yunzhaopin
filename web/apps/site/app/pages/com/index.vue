<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { comItems } = useMemberNav()
const { data, error } = await useAsyncData('me-com', () => api.post('/v1/wap/me', {}))
useSeoMeta({ title: t('member_com_00290') })
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await navigateTo('/login')
}

const links = [
  { to: '/com/profile', icon: '/legacy/h5/images/company.png' },
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
    <p class="muted">{{ $t('wap_00376') }}</p>
    <NuxtLink to="/login">{{ $t('ui.go_login') }}</NuxtLink>
  </section>
  <div v-else>
    <div class="site-pc site-inner">
      <h1>{{ $t('member_com_00290') }}</h1>
      <p class="muted">{{ data?.username || ('uid ' + data?.uid) }}</p>
      <button type="button" @click="logout">{{ $t('wap_user_00342') }}</button>
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
