<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`part-${id}`, () => api.get('/v1/wap/parts/detail', { id }))
useSeoMeta({ title: () => String(data.value?.name || '兼职详情') })
useHead({ link: [{ rel: 'canonical', href: `/parts/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || '兼职不存在' }}</h1>
    <p v-if="data?.com_name || data?.address" class="muted">{{ data.com_name }} {{ data.address }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.name" class="muted">没有这条兼职，或暂时无法加载。</p>
  </article>
</template>
