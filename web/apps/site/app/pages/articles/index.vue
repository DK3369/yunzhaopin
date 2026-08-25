<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('articles', () =>
  api.get('/v1/wap/articles', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '资讯' })
</script>

<template>
  <section>
    <h1>资讯</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无资讯</p>
    <div class="stack">
      <NuxtLink v-for="a in data?.list || []" :key="a.id" :to="`/articles/${a.id}`">
        <h3>{{ a.title }}</h3>
        <p class="muted">{{ a.datetime_n || a.published_at_n }}</p>
      </NuxtLink>
    </div>
  </section>
</template>
