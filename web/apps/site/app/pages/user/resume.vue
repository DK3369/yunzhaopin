<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('my-resume', () => api.post('/v1/mcenter/resume/list', {}))
useSeoMeta({ title: '我的简历' })
</script>

<template>
  <section>
    <h1>我的简历</h1>
    <pre>{{ JSON.stringify(data, null, 2) }}</pre>
  </section>
</template>
