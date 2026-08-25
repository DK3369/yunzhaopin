<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('links', () => api.get('/v1/wap/friend-links'))
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as { id: number; name: string; url: string }[])
useSeoMeta({ title: '友情链接' })
</script>

<template>
  <section>
    <h1>友情链接</h1>
    <p v-if="!list.length" class="muted">暂无友情链接</p>
    <ul v-else class="stack">
      <li v-for="row in list" :key="row.id">
        <a :href="row.url" rel="nofollow noopener" target="_blank">{{ row.name }}</a>
      </li>
    </ul>
  </section>
</template>
