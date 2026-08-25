<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`special-${id}`, () => api.get('/v1/wap/specials/detail', { id }))
useSeoMeta({ title: () => String(data.value?.title || '专题详情') })
useHead({ link: [{ rel: 'canonical', href: `/specials/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || '专题不存在' }}</h1>
    <p v-if="data?.intro" class="muted">{{ data.intro }}</p>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.title" class="muted">没有这个专题，或暂时无法加载。</p>
  </article>
</template>
