<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('my-apps', () =>
  api.post('/v1/mcenter/my-applications', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])
useSeoMeta({ title: '投递记录' })
</script>

<template>
  <section>
    <h1>投递记录</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <p v-else-if="!list.length" class="muted">暂无投递</p>
    <div v-else class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>职位 #{{ row.job_id }}</h3>
        <p class="muted">{{ row.datetime_n }} · {{ row.invited ? '已邀面试' : row.employer_viewed ? '企业已查看' : '待查看' }}</p>
        <NuxtLink :to="`/jobs/${row.job_id}`">查看职位</NuxtLink>
      </article>
    </div>
  </section>
</template>
