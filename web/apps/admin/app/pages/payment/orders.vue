<script setup lang="ts">
import { ElMessage } from 'element-plus'
import { lc } from '~/utils/phpLc'

type Row = {
  id: number
  pay_no: string
  merchant_code: string
  merchant_name: string
  merchant_order_no: string
  method_code: string
  amount_cents: number
  amount_yuan: number
  currency: string
  status: string
  ctime_n: string
}

type Paged = { list: Row[]; total: number; page: number; page_size: number }

const api = useApi()
const loading = ref(false)
const rows = ref<Row[]>([])
const total = ref(0)
const page = ref(1)
const pageSize = ref(20)
const filter = reactive({ merchant_code: '', method_code: '', status: '' })

async function load() {
  loading.value = true
  try {
    const data = await api.post<Paged>('/v1/admin/pay/orders/list', {
      merchant_code: filter.merchant_code.trim(),
      method_code: filter.method_code.trim(),
      status: filter.status.trim(),
      page: page.value,
      page_size: pageSize.value,
    })
    rows.value = Array.isArray(data?.list) ? data.list : []
    total.value = Number(data?.total || 0)
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('admin_user_weipin_00026', null, '加载失败'))
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div class="moduleElenAl">
    <div class="moduleSeachs">
      <div>{{ lc('admin_pay_orders_title', null, '支付订单') }}</div>
      <div class="nrtopbtn">
        <el-input
          v-model="filter.merchant_code"
          size="small"
          style="width: 140px; margin-right: 8px"
          :placeholder="lc('admin_pay_merchant_code', null, '商户代码')"
          @keyup.enter="load"
        />
        <el-select
          v-model="filter.method_code"
          size="small"
          clearable
          style="width: 140px; margin-right: 8px"
          :placeholder="lc('admin_pay_method', null, '支付方式')"
        >
          <el-option value="stripe" label="Stripe" />
          <el-option value="gcash" label="GCash" />
          <el-option value="paymaya" label="PayMaya" />
        </el-select>
        <el-select
          v-model="filter.status"
          size="small"
          clearable
          style="width: 120px; margin-right: 8px"
          :placeholder="lc('admin_pay_status', null, '状态')"
        >
          <el-option value="pending" :label="lc('admin_pay_pending', null, '待支付')" />
          <el-option value="paid" :label="lc('admin_pay_paid', null, '已支付')" />
          <el-option value="failed" :label="lc('admin_pay_failed', null, '失败')" />
          <el-option value="cancelled" :label="lc('admin_pay_cancelled', null, '已取消')" />
        </el-select>
        <el-button size="small" type="primary" @click="load">{{ lc('admin_00004', null, '搜索') }}</el-button>
      </div>
    </div>
    <div class="moduleElTable">
      <el-table
        v-loading="loading"
        :data="rows"
        border
        height="100%"
        row-key="id"
        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
        :empty-text="lc('wap_js_00113', null, '暂无数据')"
      >
        <el-table-column prop="ctime_n" :label="lc('admin_pay_ctime', null, '订单创建时间')" width="180" />
        <el-table-column prop="pay_no" :label="lc('admin_pay_pay_no', null, '订单编号')" width="200" />
        <el-table-column prop="merchant_order_no" :label="lc('admin_pay_merchant_order', null, '商户单号')" width="180" />
        <el-table-column :label="lc('admin_pay_amount', null, '订单金额')" width="140">
          <template #default="{ row }">
            {{ row.amount_yuan }} {{ row.currency }}
          </template>
        </el-table-column>
        <el-table-column prop="method_code" :label="lc('admin_pay_method', null, '支付方式')" width="120" />
        <el-table-column prop="status" :label="lc('admin_pay_status', null, '状态')" width="110" />
        <el-table-column :label="lc('admin_pay_merchant', null, '商户')">
          <template #default="{ row }">
            {{ row.merchant_name }} ({{ row.merchant_code }})
          </template>
        </el-table-column>
      </el-table>
    </div>
    <div style="padding: 8px 12px; text-align: right">
      <el-pagination
        v-model:current-page="page"
        v-model:page-size="pageSize"
        :total="total"
        layout="total, prev, pager, next"
        @current-change="load"
      />
    </div>
  </div>
</template>
