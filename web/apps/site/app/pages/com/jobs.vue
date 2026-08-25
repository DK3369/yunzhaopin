<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('com-jobs', () =>
  api.post('/v1/mcenter/jobs/list', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])
useSeoMeta({ title: '职位管理' })
</script>

<template>
  <section>
    <h1>职位管理</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <p><NuxtLink to="/com/jobs/new">发布职位</NuxtLink></p>
    <p v-if="!error && !list.length" class="muted">暂无职位</p>
    <div class="stack">
      <article v-for="job in list" :key="job.id" class="job-card">
        <h3>{{ job.name }}</h3>
        <p class="muted">状态 {{ job.state }}</p>
      </article>
    </div>
  </section>
</template>
