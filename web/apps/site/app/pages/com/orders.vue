<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type OrderRow = {
  order_no?: string
  amount_yuan?: number
  status?: number
  status_n?: string
  order_kind?: number
  created_at?: number
  created_at_n?: string
  kind?: string
  name?: string
  id?: number
}

const api = useApi()
const { t } = useI18n()
const { data: vip, error, refresh: refreshVip } = await useAsyncData('vip-orders', () =>
  api.post<{ list: OrderRow[] }>('/v1/mcenter/vip/orders/list', { page: 1, page_size: 50 }).catch(() => ({ list: [] })),
)
const { data: packs, refresh: refreshPacks } = await useAsyncData('pack-orders', () =>
  api.post<{ list: OrderRow[] }>('/v1/mcenter/packs/orders/list', { page: 1, page_size: 50 }).catch(() => ({ list: [] })),
)
const { data: redeem, refresh: refreshRedeem } = await useAsyncData('redeem-orders-merge', () =>
  api.post<{ list: Array<{ id: number; name: string; integral: number; status: number; status_n: string; created_at: number; created_at_n: string }> }>(
    '/v1/mcenter/redeem/orders',
    { page: 1, page_size: 50 },
  ).catch(() => ({ list: [] })),
)
const msg = ref('')
const merged = computed<OrderRow[]>(() => {
  const a: OrderRow[] = (vip.value?.list || []).map((o) => ({ ...o, kind: 'vip' }))
  const b: OrderRow[] = (packs.value?.list || []).map((o) => ({ ...o, kind: 'pack' }))
  const c: OrderRow[] = (redeem.value?.list || []).map((o) => ({
    order_no: String(o.id),
    amount_yuan: o.integral,
    status: o.status,
    status_n: o.status_n,
    created_at: o.created_at,
    created_at_n: o.created_at_n,
    kind: 'redeem',
    name: o.name,
    id: o.id,
  }))
  return [...a, ...b, ...c].sort((x, y) => Number(y.created_at || 0) - Number(x.created_at || 0))
})
function kindLabel(o: OrderRow) {
  if (o.kind === 'pack') return t('wap_com_00393')
  if (o.kind === 'redeem') return t('wap_user_00170')
  if (o.order_kind === 2) return t('common_01946')
  return t('wap_com_00380')
}
function canCancel(o: OrderRow) {
  return o.status === 0 && o.kind !== 'pack'
}
function canPay(o: OrderRow) {
  return o.status === 0 && o.kind !== 'redeem' && Boolean(o.order_no)
}
async function cancelOrder(o: OrderRow) {
  msg.value = ''
  try {
    if (o.kind === 'redeem' && o.id) {
      await api.post('/v1/mcenter/redeem/orders/cancel', { id: o.id })
    } else {
      await api.post('/v1/mcenter/vip/orders/cancel', { order_no: String(o.order_no || '') })
    }
    msg.value = t('common.success')
    await Promise.all([refreshVip(), refreshPacks(), refreshRedeem()])
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('common_02029') })
</script>

<template>
  <MemberPanel :title="$t('common_02029')" :error="error && !isUnauthErr(error) ? error : undefined">
    <template #pcTabs><MemberComVipTabs /></template>
    <template #h5Tabs><MemberComVipTabs /></template>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-if="!merged.length" class="muted">{{ $t('ui.no_orders') }}</p>
    <div v-if="merged.length" class="site-pc paylist_tit">
      <span class="paylist_span paylist_dh">{{ $t('ui.order_no') }}</span>
      <span class="paylist_span paylist_money">{{ $t('wap_00925') }}</span>
      <span class="paylist_span paylist_zt">{{ $t('member_user_00104') }}</span>
    </div>
    <div v-for="o in merged" :key="(o.kind || '') + (o.order_no || o.id)" class="site-pc paylist_list">
      <span class="paylist_span paylist_dh">{{ o.name || o.order_no }} · {{ kindLabel(o) }}</span>
      <span class="paylist_span paylist_money">{{ o.amount_yuan }}</span>
      <span class="paylist_span paylist_zt">{{ o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n }}</span>
      <span class="paylist_span paylist_cz">
        <NuxtLink v-if="canPay(o)" :to="`/com/cashier/${o.order_no}`" class="cblue">{{ $t('wap_00401') }}</NuxtLink>
        <a v-if="canCancel(o)" href="javascript:;" class="cblue" @click="cancelOrder(o)">{{ $t('common.cancel') }}</a>
      </span>
    </div>
    <div class="site-h5 detail_body">
      <div v-if="merged.length" class="detail_body_card">
        <ul>
          <li v-for="o in merged" :key="'h5-' + (o.kind || '') + (o.order_no || o.id)">
            <div class="detail_box">
              <div class="detail_box_title">{{ o.name || o.order_no }} · {{ kindLabel(o) }}</div>
              <div class="detail_box_time">{{ o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n }}</div>
            </div>
            <div class="detail_integral">{{ o.amount_yuan }}</div>
            <div class="detail_box_cz">
              <NuxtLink v-if="canPay(o)" :to="`/com/cashier/${o.order_no}`">{{ $t('wap_00401') }}</NuxtLink>
              <a v-if="canCancel(o)" href="javascript:;" @click="cancelOrder(o)">{{ $t('common.cancel') }}</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
