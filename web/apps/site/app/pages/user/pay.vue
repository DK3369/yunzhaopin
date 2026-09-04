<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
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
    const created = await api.post('/v1/mcenter/vip/orders', { package_code: code, channel: 'alipay' })
    if (created?.pay_url) {
      window.location.href = created.pay_url
      return
    }
    msg.value = created?.msg || created?.order_no || t('ui.load_failed')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.pay') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.pay') }}</h1>
    <p class="muted">{{ $t('ui.pay_hint') }}</p>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <div class="stack">
      <article v-for="p in packages" :key="p.code" class="job-card">
        <h3>{{ p.name }}</h3>
        <p class="muted">{{ p.price_yuan }} {{ $t('ui.yuan') }} / {{ p.duration_days }} {{ $t('ui.days') }}</p>
        <button type="button" @click="buy(p.code)">{{ $t('common.submit') }}</button>
      </article>
    </div>
    <h2>{{ $t('ui.orders') }}</h2>
    <p v-if="!(orders?.list || []).length" class="muted">{{ $t('ui.no_orders') }}</p>
    <div class="stack">
      <article v-for="o in orders?.list || []" :key="o.order_no" class="job-card">
        <h3>{{ o.order_no }}</h3>
        <p class="muted">{{ o.status_n }} · {{ o.amount_yuan }} {{ $t('ui.yuan') }}</p>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
