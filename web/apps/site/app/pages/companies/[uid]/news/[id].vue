<script setup lang="ts">
const route = useRoute()
const { t, locale } = useI18n()
const uid = Number(route.params.uid)
const id = Number(route.params.id)
const api = useApi()
const { data, error } = await useAsyncData(
  () => `company-news-${locale.value}-${uid}-${id}`,
  () => api.get('/v1/wap/companies/news/detail', { uid, id }),
)
const row = computed(() => (data.value || {}) as Record<string, unknown>)
useSeoMeta({
  title: () => String(row.value.title || t('company_00019')),
  description: () => stripHtml(row.value.summary || row.value.body || row.value.title),
})
</script>

<template>
  <article class="site-inner">
    <p>
      <NuxtLink :to="`/companies/${uid}`">{{ $t('common.company') }}</NuxtLink>
    </p>
    <h1>{{ row.title || $t('company_00019') }}</h1>
    <p v-if="error" class="muted">{{ $t('common_00376') }}</p>
    <p v-if="row.summary" class="muted">{{ row.summary }}</p>
    <div v-if="row.body" v-html="String(row.body)" />
  </article>
</template>
