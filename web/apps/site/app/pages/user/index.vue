<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('me-user', () => api.post('/v1/wap/me', {}))
useSeoMeta({ title: '求职者中心' })
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await navigateTo('/login')
}
</script>

<template>
  <section>
    <h1>求职者中心</h1>
    <p v-if="error" class="muted">请先登录后再使用会员功能。</p>
    <template v-else>
      <p class="muted">uid {{ data?.uid }}</p>
      <nav class="stack">
        <NuxtLink to="/user/resume">我的简历</NuxtLink>
        <NuxtLink to="/user/applications">投递记录</NuxtLink>
      </nav>
      <button type="button" @click="logout">退出</button>
    </template>
  </section>
</template>
