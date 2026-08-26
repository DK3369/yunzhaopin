<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data: packs } = await useAsyncData('vip-packs', () => api.post('/v1/mcenter/vip/packages', {}))
const { data: orders, error, refresh } = await useAsyncData('vip-orders', () =>
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
      msg.value = `${orderNo} ${t('ui.mock_paid')}`
    } else {
      msg.value = t('ui.submitted')
    }
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.orders') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.orders') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.please_login_com') }}</p>
    <h2>{{ $t('ui.buyable') }}</h2>
    <p v-if="!packages.length" class="muted">{{ $t('ui.no_packages') }}</p>
    <div class="stack">
      <article v-for="p in packages" :key="p.code" class="job-card">
        <h3>{{ p.name }}</h3>
        <p class="muted">{{ p.price_yuan }} {{ $t('ui.yuan') }} / {{ p.duration_days }} {{ $t('ui.days') }}</p>
        <button type="button" @click="buy(p.code)">{{ $t('ui.buy_mock') }}</button>
      </article>
    </div>
    <h2>{{ $t('ui.my_orders') }}</h2>
    <p v-if="!(orders?.list || []).length" class="muted">{{ $t('ui.no_orders') }}</p>
    <div class="stack">
      <article v-for="o in orders?.list || []" :key="o.order_no" class="job-card">
        <h3>{{ o.order_no }}</h3>
        <p class="muted">{{ o.package_code }} · {{ o.status_n }} · {{ o.amount_yuan }} {{ $t('ui.yuan') }}</p>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
