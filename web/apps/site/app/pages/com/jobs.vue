<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('com-jobs', () =>
  api.post('/v1/mcenter/jobs/list', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '职位管理' })
</script>

<template>
  <section>
    <h1>职位管理</h1>
    <NuxtLink to="/com/jobs/new">发布职位</NuxtLink>
    <pre>{{ JSON.stringify(data, null, 2) }}</pre>
  </section>
</template>
