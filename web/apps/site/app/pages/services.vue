<script setup lang="ts">
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData('site-services', () =>
  api.post<{ title?: string; content?: string; name?: string }>('/v1/wap/descriptions/get', { id: 5 }).catch(() => null),
)
useSeoMeta({ title: () => String(data.value?.title || data.value?.name || t('ui.pages')) })
useHead({ link: [{ rel: 'canonical', href: '/services' }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || data?.name || $t('ui.page_missing') }}</h1>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else class="muted">{{ $t('common_02409') }}</p>
  </article>
</template>
