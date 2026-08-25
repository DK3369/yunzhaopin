<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`gz-${id}`, () => api.get('/v1/wap/gongzhao/detail', { id }))
useSeoMeta({ title: () => String(data.value?.title || '公招详情') })
useHead({ link: [{ rel: 'canonical', href: `/gongzhao/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || '公招不存在' }}</h1>
    <p v-if="data?.start_at_n" class="muted">{{ data.start_at_n }} — {{ data.end_at_n }}</p>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.title" class="muted">没有这条公招，或暂时无法加载。</p>
  </article>
</template>
