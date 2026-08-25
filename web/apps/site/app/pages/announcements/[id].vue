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
    <h1>{{ data?.title || '公告不存在' }}</h1>
    <div v-if="data?.content || data?.body" v-html="data?.content || data?.body" />
    <p v-else class="muted">没有这条公告，或暂时无法加载。</p>
  </article>
</template>
