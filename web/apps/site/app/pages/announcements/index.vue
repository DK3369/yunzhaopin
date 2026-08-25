<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('announcements', () =>
  api.get('/v1/wap/announcements', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '公告' })
</script>

<template>
  <section>
    <h1>公告</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无公告</p>
    <div class="stack">
      <NuxtLink v-for="a in data?.list || []" :key="a.id" :to="`/announcements/${a.id}`">
        {{ a.title }}
      </NuxtLink>
    </div>
  </section>
</template>
