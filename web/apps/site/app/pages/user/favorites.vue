<script setup lang="ts">
const api = useApi()
const kind = ref(1)
const { data, error, refresh } = await useAsyncData(
  () => `fav-${kind.value}`,
  () => api.post('/v1/mcenter/favorites/list', { kind: kind.value, page: 1, page_size: 20 }),
)
watch(kind, () => refresh())
async function remove(targetId: number) {
  await api.post('/v1/mcenter/favorites/remove', { kind: kind.value, target_id: targetId })
  refresh()
}
useSeoMeta({ title: '我的收藏' })
</script>

<template>
  <section>
    <h1>我的收藏</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <p>
      <button type="button" @click="kind = 1">职位</button>
      <button type="button" @click="kind = 2">企业</button>
      <button type="button" @click="kind = 3">用户</button>
    </p>
    <p v-if="!(data?.list || []).length" class="muted">暂无收藏</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.target_id" class="job-card">
        <h3>#{{ row.target_id }}</h3>
        <button type="button" @click="remove(row.target_id)">取消收藏</button>
      </article>
    </div>
  </section>
</template>
