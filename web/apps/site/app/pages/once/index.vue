<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const api = useApi()
const { data } = await useAsyncData(
  () => `once-${page.value}`,
  () => api.get('/v1/wap/once-jobs/list', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: '店铺招聘' })
</script>

<template>
  <section>
    <h1>店铺招聘</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无店铺招聘</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/once/${row.id}`"
        :title="row.companyname"
        :meta="`招聘 ${row.number} 人`"
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
