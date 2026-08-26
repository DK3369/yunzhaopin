<script setup lang="ts">
const api = useApi()
const { data: bal, error } = await useAsyncData('integral-bal', () =>
  api.post('/v1/mcenter/integral/balance', {}),
)
const { data: hist } = await useAsyncData('integral-hist', () =>
  api.post('/v1/mcenter/integral/history', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '我的积分' })
</script>

<template>
  <section>
    <h1>我的积分</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <p v-else>余额 {{ bal?.balance ?? 0 }}</p>
    <h2>流水</h2>
    <p v-if="!(hist?.list || []).length" class="muted">暂无记录</p>
    <div class="stack">
      <article v-for="(row, i) in hist?.list || []" :key="i" class="job-card">
        <pre>{{ JSON.stringify(row) }}</pre>
      </article>
    </div>
  </section>
</template>
