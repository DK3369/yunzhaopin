<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const api = useApi()
const { data } = await useAsyncData(
  () => `jobs-${page.value}-${keyword.value}`,
  () =>
    api.get('/v1/wap/jobs', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
    }),
)
const list = computed(() => data.value?.list || [])
useSeoMeta({ title: keyword.value ? `${keyword.value} - 职位` : '职位列表' })
</script>

<template>
  <section>
    <h1>职位</h1>
    <form method="get" action="/jobs">
      <input name="keyword" :value="keyword" placeholder="关键词" />
      <button type="submit">搜索</button>
    </form>
    <p v-if="!list.length" class="muted">暂无职位</p>
    <div class="stack">
      <JobCard v-for="job in list" :key="job.id" :job="job" />
    </div>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </section>
</template>
