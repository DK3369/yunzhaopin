<script setup lang="ts">
const api = useApi()
const { data: packs, error } = await useAsyncData('user-vip-packs', () =>
  api.post('/v1/mcenter/vip/packages', {}),
)
const { data: orders, refresh } = await useAsyncData('user-vip-orders', () =>
  api.post('/v1/mcenter/vip/orders/list', { page: 1, page_size: 20 }),
)
const msg = ref('')
const packages = computed(() => (Array.isArray(packs.value) ? packs.value : packs.value?.list || []))
async function buy(code: string) {
  msg.value = ''
  try {
    const created = await api.post('/v1/mcenter/vip/orders', { package_code: code, channel: 'stub' })
    const orderNo = created?.order_no
    if (orderNo) {
      await api.post('/v1/mcenter/vip/orders/mock-paid', { order_no: orderNo })
      msg.value = `订单 ${orderNo} 已模拟支付`
    }
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '下单失败'
  }
}
useSeoMeta({ title: '套餐支付' })
</script>

<template>
  <section>
    <h1>套餐支付</h1>
    <p class="muted">走已有 mcenter VIP；无沙箱密钥时用 mock-paid。真支付宝/微信回调见 /callback/*。</p>
    <p v-if="error" class="muted">请先登录。企业账号也可在 /com/orders 购买。</p>
    <div class="stack">
      <article v-for="p in packages" :key="p.code" class="job-card">
        <h3>{{ p.name }}</h3>
        <p class="muted">{{ p.price_yuan }} 元 / {{ p.duration_days }} 天</p>
        <button type="button" @click="buy(p.code)">购买并模拟支付</button>
      </article>
    </div>
    <h2>订单</h2>
    <p v-if="!(orders?.list || []).length" class="muted">暂无订单</p>
    <div class="stack">
      <article v-for="o in orders?.list || []" :key="o.order_no" class="job-card">
        <h3>{{ o.order_no }}</h3>
        <p class="muted">{{ o.status_n }} · {{ o.amount_yuan }} 元</p>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
