<template>
  <footer class="site-footer">© {{ new Date().getFullYear() }} {{ siteName }}</footer>
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
</style>
