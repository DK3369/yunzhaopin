<script setup lang="ts">
const route = useRoute()
const uid = Number(route.params.uid)
const api = useApi()
const { data } = await useAsyncData(`company-${uid}`, () =>
  api.get('/v1/wap/companies/detail', { uid }),
)
const company = computed(() => (data.value || {}) as Record<string, unknown>)
useSeoMeta({
  title: () => String(company.value.name || '企业详情'),
  description: () => String(company.value.content || company.value.hy_n || ''),
})
useHead({
  link: [{ rel: 'canonical', href: `/companies/${uid}` }],
  script: [
    {
      type: 'application/ld+json',
      innerHTML: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'Organization',
        name: company.value.name,
        identifier: uid,
      }),
    },
  ],
})
</script>

<template>
  <article v-if="company.name">
    <h1>{{ company.name }}</h1>
    <p class="muted">{{ company.hy_n }} · {{ company.city_two }}</p>
    <div v-html="company.content" />
  </article>
  <article v-else>
    <h1>企业不存在</h1>
    <p class="muted">没有这家企业，或暂时无法加载。</p>
  </article>
</template>
