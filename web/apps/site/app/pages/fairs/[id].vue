<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`fair-${id}`, () => api.get('/v1/wap/zph/detail', { id }))
useSeoMeta({ title: () => String(data.value?.title || '招聘会详情') })
useHead({ link: [{ rel: 'canonical', href: `/fairs/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || '招聘会不存在' }}</h1>
    <p v-if="data?.address" class="muted">{{ data.address }} · {{ data.start_at_n }}</p>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.title" class="muted">没有这场招聘会，或暂时无法加载。</p>
  </article>
</template>
