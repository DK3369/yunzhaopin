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
    <h1>{{ data?.title }}</h1>
    <div v-html="data?.content || data?.body" />
  </article>
</template>
