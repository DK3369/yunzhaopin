<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const api = useApi()
const { data } = await useAsyncData(
  () => `tiny-${page.value}`,
  () => api.get('/v1/wap/tiny-resumes/list', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: '普工简历' })
</script>

<template>
  <section>
    <h1>普工简历</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无普工简历</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/tiny/${row.id}`"
        :title="row.username"
        :meta="row.job"
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
