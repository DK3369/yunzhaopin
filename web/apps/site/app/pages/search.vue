<script setup lang="ts">
const q = ref('')
const type = ref('job')
const api = useApi()
const { data, refresh } = await useAsyncData('search', () =>
  q.value
    ? api.get('/v1/wap/search', { keyword: q.value, type: type.value })
    : Promise.resolve(null),
)
useSeoMeta({ title: '搜索' })
</script>

<template>
  <section>
    <h1>搜索</h1>
    <form class="form" @submit.prevent="refresh()">
      <select v-model="type">
        <option value="job">职位</option>
        <option value="company">企业</option>
        <option value="resume">简历</option>
      </select>
      <input v-model="q" placeholder="关键词" />
      <button type="submit">搜索</button>
    </form>
    <pre class="muted">{{ JSON.stringify(data, null, 2) }}</pre>
  </section>
</template>
