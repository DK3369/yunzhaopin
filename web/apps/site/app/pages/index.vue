<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('home', () => api.get('/v1/wap/home', { did: 0 }))
useSeoMeta({
  title: '招聘首页',
  description: '职位、企业、资讯聚合',
})
useHead({
  link: [{ rel: 'canonical', href: '/' }],
})
</script>

<template>
  <section>
    <h1>找工作，从这里开始</h1>
    <p v-if="error" class="muted">首页数据暂时不可用，请确认 Rust 后端已启动。</p>
    <div v-else class="stack">
    <h2>热门职位</h2>
      <p v-if="!(data?.hot_jobs || []).length" class="muted">暂无热门职位</p>
      <JobCard v-for="job in data?.hot_jobs || []" :key="job.id" :job="job" />
      <h2>推荐企业</h2>
      <p v-if="!(data?.rec_companies || []).length" class="muted">暂无推荐企业</p>
      <CompanyCard v-for="c in data?.rec_companies || []" :key="c.uid" :company="c" />
    </div>
  </section>
</template>
