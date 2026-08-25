<script setup lang="ts">
const code = String(useRoute().params.code || '')
const api = useApi()
const { data } = await useAsyncData(`site-page-${code}`, () =>
  api.get('/v1/wap/site/pages', { code }),
)
useSeoMeta({ title: () => String(data.value?.title || '站点页面') })
useHead({ link: [{ rel: 'canonical', href: `/pages/${code}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || '页面不存在' }}</h1>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else class="muted">没有这个页面。</p>
  </article>
</template>
