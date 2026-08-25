<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const api = useApi()
const { data } = await useAsyncData(
  () => `hr-${page.value}`,
  () => api.get('/v1/wap/hr-docs', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: 'HR 工具' })
</script>

<template>
  <section>
    <h1>HR 工具</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无 HR 文档</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/hr/${row.id}`"
        :title="row.name"
        :meta="row.created_at_n"
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
