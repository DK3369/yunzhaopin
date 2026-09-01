<script setup lang="ts">
const code = String(useRoute().params.code || '')
const { t } = useI18n()
const api = useApi()
const LEGAL = new Set(['about', 'contact', 'privacy', 'protocol'])
const { data } = await useAsyncData(`site-page-${code}`, async () => {
  if (LEGAL.has(code)) {
    return await api.get<{ title?: string; content?: string }>('/v1/wap/legal', { slug: code })
  }
  try {
    return await api.get<{ title?: string; content?: string }>('/v1/wap/site/pages', { code })
  } catch {
    return null
  }
})
useSeoMeta({ title: () => String(data.value?.title || t('ui.pages')) })
useHead({ link: [{ rel: 'canonical', href: `/pages/${code}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || $t('ui.page_missing') }}</h1>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else class="muted">{{ $t('home.no_job_data') }}</p>
  </article>
</template>
