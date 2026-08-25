<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('my-apps', () =>
  api.post('/v1/mcenter/my-applications', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '投递记录' })
</script>

<template>
  <section>
    <h1>投递记录</h1>
    <pre>{{ JSON.stringify(data, null, 2) }}</pre>
  </section>
</template>
