<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('vip-orders', () =>
  api.post('/v1/mcenter/vip/orders/list', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: '套餐订单' })
</script>

<template>
  <section>
    <h1>套餐订单</h1>
    <pre>{{ JSON.stringify(data, null, 2) }}</pre>
  </section>
</template>
