<template>
  <header class="site-header">
    <NuxtLink to="/" class="brand">{{ siteName }}</NuxtLink>
    <nav>
      <NuxtLink to="/jobs">职位</NuxtLink>
      <NuxtLink to="/companies">企业</NuxtLink>
      <NuxtLink to="/resumes">简历</NuxtLink>
      <NuxtLink to="/search">搜索</NuxtLink>
      <NuxtLink to="/articles">资讯</NuxtLink>
      <NuxtLink to="/login">登录</NuxtLink>
      <NuxtLink to="/user">会员中心</NuxtLink>
    </nav>
  </header>
</template>

<script setup lang="ts">
const siteName = useRuntimeConfig().public.siteName
</script>

<style scoped>
.site-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem 1.25rem;
  border-bottom: 1px solid #e5e7eb;
  flex-wrap: wrap;
}
.brand { font-weight: 700; text-decoration: none; color: #111; }
nav { display: flex; gap: 0.85rem; flex-wrap: wrap; }
nav a { color: #374151; text-decoration: none; }
nav a.router-link-active { color: #2563eb; }
</style>
