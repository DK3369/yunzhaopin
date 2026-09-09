<template>
  <!-- PC：深色单行顶栏 -->
  <div class="site-pc">
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
          <NuxtLink :to="hireTo" class="pc-topbar__text">{{ $t('common.publish_job') }}</NuxtLink>
          <NuxtLink to="/jobs" class="pc-topbar__text">{{ $t('common.job') }}</NuxtLink>
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

  <!-- H5：首页 yunTop；内页蓝条返回 -->
  <div class="site-h5">
    <div v-if="isHome" class="yunTop">
      <div class="yunlogobox" style="display: flex; align-items: center; justify-content: space-between; gap: 0.16rem">
        <img v-if="logoH5" :src="logoH5" alt="" class="yunlogo" />
        <span v-else class="header_p_z">{{ siteName }}</span>
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
    <template v-else>
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
const {
  siteName,
  logoPc,
  logoH5,
  nav,
  me,
  isHome,
  memberHome,
  h5Title,
  logout,
  navActive,
} = useSiteChrome()

const hireTo = computed(() => (me.value ? '/com' : '/login?next=/com'))

function goBack() {
  if (window.history.length > 1) {
    window.history.back()
    return
  }
  navigateTo('/')
}
</script>
