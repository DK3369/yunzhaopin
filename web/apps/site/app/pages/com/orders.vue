<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: orders, error, refresh } = await useAsyncData('vip-orders', () =>
  api.post('/v1/mcenter/vip/orders/list', { page: 1, page_size: 20 }),
)
const msg = ref('')
function canCancel(o: { status?: number }) {
  return o.status === 0
}
async function cancelOrder(o: { order_no?: string }) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/vip/orders/cancel', { order_no: String(o.order_no || '') })
    msg.value = t('common.success')
    await refresh()
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
    <p v-if="!(orders?.list || []).length" class="muted">{{ $t('ui.no_orders') }}</p>
    <div v-if="(orders?.list || []).length" class="site-pc paylist_tit">
      <span class="paylist_span paylist_dh">{{ $t('ui.order_no') }}</span>
      <span class="paylist_span paylist_money">{{ $t('wap_00925') }}</span>
      <span class="paylist_span paylist_zt">{{ $t('member_user_00104') }}</span>
    </div>
    <div v-for="o in orders?.list || []" :key="o.order_no" class="site-pc paylist_list">
      <span class="paylist_span paylist_dh">{{ o.order_no }}</span>
      <span class="paylist_span paylist_money">{{ o.amount_yuan }}</span>
      <span class="paylist_span paylist_zt">{{ o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n }}</span>
      <span class="paylist_span paylist_cz">
        <a v-if="canCancel(o)" href="javascript:;" class="cblue" @click="cancelOrder(o)">{{ $t('common.cancel') }}</a>
      </span>
    </div>
    <div class="site-h5 detail_body">
      <div v-if="(orders?.list || []).length" class="detail_body_card">
        <ul>
          <li v-for="o in orders?.list || []" :key="'h5-' + o.order_no">
            <div class="detail_box">
              <div class="detail_box_title">{{ o.order_no }}</div>
              <div class="detail_box_time">{{ o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n }}</div>
            </div>
            <div class="detail_integral">{{ o.amount_yuan }}</div>
            <div class="detail_box_cz">
              <a v-if="canCancel(o)" href="javascript:;" @click="cancelOrder(o)">{{ $t('common.cancel') }}</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
