<script setup lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus'
import { lc } from '~/utils/phpLc'

type Row = {
  id: number
  pay_no: string
  merchant_code: string
  event: string
  url: string
  http_status: number
  ok: number
  error: string
  ctime_n: string
}

type Paged = { list: Row[]; total: number; page: number; page_size: number }

const api = useApi()
const route = useRoute()
const loading = ref(false)
const rows = ref<Row[]>([])
const total = ref(0)
const page = ref(1)
const pageSize = ref(20)
const payNo = ref('')
const filterOk = ref('all')
const dateRange = ref<[Date, Date] | null>(null)

function allOr(v: string) {
  const s = (v || '').trim()
  return !s || s === 'all' ? '' : s
}

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

async function load() {
  loading.value = true
  try {
    const range = unixRange()
    const data = await api.post<Paged>('/v1/admin/pay/notifies/list', {
      pay_no: payNo.value.trim(),
      ok: allOr(filterOk.value),
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

async function retryRow(row: Row) {
  try {
    await ElMessageBox.confirm(
      lc('admin_pay_retry_confirm', [row.pay_no], `重试回调 ${row.pay_no}？`),
      lc('common_01520', null, '提示'),
      { type: 'warning' },
    )
  } catch {
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/pay/notifies/retry', { id: row.id })
    ElMessage.success(lc('wap_00225', null, '已保存'))
    await load()
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('wap_00225', null, '保存失败'))
  } finally {
    loading.value = false
  }
}

function resultText(ok: number) {
  return ok === 1
    ? lc('admin_pay_notify_ok', null, '成功')
    : lc('admin_pay_notify_fail', null, '失败')
}

onMounted(() => {
  const q = String(route.query.ok || '')
  if (q === '0' || q === '1') filterOk.value = q
  load()
})
</script>

<template>
  <div class="pay-page">
    <div class="moduleSeachs">
      <div class="pay-bar">
        <span>{{ lc('admin_pay_notifies_title', null, '商户回调') }}</span>
        <el-input
          v-model="payNo"
          size="small"
          clearable
          class="pay-kw"
          :placeholder="lc('admin_pay_pay_no', null, '订单编号')"
          @keyup.enter="search"
        />
        <el-select
          v-model="filterOk"
          size="small"
          style="width: 120px"
          :placeholder="lc('admin_pay_status', null, '状态')"
          @change="search"
        >
          <el-option :label="lc('admin_pay_all', null, '全部')" value="all" />
          <el-option value="1" :label="lc('admin_pay_notify_ok', null, '成功')" />
          <el-option value="0" :label="lc('admin_pay_notify_fail', null, '失败')" />
        </el-select>
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          unlink-panels
          size="small"
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
        <el-table-column prop="merchant_code" :label="lc('admin_pay_merchant', null, '商户')" width="120" />
        <el-table-column prop="event" :label="lc('admin_pay_event', null, '事件')" width="100" />
        <el-table-column prop="url" :label="lc('admin_pay_notify_url', null, '回调地址')" min-width="200" show-overflow-tooltip />
        <el-table-column :label="lc('admin_pay_status', null, '状态')" width="90">
          <template #default="{ row }">{{ resultText(row.ok) }}</template>
        </el-table-column>
        <el-table-column prop="http_status" :label="lc('admin_pay_http_status', null, 'HTTP')" width="80" />
        <el-table-column prop="error" :label="lc('admin_pay_error', null, '错误')" min-width="140" show-overflow-tooltip />
        <el-table-column :label="lc('member_user_00048', null, '操作')" width="90" fixed="right">
          <template #default="{ row }">
            <el-button
              v-if="row.ok !== 1"
              type="primary"
              size="small"
              @click="retryRow(row)"
            >{{ lc('admin_pay_retry', null, '重试') }}</el-button>
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
  width: 240px;
}
.pay-footer {
  flex-shrink: 0;
  padding: 8px 12px;
  text-align: right;
}
</style>
