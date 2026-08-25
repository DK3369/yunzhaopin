<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const api = useApi()
const { data } = await useAsyncData(
  () => `questions-${page.value}`,
  () => api.get('/v1/wap/questions', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: '问答' })
</script>

<template>
  <section>
    <h1>问答</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无问答</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/questions/${row.id}`"
        :title="row.title"
        :meta="`${row.catname || ''} · ${row.answer_count || 0} 回答`"
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
