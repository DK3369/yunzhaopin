<script setup lang="ts">
const route = useRoute()
const { locale, setLocale } = useI18n()
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
const { isHome, isAuth, isMember } = useSiteChrome()
const { userItems, comItems } = useMemberNav()
const memberItems = computed(() => (route.path.startsWith('/com') ? comItems.value : userItems.value))

useHead({
  htmlAttrs: { lang: () => (locale.value === 'en' ? 'en' : 'zh-CN') },
  bodyAttrs: { class: 'body_bg' },
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
  <div class="site-root">
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
