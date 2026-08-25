<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`article-${id}`, () =>
  api.get('/v1/wap/articles/detail', { id }),
)
useSeoMeta({ title: () => String(data.value?.title || '文章') })
useHead({ link: [{ rel: 'canonical', href: `/articles/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || '文章不存在' }}</h1>
    <div v-if="data?.content || data?.body" v-html="data?.content || data?.body" />
    <p v-else class="muted">没有这篇文章，或暂时无法加载。</p>
  </article>
</template>
