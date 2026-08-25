<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const api = useApi()
const { data } = await useAsyncData(
  () => `redeem-${page.value}`,
  () => api.get('/v1/wap/redeem/rewards', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: '积分商城' })
</script>

<template>
  <section>
    <h1>积分商城</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无兑换商品</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/redeem/${row.id}`"
        :title="row.name"
        :meta="`${row.integral} 积分`"
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
