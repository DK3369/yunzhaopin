<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`reward-${id}`, () =>
  api.get('/v1/wap/redeem/rewards/detail', { id }),
)
useSeoMeta({ title: () => String(data.value?.name || '兑换详情') })
useHead({ link: [{ rel: 'canonical', href: `/redeem/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || '商品不存在' }}</h1>
    <p v-if="data?.integral" class="muted">{{ data.integral }} 积分 · 库存 {{ data.remaining }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.name" class="muted">没有这件兑换商品，或暂时无法加载。</p>
  </article>
</template>
