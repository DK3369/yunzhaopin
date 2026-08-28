<template>
  <!-- PC 顶栏 + 导航，对齐原版 header.htm / index_header.htm -->
  <div class="site-pc">
    <div class="yun_new_top">
      <div class="yun_new_cont">
        <div class="yun_new_left">{{ $t('wap_com_00102') }}：{{ phone || '—' }}</div>
        <div class="yun_new_right">
          <span class="yun_new_right_we">{{ $t('common.welcome', { site: siteName }) }}</span>
          <LangSwitch />
          <span class="login_head_id">
            <template v-if="me">
              <NuxtLink :to="memberHome">{{ me.username }}</NuxtLink>
              <a href="javascript:;" @click.prevent="logout">{{ $t('common.logout') }}</a>
            </template>
            <template v-else>
              <NuxtLink to="/login">{{ $t('common.login') }}</NuxtLink>
              <NuxtLink to="/register">{{ $t('common.register') }}</NuxtLink>
            </template>
          </span>
        </div>
      </div>
    </div>

    <div v-if="isHome" class="yunheader_60">
      <div class="w1200">
        <div class="yunheader_60logo fl">
          <NuxtLink to="/" :title="siteName">
            <img v-if="logoPc" :src="logoPc" :alt="siteName" />
            <span v-else class="site-wordmark">{{ siteName }}</span>
          </NuxtLink>
        </div>
        <div class="hp_head_search fl">
          <div class="hp_head_searchbor">
            <form action="/jobs" method="get">
              <div class="hp_head_search_job fl">
                <span class="hp_head_search_job_b">{{ $t('common.job') }}</span>
              </div>
              <input
                class="hp_head_search_text fl"
                type="text"
                name="keyword"
                :value="String(route.query.keyword || '')"
                :placeholder="$t('default_00348')"
              />
              <input class="hp_head_search_sr fl" type="submit" :value="$t('common.search')" />
            </form>
          </div>
        </div>
        <div class="yunheader_60nav">
          <ul>
            <li v-for="item in nav" :key="item.to + item.label" :class="{ nav_list_hover: navActive(item.to) }">
              <NuxtLink :to="item.to" class="png">{{ item.label }}</NuxtLink>
              <i class="yun_new_headernav_list_line" />
            </li>
          </ul>
        </div>
      </div>
    </div>

    <div v-else class="hp_head hp_head_box">
      <div class="w1200">
        <div class="hp_head_ft fl">
          <div class="phpyun_logo fl">
            <NuxtLink to="/" :title="siteName">
              <img v-if="logoPc" :src="logoPc" :alt="siteName" />
              <span v-else class="site-wordmark">{{ siteName }}</span>
            </NuxtLink>
          </div>
        </div>
        <div class="yun_header_nav_box">
          <ul>
            <li v-for="item in nav" :key="item.to + item.label" :class="{ nav_list_hover: navActive(item.to) }">
              <NuxtLink :to="item.to" class="png">{{ item.label }}</NuxtLink>
              <i class="yun_new_headernav_list_line" />
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>

  <!-- H5：首页 yunTop；内页蓝条返回 -->
  <div class="site-h5">
    <div v-if="isHome" class="yunTop">
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
    <template v-else>
      <div class="header_fixed">
        <div class="header_bg">
          <a class="header_back" href="javascript:;" @click.prevent="goBack" />
          <div class="header_h1">{{ h5Title }}</div>
          <div class="header_lang" style="position: absolute; right: 0.2rem; top: 50%; transform: translateY(-50%); z-index: 2">
            <LangSwitch />
          </div>
        </div>
      </div>
      <div class="header_h" />
    </template>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const {
  siteName,
  phone,
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

function goBack() {
  if (window.history.length > 1) {
    window.history.back()
    return
  }
  navigateTo('/')
}
</script>
