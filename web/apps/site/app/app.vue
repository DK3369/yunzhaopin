<script setup lang="ts">
import { isPathModuleOn } from '~/utils/site'

const route = useRoute()
const { locale, setLocale, t } = useI18n()
async function applyQueryLang() {
  const mapped = mapPhpLang(String(route.query.lang || ''))
  if (mapped && mapped !== locale.value) {
    persistWebLocale(mapped)
    await setLocale(mapped)
  }
}
await applyQueryLang()
watch(() => route.query.lang, () => applyQueryLang())
const siteUrl = String(useRuntimeConfig().public.siteUrl || 'http://127.0.0.1:3001').replace(/\/$/, '')
const { isHome, isAuth, isMember, settings } = useSiteChrome()
const { userItems, comItems } = useMemberNav()
const memberItems = computed(() => (route.path.startsWith('/com') ? comItems.value : userItems.value))
const siteClosed = computed(() => String(settings.value.sy_web_online || '') === '2')
const ipBanned = computed(() => String(settings.value.sy_client_ip_banned || '') === '1')
const moduleOff = computed(() => {
  if (isAuth.value || isMember.value) return false
  return !isPathModuleOn(settings.value, route.path)
})
const siteBlocked = computed(() => siteClosed.value || ipBanned.value || moduleOff.value)
const blockHtml = computed(() => {
  if (ipBanned.value) return settings.value.sy_bannedip_alert || t('ui.ip_banned')
  if (siteClosed.value) return settings.value.sy_webclose || t('ui.site_closed')
  return t('ui.module_closed')
})

useHead({
  htmlAttrs: {
    lang: () => (locale.value === 'en' ? 'en' : 'zh-CN'),
    class: () => (String(settings.value.sy_wap_web || '') === '2' ? 'force-pc' : ''),
  },
  bodyAttrs: {
    class: () => (/^\/jobs\/\d+/.test(route.path) ? 'comapply_bg' : 'body_bg'),
  },
  link: () => [
    { rel: 'canonical', href: `${siteUrl}${route.path}` },
    ...(route.path.startsWith('/user') || route.path.startsWith('/com')
      ? [
          {
            rel: 'stylesheet',
            href: '/legacy/h5/css/member/memberwap.css',
            media: 'screen and (max-width: 1199px)',
          },
        ]
      : []),
  ],
})

const mainClass = computed(() => {
  if (isHome.value || isAuth.value || isMember.value) return ''
  if (route.path.startsWith('/jobs') || route.path.startsWith('/companies') || route.path.startsWith('/resumes')) return ''
  return 'site-inner'
})
</script>

<template>
  <div v-if="siteBlocked" class="site-closed site-inner" v-html="blockHtml" />
  <div v-else class="site-root">
    <AppHeader v-if="!isAuth" />
    <main :class="mainClass">
      <MemberShell v-if="isMember" :items="memberItems" :kind="route.path.startsWith('/com') ? 'com' : 'user'">
        <NuxtPage />
      </MemberShell>
      <NuxtPage v-else />
    </main>
    <AppFooter v-if="!isAuth && !isMember" />
  </div>
</template>
