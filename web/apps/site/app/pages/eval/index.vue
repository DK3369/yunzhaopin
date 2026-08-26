<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const api = useApi()
const { data, error } = await useAsyncData(
  () => `eval-${page.value}`,
  () => api.get('/v1/wap/eval-papers', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: '职业测评' })
</script>

<template>
  <section>
    <h1>职业测评</h1>
    <p v-if="error" class="muted">测评列表暂时不可用。</p>
    <p v-else-if="!(data?.list || []).length" class="muted">暂无测评</p>
    <div v-else class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/eval/${row.id}`"
        :title="row.name"
        :meta="row.description || ''"
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
