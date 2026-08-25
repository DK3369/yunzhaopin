<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const api = useApi()
const { data } = await useAsyncData(
  () => `specials-${page.value}`,
  () => api.get('/v1/wap/specials', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: '专题招聘' })
</script>

<template>
  <section>
    <h1>专题招聘</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无专题</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/specials/${row.id}`"
        :title="row.title"
        :meta="row.intro"
      />
    </div>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { page: p } })"
    />
  </section>
</template>
