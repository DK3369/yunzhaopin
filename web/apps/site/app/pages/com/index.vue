<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('me-com', () => api.post('/v1/wap/me', {}))
useSeoMeta({ title: '企业中心' })
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await navigateTo('/login')
}
</script>

<template>
  <section>
    <h1>企业中心</h1>
    <p v-if="error" class="muted">请先登录后再使用企业功能。</p>
    <template v-else>
      <p class="muted">uid {{ data?.uid }}</p>
      <nav class="stack">
        <NuxtLink to="/com/profile">企业资料</NuxtLink>
        <NuxtLink to="/com/jobs">职位管理</NuxtLink>
        <NuxtLink to="/com/jobs/new">发布职位</NuxtLink>
        <NuxtLink to="/com/applications">收到的简历</NuxtLink>
        <NuxtLink to="/com/talent">人才库</NuxtLink>
        <NuxtLink to="/com/orders">套餐订单</NuxtLink>
      </nav>
      <button type="button" @click="logout">退出</button>
    </template>
  </section>
</template>
