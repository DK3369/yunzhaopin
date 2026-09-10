<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { data: packs, error } = await useAsyncData('user-vip-packs', () =>
  api.post('/v1/mcenter/vip/packages', {}),
)
const { data: orders, refresh } = await useAsyncData('user-vip-orders', () =>
  api.post('/v1/mcenter/vip/orders/list', { page: 1, page_size: 20 }),
)
const msg = ref('')
const channel = ref('alipay')
const bankOrderNo = ref('')
const bankForm = reactive({
  bank_name: '',
  bank_number: '',
  bank_price: '',
  bank_time: '',
  order_remark: '',
  order_pic: '',
})
const { data: banks } = await useAsyncData('user-vip-banks', () =>
  api.post<{ id: number; name: string; bank_name: string; bank_number: string; bank_address: string }[]>(
    '/v1/mcenter/vip/bank-accounts',
    {},
  ).catch(() => []),
)
const bankList = computed(() => (Array.isArray(banks.value) ? banks.value : []))
const wxPayOn = computed(() =>
  Boolean(settings.value.sy_wxpayid || settings.value.sy_wxpaykey || settings.value.wx_appid),
)
function payChannel() {
  if (channel.value === 'bank') return 'bank'
  if (channel.value !== 'wxpay') return channel.value
  if (import.meta.client && /Android|iPhone|iPad|Mobile|MicroMessenger/i.test(navigator.userAgent)) return 'wxh5'
  return 'wxpay'
}
const packages = computed(() => (Array.isArray(packs.value) ? packs.value : packs.value?.list || []))
async function buy(code: string) {
  msg.value = ''
  try {
    const created = await api.post('/v1/mcenter/vip/orders', { package_code: code, channel: payChannel() })
    if (channel.value === 'bank') {
      bankOrderNo.value = String(created?.order_no || '')
      bankForm.bank_price = String(created?.amount_yuan ?? '')
      msg.value = created?.order_no || t('common.success')
      await refresh()
      return
    }
    if (created?.pay_url) {
      window.location.href = created.pay_url
      return
    }
    msg.value = created?.msg || created?.order_no || t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function onVoucher(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/cert', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    bankForm.order_pic = r.key || r.url || ''
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function submitBank() {
  msg.value = ''
  if (!bankOrderNo.value) return
  try {
    let bank_time = 0
    if (bankForm.bank_time) {
      bank_time = Math.floor(new Date(`${bankForm.bank_time}T00:00:00`).getTime() / 1000)
    }
    await api.post('/v1/mcenter/vip/orders/paybank', {
      order_no: bankOrderNo.value,
      bank_name: bankForm.bank_name,
      bank_number: bankForm.bank_number,
      bank_price: bankForm.bank_price,
      bank_time,
      order_remark: bankForm.order_remark,
      order_pic: bankForm.order_pic,
    })
    msg.value = t('common.success')
    bankOrderNo.value = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function canFillBank(o: { channel?: string; status?: number; status_n?: string }) {
  return o.channel === 'bank' && (o.status === 0 || o.status === 3 || o.status_n === 'awaiting_confirm')
}
function fillBank(o: { order_no?: string; amount_yuan?: number }) {
  bankOrderNo.value = String(o.order_no || '')
  bankForm.bank_price = String(o.amount_yuan ?? '')
}
useSeoMeta({ title: t('ui.pay') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.pay') }}</h1>
    <p class="muted">{{ $t('ui.pay_hint') }}</p>
    <p>
      <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
      <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
      <label v-if="bankList.length"><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
    </p>
    <div v-if="channel === 'bank' && bankList.length" class="stack">
      <article v-for="b in bankList" :key="b.id" class="job-card">
        <h3>{{ b.name }}</h3>
        <p class="muted">{{ b.bank_name }} {{ b.bank_number }}</p>
        <p v-if="b.bank_address" class="muted">{{ b.bank_address }}</p>
      </article>
    </div>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <div class="stack">
      <article v-for="p in packages" :key="p.code" class="job-card">
        <h3>{{ p.name }}</h3>
        <p class="muted">{{ p.price_yuan }} {{ $t('wap_00925') }} / {{ p.duration_days }} {{ $t('wap_01197') }}</p>
        <button type="button" @click="buy(p.code)">{{ $t('common.submit') }}</button>
      </article>
    </div>
    <form v-if="bankOrderNo" class="form" @submit.prevent="submitBank">
      <p class="muted">{{ $t('ui.order_no') }} {{ bankOrderNo }}</p>
      <input v-model="bankForm.bank_name" :placeholder="$t('model_00022')" required />
      <input v-model="bankForm.bank_number" :placeholder="$t('model_00023')" required />
      <input v-model="bankForm.bank_price" :placeholder="$t('model_00024')" required />
      <input v-model="bankForm.bank_time" type="date" required />
      <input v-model="bankForm.order_remark" :placeholder="$t('wap_com_00345')" />
      <input type="file" accept="image/jpeg,image/png,image/webp" @change="onVoucher" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('ui.orders') }}</h2>
    <p v-if="!(orders?.list || []).length" class="muted">{{ $t('ui.no_orders') }}</p>
    <div class="stack">
      <article v-for="o in orders?.list || []" :key="o.order_no" class="job-card">
        <h3>{{ o.order_no }}</h3>
        <p class="muted">{{ o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n }} · {{ o.amount_yuan }} {{ $t('wap_00925') }}</p>
        <button v-if="canFillBank(o)" type="button" @click="fillBank(o)">{{ $t('wap_01805') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
