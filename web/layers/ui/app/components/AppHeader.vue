<template>
  <!-- PC 顶栏 + 导航，对齐原版 header.htm / index_header.htm -->
  <div class="site-pc">
    <div class="yun_new_top">
      <div class="yun_new_cont">
        <div class="yun_new_left">{{ $t('wap_com_00102') }}：{{ phone || '—' }}</div>
        <div class="yun_new_right" id="login_head_div">
          <div
            class="yun_topNav fr"
            @mouseenter="navMore = true"
            @mouseleave="navMore = false"
          >
            <a
              class="yun_navMore"
              :class="{ yun_webMorecurrent: navMore }"
              href="javascript:;"
            >{{ $t('common.website_nav') }}</a>
            <div v-show="navMore" class="yun_webMoredown">
              <div class="yun_top_nav_box">
                <ul class="yun_top_nav_box_l">
                  <li v-for="item in nav" :key="'map-' + String(item.id || item.to)">
                    <NuxtLink :to="item.to">{{ item.label }}</NuxtLink>
                  </li>
                </ul>
                <ul v-if="appNav.length || wxQr || wapQr" class="yun_top_nav_box_wx">
                  <li v-for="item in appNav" :key="'app-' + String(item.id || item.to)">
                    <NuxtLink :to="item.to">{{ item.label }}</NuxtLink>
                  </li>
                  <li v-if="wapQr">
                    <img :src="wapQr" width="70" height="70" alt="" />
                  </li>
                  <li v-if="wxQr">
                    <img :src="wxQr" width="70" height="70" alt="" />
                  </li>
                </ul>
              </div>
            </div>
          </div>
          <span class="yun_new_right_we">{{ $t('common.welcome', { site: siteName }) }}</span>
          <NuxtLink v-if="sitePickOn" to="/site" class="yun_new_right_wap">{{ $t('ui.pick_site') }}</NuxtLink>
          <NuxtLink to="/" class="yun_new_right_wap">{{ $t('common.mobile_site') }}</NuxtLink>
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
            <form :action="searchAction" method="get">
              <div class="hp_head_search_job fl" @click.stop="searchMenu = !searchMenu">
                <span class="hp_head_search_job_b">{{ searchKindLabel }}</span>
                <div v-show="searchMenu" class="index_header_seach_find_list yunHeaderSearch_list_box">
                  <a href="javascript:;" @click.prevent="setSearchKind('job')">{{ $t('default_00246') }}</a>
                  <a href="javascript:;" @click.prevent="setSearchKind('resume')">{{ $t('default_00312') }}</a>
                  <a href="javascript:;" @click.prevent="setSearchKind('tiny')">{{ $t('wap_js_00066') }}</a>
                  <a href="javascript:;" @click.prevent="setSearchKind('once')">{{ $t('wap_js_00130') }}</a>
                </div>
              </div>
              <input
                class="hp_head_search_text fl"
                type="text"
                name="keyword"
                :value="String(route.query.keyword || '')"
                :placeholder="searchPlaceholder"
              />
              <input class="hp_head_search_sr fl" type="submit" :value="$t('common.search')" />
            </form>
          </div>
          <div class="clear" />
          <div class="hp_head_search_bom">
            <div class="hp_head_search_bom_left">
              <span style="color: #a4a1a1">{{ $t('common_02507') }}</span>
              <NuxtLink
                v-for="k in hotSearches"
                :key="k.keyword"
                :to="`/jobs?keyword=${encodeURIComponent(k.keyword)}`"
                :title="k.keyword"
              >{{ k.keyword }}</NuxtLink>
            </div>
            <div class="yun_new_header_search_more moreOptions">
              <div>
                <NuxtLink to="/jobs">{{ $t('default_00246') }}</NuxtLink>
                <NuxtLink to="/map">{{ $t('default_00139') }}</NuxtLink>
                <NuxtLink to="/jobs">{{ $t('common.search') }}</NuxtLink>
              </div>
            </div>
          </div>
        </div>
        <div class="yunheader_60nav">
          <ul>
            <li v-for="item in nav" :key="String(item.id || item.to) + item.label" :class="{ nav_list_hover: navActive(item.to) }">
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
            <li v-for="item in nav" :key="String(item.id || item.to) + item.label" :class="{ nav_list_hover: navActive(item.to) }">
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
const route = useRoute()
const { t } = useI18n()
const {
  siteName,
  phone,
  logoPc,
  logoH5,
  nav,
  appNav,
  me,
  isHome,
  memberHome,
  h5Title,
  logout,
  navActive,
  hotSearches,
  wxQr,
  wapQr,
  settings,
} = useSiteChrome()
const sitePickOn = computed(() => String(settings.value.sy_web_site || '') === '1')

type SearchKind = 'job' | 'resume' | 'tiny' | 'once'
const searchKind = ref<SearchKind>('job')
const searchMenu = ref(false)
const navMore = ref(false)
const searchAction = computed(() => {
  if (searchKind.value === 'resume') return '/resumes'
  if (searchKind.value === 'tiny') return '/tiny'
  if (searchKind.value === 'once') return '/once'
  return '/jobs'
})
const searchKindLabel = computed(() => {
  if (searchKind.value === 'resume') return t('default_00312')
  if (searchKind.value === 'tiny') return t('wap_js_00066')
  if (searchKind.value === 'once') return t('wap_js_00130')
  return t('default_00246')
})
const searchPlaceholder = computed(() => t('default_00348'))

function setSearchKind(kind: SearchKind) {
  searchKind.value = kind
  searchMenu.value = false
}

function goBack() {
  if (window.history.length > 1) {
    window.history.back()
    return
  }
  navigateTo('/')
}

onMounted(() => {
  document.addEventListener('click', () => {
    searchMenu.value = false
  })
})
</script>
