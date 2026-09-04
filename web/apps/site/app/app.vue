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
const { saveSite, gotocity } = useSubSite()
const api = useApi()
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

onMounted(() => {
  if (String(settings.value.sy_web_site || '') !== '1') return
  if (String(settings.value.sy_gotocity || '') !== '1') return
  if (gotocity.value) return
  if (!navigator.geolocation) {
    gotocity.value = '1'
    return
  }
  navigator.geolocation.getCurrentPosition(
    async (pos) => {
      try {
        const row = await api.post<{
          error?: number
          city?: string
          did?: number
          domain?: string
          mode?: number
          indexdir?: string
          province?: number
          city_id?: number
          three_city_id?: number
          hy?: number
          fz_type?: number
          web_name?: string
          web_title?: string
          web_logo?: string
        }>('/v1/wap/regions/city-domain', {
          x: pos.coords.longitude,
          y: pos.coords.latitude,
        })
        gotocity.value = '1'
        if (!row || Number(row.error) !== 1 || !row.did) return
        const ok = window.confirm(t('ui.goto_city', { city: row.city || '' }))
        if (!ok) return
        saveSite({
          did: row.did,
          province: row.province,
          city_id: row.city_id,
          three_city_id: row.three_city_id,
          hy: row.hy,
          fz_type: row.fz_type,
          web_name: row.web_name,
          web_title: row.web_title,
          web_logo: row.web_logo,
          mode: row.mode,
          domain: row.domain,
          indexdir: row.indexdir,
        })
        const mode = Number(row.mode || 0)
        if (mode === 1 && row.domain) {
          const host = String(row.domain).replace(/^https?:\/\//, '')
          window.location.href = `${window.location.protocol}//${host}`
          return
        }
        if (mode === 2 && row.indexdir) {
          const dir = String(row.indexdir).replace(/^\/+|\/+$/g, '')
          if (dir) await navigateTo(`/${dir}/`)
        }
      } catch {
        gotocity.value = '1'
      }
    },
    () => {
      gotocity.value = '1'
    },
  )
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
