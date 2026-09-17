<template>
  <!-- 会员中心 PC 用 PHP user_header / 企业 header，见 MemberPcHeader -->
  <div v-if="!isMember" class="site-pc">
    <header class="pc-topbar">
      <div class="pc-topbar__inner">
        <NuxtLink to="/" class="pc-topbar__logo" :title="siteName">
          <img v-if="logoPc" :src="logoPc" :alt="siteName" />
          <span v-else class="site-wordmark">{{ siteName }}</span>
        </NuxtLink>
        <nav class="pc-topbar__nav" aria-label="site">
          <NuxtLink
            v-for="item in nav"
            :key="String(item.id || item.to) + item.label"
            :to="item.to"
            class="pc-topbar__link"
            :class="{ 'is-on': navActive(item.to) }"
          >{{ item.label }}</NuxtLink>
        </nav>
        <div class="pc-topbar__actions">
          <LangSwitch />
          <NuxtLink v-if="showPublishJob" to="/com/jobs/new" class="pc-topbar__text">{{ $t('common.publish_job') }}</NuxtLink>
          <template v-if="me">
            <NuxtLink :to="memberHome" class="pc-topbar__text">{{ me.username }}</NuxtLink>
            <a href="javascript:;" class="pc-topbar__text" @click.prevent="logout">{{ $t('common.logout') }}</a>
          </template>
          <NuxtLink v-else to="/login" class="pc-topbar__pill">
            {{ $t('common.login') }}/{{ $t('common.register') }}
          </NuxtLink>
        </div>
      </div>
    </header>
  </div>

  <!-- H5：首页 yunTop；会员首页无返回条；简历白顶 / 财务深顶 / 其余蓝条 -->
  <div class="site-h5">
    <div v-if="h5Bar === 'home'" class="yunTop">
      <div class="yunlogobox" style="display: flex; align-items: center; justify-content: space-between; gap: 0.16rem">
        <img v-if="logoH5" :src="logoH5" alt="" class="yunlogo" />
        <span v-else class="header_p_z">{{ siteName }}</span>
        <LangSwitch />
      </div>
      <div class="index_newedition_search_box">
        <div class="index_newedition_searchbg">
          <form class="index_newedition_search_c" action="/jobs" method="get">
            <input
              class="index_newedition_search_p searchnew"
              style="width: 95%"
              name="keyword"
              :placeholder="$t('wap_user_00254')"
            />
            <button class="index_newedition_searchbth" type="submit" :aria-label="$t('common.search')" />
          </form>
        </div>
      </div>
    </div>
    <template v-else-if="h5Bar === 'white'">
      <div class="m_whiteheader">
        <div class="m_whiteheaderfid">
          <a class="header_back" href="javascript:;" @click.prevent="goBack" />
          <div class="m_header_cont">{{ h5Title }}</div>
        </div>
      </div>
    </template>
    <template v-else-if="h5Bar === 'dark'">
      <div class="m_backheader">
        <a class="header_back" href="javascript:;" @click.prevent="goBack" />
        <div class="m_header_cont">{{ h5Title }}</div>
      </div>
    </template>
    <template v-else-if="h5Bar === 'blue'">
      <div class="header_fixed">
        <div class="header_bg">
          <a class="header_back" href="javascript:;" @click.prevent="goBack" />
          <div class="header_h1">{{ h5Title }}</div>
        </div>
      </div>
      <div class="header_h" />
    </template>
  </div>
</template>

<script setup lang="ts">
import { isMemberModuleOn } from '../utils/site'

const {
  siteName,
  logoPc,
  logoH5,
  nav,
  me,
  isHome,
  isMember,
  memberHome,
  h5Title,
  logout,
  navActive,
  settings,
} = useSiteChrome()
const route = useRoute()
const isCompany = computed(() => Number(me.value?.usertype) === 2)
const showPublishJob = computed(() => isCompany.value && isMemberModuleOn(settings.value, '/com/jobs/new'))
const H5_DARK = ['/user/finance', '/user/pay', '/user/integral', '/user/rewards', '/user/integral-rules', '/com/pay', '/com/integral', '/com/orders', '/com/record', '/com/rewards', '/com/integral-rules', '/com/services']
const h5Bar = computed<'home' | 'none' | 'white' | 'dark' | 'blue'>(() => {
  if (isHome.value) return 'home'
  const p = route.path
  if (p === '/user' || p === '/com') return 'none'
  if (p === '/user/resume' || p.startsWith('/user/resume/') || p === '/user/expects') return 'white'
  if (H5_DARK.some((x) => p === x || p.startsWith(`${x}/`))) return 'dark'
  return 'blue'
})

function goBack() {
  if (window.history.length > 1) {
    window.history.back()
    return
  }
  navigateTo('/')
}
</script>
