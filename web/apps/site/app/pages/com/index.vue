<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('me-com', () => api.post('/v1/wap/me', {}))
useSeoMeta({ title: '企业中心' })
</script>

<template>
  <section>
    <h1>企业中心</h1>
    <nav class="stack">
      <NuxtLink to="/com/jobs">职位管理</NuxtLink>
      <NuxtLink to="/com/jobs/new">发布职位</NuxtLink>
      <NuxtLink to="/com/talent">人才库</NuxtLink>
      <NuxtLink to="/com/orders">套餐订单</NuxtLink>
    </nav>
    <pre class="muted">{{ JSON.stringify(data, null, 2) }}</pre>
  </section>
</template>
