<script setup lang="ts">
import { ElMessage } from 'element-plus'
import { lc } from '~/utils/phpLc'

type Overview = {
  paid_today_n: number
  paid_today_cents: number
  paid_7d_n: number
  paid_7d_cents: number
  pending_n: number
  paid_n: number
  failed_n: number
  cancelled_n: number
  refunded_n: number
  charge_ready_n: number
  notify_fail_n: number
}

const api = useApi()
const loading = ref(false)
const data = ref<Overview | null>(null)

function yuan(cents: number) {
  return (Number(cents || 0) / 100).toFixed(2)
}

async function load() {
  loading.value = true
  try {
    data.value = await api.post<Overview>('/v1/admin/pay/overview', {})
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('admin_user_weipin_00026', null, '加载失败'))
  } finally {
    loading.value = false
  }
}

function goOrders(status: string) {
  navigateTo({ path: '/payment/orders', query: status ? { status } : {} })
}

function goNotifies() {
  navigateTo({ path: '/payment/notifies', query: { ok: '0' } })
}

onMounted(load)
</script>

<template>
  <div class="pay-page" v-loading="loading">
    <div class="moduleSeachs">
      <div class="pay-bar">
        <span>{{ lc('admin_pay_overview_title', null, '支付概览') }}</span>
        <span class="pay-hint">{{ lc('admin_pay_overview_hint', null, 'Stripe 真收；PayPal / GrabPay / GCash / PayMaya 目录占位。') }}</span>
      </div>
    </div>
    <div v-if="data" class="pay-overview">
      <button type="button" class="pay-card" @click="goOrders('paid')">
        <div class="pay-card-k">{{ lc('admin_pay_today_paid', null, '今日已付') }}</div>
        <div class="pay-card-v">{{ data.paid_today_n }}</div>
        <div class="pay-card-s">{{ yuan(data.paid_today_cents) }}</div>
      </button>
      <button type="button" class="pay-card" @click="goOrders('paid')">
        <div class="pay-card-k">{{ lc('admin_pay_days_7_paid', null, '近7日已付') }}</div>
        <div class="pay-card-v">{{ data.paid_7d_n }}</div>
        <div class="pay-card-s">{{ yuan(data.paid_7d_cents) }}</div>
      </button>
      <button type="button" class="pay-card" @click="goOrders('pending')">
        <div class="pay-card-k">{{ lc('admin_pay_pending', null, '待支付') }}</div>
        <div class="pay-card-v">{{ data.pending_n }}</div>
      </button>
      <button type="button" class="pay-card" @click="goOrders('paid')">
        <div class="pay-card-k">{{ lc('admin_pay_paid', null, '已支付') }}</div>
        <div class="pay-card-v">{{ data.paid_n }}</div>
      </button>
      <button type="button" class="pay-card" @click="goOrders('refunded')">
        <div class="pay-card-k">{{ lc('admin_pay_refunded', null, '已退款') }}</div>
        <div class="pay-card-v">{{ data.refunded_n }}</div>
      </button>
      <button type="button" class="pay-card" @click="goOrders('failed')">
        <div class="pay-card-k">{{ lc('admin_pay_failed', null, '失败') }}</div>
        <div class="pay-card-v">{{ data.failed_n }}</div>
      </button>
      <button type="button" class="pay-card" @click="goOrders('cancelled')">
        <div class="pay-card-k">{{ lc('admin_pay_cancelled', null, '已取消') }}</div>
        <div class="pay-card-v">{{ data.cancelled_n }}</div>
      </button>
      <button type="button" class="pay-card" @click="navigateTo('/payment/methods')">
        <div class="pay-card-k">{{ lc('admin_pay_charge_ready_n', null, '可收款渠道') }}</div>
        <div class="pay-card-v">{{ data.charge_ready_n }}</div>
      </button>
      <button type="button" class="pay-card" @click="goNotifies">
        <div class="pay-card-k">{{ lc('admin_pay_notify_fail_n', null, '回调失败') }}</div>
        <div class="pay-card-v">{{ data.notify_fail_n }}</div>
      </button>
    </div>
  </div>
</template>

<style scoped>
.pay-bar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  width: 100%;
}
.pay-hint {
  color: #909399;
  font-weight: 400;
  font-size: 12px;
}
.pay-overview {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 12px;
  padding: 12px;
}
.pay-card {
  text-align: left;
  background: #fff;
  border: 1px solid #ebeef5;
  border-radius: 6px;
  padding: 14px 16px;
  cursor: pointer;
}
.pay-card:hover {
  border-color: #409eff;
}
.pay-card-k {
  color: #909399;
  font-size: 12px;
}
.pay-card-v {
  font-size: 22px;
  font-weight: 600;
  margin-top: 6px;
}
.pay-card-s {
  color: #606266;
  font-size: 13px;
  margin-top: 4px;
}
</style>
