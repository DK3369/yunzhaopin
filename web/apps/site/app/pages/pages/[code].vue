<script setup lang="ts">
const code = String(useRoute().params.code || '')
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`site-page-${code}`, () =>
  api.get('/v1/wap/site/pages', { code }),
)
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
