<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = ref(String(route.query.keyword || ''))
const api = useApi()
const { data } = await useAsyncData(
  () => `jobs-${page.value}-${route.query.keyword || ''}`,
  () =>
    api.get('/v1/wap/jobs', {
      page: page.value,
      page_size: 20,
      keyword: String(route.query.keyword || '') || undefined,
    }),
)
useSeoMeta({ title: '职位列表' })
function search() {
  navigateTo({ query: { keyword: keyword.value, page: 1 } })
}
</script>

<template>
  <section>
    <h1>职位</h1>
    <form @submit.prevent="search">
      <input v-model="keyword" placeholder="关键词" />
    </form>
    <div class="stack">
      <JobCard v-for="job in data?.list || []" :key="job.id" :job="job" />
    </div>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </section>
</template>
