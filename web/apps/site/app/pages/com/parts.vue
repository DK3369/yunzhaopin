<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('com-parts', () =>
  api.post('/v1/mcenter/com-parts/list', { page: 1, page_size: 20 }),
)
const { data: applies } = await useAsyncData('com-part-applies', () =>
  api.post('/v1/mcenter/com-part-applications', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '企业兼职' })
</script>

<template>
  <section>
    <h1>企业兼职</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <h2>已发布</h2>
    <p v-if="!(data?.list || []).length" class="muted">暂无兼职</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.name || row.id }}</h3>
      </article>
    </div>
    <h2>收到的申请</h2>
    <p v-if="!(applies?.list || []).length" class="muted">暂无申请</p>
    <div class="stack">
      <article v-for="row in applies?.list || []" :key="row.id" class="job-card">
        <h3>uid {{ row.uid }} · job_id {{ row.job_id }}</h3>
        <p class="muted">status {{ row.status }}</p>
      </article>
    </div>
  </section>
</template>
