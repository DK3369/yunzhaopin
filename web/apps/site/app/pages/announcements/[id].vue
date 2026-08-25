<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`ann-${id}`, () =>
  api.get('/v1/wap/announcements/detail', { id }),
)
useSeoMeta({ title: () => String(data.value?.title || '公告') })
</script>

<template>
  <article>
    <h1>{{ data?.title }}</h1>
    <div v-html="data?.content || data?.body" />
  </article>
</template>
