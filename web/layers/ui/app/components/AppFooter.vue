<template>
  <footer class="site-footer">
    <nav>
      <NuxtLink to="/gongzhao">公招</NuxtLink>
      <NuxtLink to="/specials">专题</NuxtLink>
      <NuxtLink to="/once">店铺招聘</NuxtLink>
      <NuxtLink to="/tiny">普工简历</NuxtLink>
      <NuxtLink to="/redeem">积分商城</NuxtLink>
      <NuxtLink to="/hr">HR 工具</NuxtLink>
      <NuxtLink to="/map">地图</NuxtLink>
      <NuxtLink to="/links">友情链接</NuxtLink>
      <NuxtLink to="/pages/about">关于我们</NuxtLink>
      <NuxtLink to="/pages/privacy">隐私政策</NuxtLink>
      <NuxtLink to="/pages/protocol">用户协议</NuxtLink>
      <NuxtLink to="/pages/contact">联系我们</NuxtLink>
    </nav>
    <p>© {{ new Date().getFullYear() }} {{ siteName }}</p>
  </footer>
</template>

<script setup lang="ts">
const siteName = useRuntimeConfig().public.siteName
</script>

<style scoped>
.site-footer {
  margin-top: 3rem;
  padding: 1.5rem;
  text-align: center;
  color: #6b7280;
  border-top: 1px solid #e5e7eb;
}
nav {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem 1rem;
  justify-content: center;
  margin-bottom: 0.75rem;
}
nav a { color: #374151; text-decoration: none; }
p { margin: 0; }
</style>
