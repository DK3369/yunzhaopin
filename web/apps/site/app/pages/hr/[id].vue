<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`hr-${id}`, () => api.get('/v1/wap/hr-docs/detail', { id }))
useSeoMeta({ title: () => String(data.value?.name || 'HR 文档') })
useHead({ link: [{ rel: 'canonical', href: `/hr/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || '文档不存在' }}</h1>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.name" class="muted">没有这份文档，或暂时无法加载。</p>
  </article>
</template>
