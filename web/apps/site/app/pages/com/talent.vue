<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('talent', () =>
  api.post('/v1/mcenter/talent-pool/list', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '人才库' })
</script>

<template>
  <section>
    <h1>人才库</h1>
    <pre>{{ JSON.stringify(data, null, 2) }}</pre>
  </section>
</template>
