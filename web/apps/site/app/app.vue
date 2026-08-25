<script setup lang="ts">
const route = useRoute()
const siteUrl = String(useRuntimeConfig().public.siteUrl || 'http://127.0.0.1:3001').replace(/\/$/, '')
useHead({
  link: [{ rel: 'canonical', href: () => `${siteUrl}${route.path}` }],
})
</script>

<template>
  <div class="page">
    <AppHeader />
    <main>
      <NuxtPage />
    </main>
    <AppFooter />
  </div>
</template>
