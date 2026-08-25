<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('me-user', () => api.post('/v1/wap/me', {}))
useSeoMeta({ title: '求职者中心' })
</script>

<template>
  <section>
    <h1>求职者中心</h1>
    <nav class="stack">
      <NuxtLink to="/user/resume">我的简历</NuxtLink>
      <NuxtLink to="/user/applications">投递记录</NuxtLink>
    </nav>
    <pre class="muted">{{ JSON.stringify(data, null, 2) }}</pre>
    <button @click="$fetch('/api/auth/logout', { method: 'POST' }).then(() => navigateTo('/login'))">
      退出
    </button>
  </section>
</template>
