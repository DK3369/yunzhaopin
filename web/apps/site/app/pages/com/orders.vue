<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

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
useSeoMeta({ title: t('ui.orders') })
</script>

<template>
  <MemberPanel :title="$t('ui.orders')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <div class="payment_list">
      <div class="payment_list_s">{{ $t('ui.buyable') }}</div>
      <div class="payment_list_r">
        <p v-if="!packages.length" class="muted">{{ $t('ui.no_packages') }}</p>
        <span v-for="p in packages" :key="p.code" class="payment_list_text">
          <div class="payment_list_text_n">
            {{ p.name }}
            <em class="payment_list_text_dw">{{ p.price_yuan }} {{ $t('wap_00925') }} / {{ p.duration_days }} {{ $t('wap_01197') }}</em>
          </div>
          <input type="button" class="payment_list_other" :value="$t('common.submit')" @click="buy(p.code)" />
        </span>
      </div>
    </div>
    <MemberResumeH1 :title="$t('ui.my_orders')" />
    <p v-if="!(orders?.list || []).length" class="muted">{{ $t('ui.no_orders') }}</p>
    <div v-if="(orders?.list || []).length" class="site-pc paylist_tit">
      <span class="paylist_span paylist_span_dh">{{ $t('ui.order_no') }}</span>
      <span class="paylist_span paylist_span_money">{{ $t('wap_00925') }}</span>
      <span class="paylist_span paylist_span_zt">{{ $t('member_user_00104') }}</span>
    </div>
    <div v-for="o in orders?.list || []" :key="o.order_no" class="site-pc paylist_list">
      <span class="paylist_span paylist_span_dh">{{ o.order_no }}</span>
      <span class="paylist_span paylist_span_money">{{ o.amount_yuan }}</span>
      <span class="paylist_span paylist_span_zt">{{ o.status_n }} · {{ o.package_code }}</span>
    </div>
    <div class="site-h5 m_cardbox">
      <MemberSxNewsCard
        v-for="o in orders?.list || []"
        :key="'h5-' + o.order_no"
        :title="String(o.order_no)"
        :sub="String(o.amount_yuan)"
        :time="o.status_n"
      />
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
