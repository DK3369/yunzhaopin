<script setup lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus'
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
  channel_ref: string
  pay_url: string
  subject: string
  paid_at_n: string
  ctime_n: string
}

type Merchant = { id: number; code: string; name: string }
type Channel = { code: string; name: string; live: boolean }
type Paged = { list: Row[]; total: number; page: number; page_size: number }

const api = useApi()
const route = useRoute()
const loading = ref(false)
const rows = ref<Row[]>([])
const merchants = ref<Merchant[]>([])
const channels = ref<Channel[]>([])
const total = ref(0)
const page = ref(1)
const pageSize = ref(20)
const kwType = ref<'pay_no' | 'merchant_order_no' | 'channel_ref'>('pay_no')
const keyword = ref('')
const filter = reactive({ merchant_code: 'all', method_code: 'all', status: 'all' })

function allOr(v: string) {
  const s = (v || '').trim()
  return !s || s === 'all' ? '' : s
}
const dateRange = ref<[Date, Date] | null>(null)
const detailOpen = ref(false)
const detail = ref<Row | null>(null)

function startOfDay(d: Date) {
  const x = new Date(d)
  x.setHours(0, 0, 0, 0)
  return x
}
function endOfDay(d: Date) {
  const x = new Date(d)
  x.setHours(23, 59, 59, 0)
  return x
}
function startOfWeek(d: Date) {
  const x = startOfDay(d)
  const day = x.getDay()
  x.setDate(x.getDate() - (day === 0 ? 6 : day - 1))
  return x
}
function startOfMonth(d: Date) {
  return new Date(d.getFullYear(), d.getMonth(), 1, 0, 0, 0, 0)
}

const dateShortcuts = computed(() => [
  {
    text: lc('admin_pay_days_7', null, '近7天'),
    value: () => {
      const end = endOfDay(new Date())
      const start = startOfDay(new Date())
      start.setDate(start.getDate() - 6)
      return [start, end]
    },
  },
  {
    text: lc('admin_user_00146', null, '本周'),
    value: () => [startOfWeek(new Date()), endOfDay(new Date())],
  },
  {
    text: lc('admin_user_00142', null, '上周'),
    value: () => {
      const thisMon = startOfWeek(new Date())
      const lastMon = new Date(thisMon)
      lastMon.setDate(lastMon.getDate() - 7)
      const lastSun = new Date(thisMon)
      lastSun.setDate(lastSun.getDate() - 1)
      return [lastMon, endOfDay(lastSun)]
    },
  },
  {
    text: lc('admin_user_00147', null, '本月'),
    value: () => [startOfMonth(new Date()), endOfDay(new Date())],
  },
  {
    text: lc('admin_user_00143', null, '上月'),
    value: () => {
      const firstThis = startOfMonth(new Date())
      const lastMonthEnd = new Date(firstThis.getTime() - 1)
      return [startOfMonth(lastMonthEnd), endOfDay(lastMonthEnd)]
    },
  },
])

function unixRange(): { ctime_from: number; ctime_to: number } {
  if (!dateRange.value || dateRange.value.length !== 2) {
    return { ctime_from: 0, ctime_to: 0 }
  }
  const a = startOfDay(new Date(dateRange.value[0]))
  const b = endOfDay(new Date(dateRange.value[1]))
  return {
    ctime_from: Math.floor(a.getTime() / 1000),
    ctime_to: Math.floor(b.getTime() / 1000),
  }
}

async function loadMerchants() {
  const data = await api.post<Merchant[]>('/v1/admin/pay/merchants/list', {})
  merchants.value = Array.isArray(data) ? data : []
}

async function loadChannels() {
  const data = await api.post<Channel[]>('/v1/admin/pay/channels/list', {})
  channels.value = Array.isArray(data) ? data : []
}

async function load() {
  loading.value = true
  try {
    const range = unixRange()
    const kw = keyword.value.trim()
    const data = await api.post<Paged>('/v1/admin/pay/orders/list', {
      merchant_code: allOr(filter.merchant_code),
      method_code: allOr(filter.method_code),
      status: allOr(filter.status),
      pay_no: kwType.value === 'pay_no' ? kw : '',
      merchant_order_no: kwType.value === 'merchant_order_no' ? kw : '',
      channel_ref: kwType.value === 'channel_ref' ? kw : '',
      ctime_from: range.ctime_from,
      ctime_to: range.ctime_to,
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

function search() {
  page.value = 1
  load()
}

function openDetail(row: Row) {
  detail.value = row
  detailOpen.value = true
}

function statusText(s: string) {
  if (s === 'pending') return lc('admin_pay_pending', null, '待支付')
  if (s === 'paid') return lc('admin_pay_paid', null, '已支付')
  if (s === 'failed') return lc('admin_pay_failed', null, '失败')
  if (s === 'cancelled') return lc('admin_pay_cancelled', null, '已取消')
  if (s === 'refunded') return lc('admin_pay_refunded', null, '已退款')
  return s
}

async function closeRow(row: Row) {
  try {
    await ElMessageBox.confirm(
      lc('admin_pay_close_confirm', [row.pay_no], `关闭订单 ${row.pay_no}？`),
      lc('common_01520', null, '提示'),
      { type: 'warning' },
    )
  } catch {
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/pay/orders/close', { pay_no: row.pay_no })
    ElMessage.success(lc('wap_00225', null, '已保存'))
    await load()
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('wap_00225', null, '保存失败'))
  } finally {
    loading.value = false
  }
}

async function refundRow(row: Row) {
  try {
    await ElMessageBox.confirm(
      lc('admin_pay_refund_confirm', [row.pay_no], `退款 ${row.pay_no}？不会冲会员套餐。`),
      lc('common_01520', null, '提示'),
      { type: 'warning' },
    )
  } catch {
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/pay/orders/refund', { pay_no: row.pay_no })
    ElMessage.success(lc('wap_00225', null, '已保存'))
    await load()
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('wap_00225', null, '保存失败'))
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  const st = String(route.query.status || '')
  if (['pending', 'paid', 'failed', 'cancelled', 'refunded'].includes(st)) {
    filter.status = st
  }
  try {
    await Promise.all([loadMerchants(), loadChannels()])
  } catch {
    merchants.value = []
    channels.value = []
  }
  await load()
})
</script>

<template>
  <div class="pay-page">
    <div class="moduleSeachs">
      <div class="pay-bar">
        <el-input
          v-model="keyword"
          size="small"
          clearable
          class="pay-kw"
          :placeholder="lc('admin_pay_kw', null, '关键词')"
          @keyup.enter="search"
        >
          <template #prepend>
            <el-select v-model="kwType" style="width: 120px">
              <el-option :label="lc('admin_pay_pay_no', null, '订单编号')" value="pay_no" />
              <el-option :label="lc('admin_pay_merchant_order', null, '商户单号')" value="merchant_order_no" />
              <el-option :label="lc('admin_pay_channel_ref', null, '三方ID')" value="channel_ref" />
            </el-select>
          </template>
        </el-input>
        <el-select
          v-model="filter.merchant_code"
          size="small"
          style="width: 140px"
          :placeholder="lc('admin_pay_merchant', null, '商户')"
          @change="search"
        >
          <el-option :label="lc('admin_pay_all', null, '全部')" value="all" />
          <el-option
            v-for="m in merchants"
            :key="m.id"
            :label="`${m.name} (${m.code})`"
            :value="m.code"
          />
        </el-select>
        <el-select
          v-model="filter.method_code"
          size="small"
          style="width: 130px"
          :placeholder="lc('admin_pay_method', null, '支付方式')"
          @change="search"
        >
          <el-option :label="lc('admin_pay_all', null, '全部')" value="all" />
          <el-option v-for="c in channels" :key="c.code" :label="c.name" :value="c.code" />
        </el-select>
        <el-select
          v-model="filter.status"
          size="small"
          style="width: 120px"
          :placeholder="lc('admin_pay_status', null, '状态')"
          @change="search"
        >
          <el-option :label="lc('admin_pay_all', null, '全部')" value="all" />
          <el-option value="pending" :label="lc('admin_pay_pending', null, '待支付')" />
          <el-option value="paid" :label="lc('admin_pay_paid', null, '已支付')" />
          <el-option value="failed" :label="lc('admin_pay_failed', null, '失败')" />
          <el-option value="cancelled" :label="lc('admin_pay_cancelled', null, '已取消')" />
          <el-option value="refunded" :label="lc('admin_pay_refunded', null, '已退款')" />
        </el-select>
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          unlink-panels
          size="small"
          :shortcuts="dateShortcuts"
          :range-separator="lc('admin_company_00019', null, '至')"
          :start-placeholder="lc('admin_00343', null, '开始日期')"
          :end-placeholder="lc('admin_00344', null, '结束日期')"
        />
        <el-button size="small" type="primary" @click="search">{{ lc('admin_user_weipin_00049', null, '搜索') }}</el-button>
      </div>
    </div>
    <div class="moduleElTable">
      <el-table
        v-loading="loading"
        :data="rows"
        border
        style="width: 100%"
        row-key="id"
        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
        :empty-text="lc('wap_js_00113', null, '暂无数据')"
      >
        <el-table-column prop="ctime_n" :label="lc('admin_pay_ctime', null, '订单创建时间')" width="170" />
        <el-table-column prop="pay_no" :label="lc('admin_pay_pay_no', null, '订单编号')" min-width="160" show-overflow-tooltip />
        <el-table-column prop="merchant_order_no" :label="lc('admin_pay_merchant_order', null, '商户单号')" min-width="140" show-overflow-tooltip />
        <el-table-column prop="channel_ref" :label="lc('admin_pay_channel_ref', null, '三方ID')" min-width="160" show-overflow-tooltip />
        <el-table-column :label="lc('admin_pay_amount', null, '订单金额')" width="140">
          <template #default="{ row }">
            {{ row.amount_yuan }} {{ row.currency }}
          </template>
        </el-table-column>
        <el-table-column prop="method_code" :label="lc('admin_pay_method', null, '支付方式')" width="110" />
        <el-table-column :label="lc('admin_pay_status', null, '状态')" width="100">
          <template #default="{ row }">{{ statusText(row.status) }}</template>
        </el-table-column>
        <el-table-column :label="lc('admin_pay_merchant', null, '商户')" min-width="140">
          <template #default="{ row }">
            {{ row.merchant_name }} ({{ row.merchant_code }})
          </template>
        </el-table-column>
        <el-table-column :label="lc('member_user_00048', null, '操作')" width="220" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" size="small" @click="openDetail(row)">{{ lc('admin_pay_detail', null, '详情') }}</el-button>
            <el-button
              v-if="row.status === 'pending'"
              size="small"
              @click="closeRow(row)"
            >{{ lc('admin_pay_close', null, '关闭') }}</el-button>
            <el-button
              v-if="row.status === 'paid' && row.method_code === 'stripe'"
              size="small"
              type="danger"
              @click="refundRow(row)"
            >{{ lc('admin_pay_refund', null, '退款') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
    <div class="pay-footer">
      <el-pagination
        v-model:current-page="page"
        v-model:page-size="pageSize"
        :total="total"
        layout="total, prev, pager, next"
        @current-change="load"
      />
    </div>
    <el-drawer
      v-model="detailOpen"
      :title="lc('admin_pay_detail', null, '详情')"
      size="480px"
      append-to-body
    >
      <el-descriptions v-if="detail" :column="1" border>
        <el-descriptions-item :label="lc('admin_pay_pay_no', null, '订单编号')">{{ detail.pay_no }}</el-descriptions-item>
        <el-descriptions-item :label="lc('admin_pay_merchant_order', null, '商户单号')">{{ detail.merchant_order_no }}</el-descriptions-item>
        <el-descriptions-item :label="lc('admin_pay_channel_ref', null, '三方ID')">{{ detail.channel_ref || '—' }}</el-descriptions-item>
        <el-descriptions-item :label="lc('admin_pay_subject', null, '商品')">{{ detail.subject || '—' }}</el-descriptions-item>
        <el-descriptions-item :label="lc('admin_pay_pay_url', null, '支付链接')">
          <span style="word-break: break-all">{{ detail.pay_url || '—' }}</span>
        </el-descriptions-item>
        <el-descriptions-item :label="lc('admin_pay_paid_at', null, '支付时间')">{{ detail.paid_at_n || '—' }}</el-descriptions-item>
        <el-descriptions-item :label="lc('admin_pay_status', null, '状态')">{{ statusText(detail.status) }}</el-descriptions-item>
        <el-descriptions-item :label="lc('admin_pay_ctime', null, '订单创建时间')">{{ detail.ctime_n }}</el-descriptions-item>
      </el-descriptions>
    </el-drawer>
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
.pay-kw {
  width: 320px;
}
.pay-footer {
  flex-shrink: 0;
  padding: 8px 12px;
  text-align: right;
}
</style>
