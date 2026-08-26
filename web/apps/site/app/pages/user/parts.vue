<script setup lang="ts">
const api = useApi()
const { data: applies, error } = await useAsyncData('my-part-applies', () =>
  api.post('/v1/mcenter/my-part-applications/list', { page: 1, page_size: 20 }),
)
const { data: collects } = await useAsyncData('my-part-collects', () =>
  api.post('/v1/mcenter/my-part-collects/list', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '兼职申请与收藏' })
</script>

<template>
  <section>
    <h1>兼职申请 / 收藏</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <h2>申请</h2>
    <p v-if="!(applies?.list || []).length" class="muted">暂无兼职申请</p>
    <div class="stack">
      <article v-for="row in applies?.list || []" :key="row.id" class="job-card">
        <h3>job_id {{ row.job_id }}</h3>
        <p class="muted">status {{ row.status }} · {{ row.ctime_n }}</p>
      </article>
    </div>
    <h2>收藏</h2>
    <p v-if="!(collects?.list || []).length" class="muted">暂无兼职收藏</p>
    <div class="stack">
      <article v-for="row in collects?.list || []" :key="row.id" class="job-card">
        <h3>job_id {{ row.job_id }}</h3>
        <p class="muted">{{ row.ctime_n }}</p>
      </article>
    </div>
  </section>
</template>
