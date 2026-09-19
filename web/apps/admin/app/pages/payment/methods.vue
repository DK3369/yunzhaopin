<script setup lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus'
import { lc } from '~/utils/phpLc'

type Merchant = { id: number; code: string; name: string }
type Row = {
  id: number
  merchant_id: number
  code: string
  name: string
  status: string
  sort: number
  charge_ready: boolean
  secret_key_set: boolean
  webhook_secret_set: boolean
  currency: string
}

const api = useApi()
const loading = ref(false)
const rows = ref<Row[]>([])
const merchants = ref<Merchant[]>([])
const editOpen = ref(false)
const form = reactive({
  id: 0,
  merchant_id: 0,
  code: 'stripe',
  name: 'Stripe',
  status: 'paused',
  sort: 0,
  secret_key: '',
  webhook_secret: '',
  currency: '',
})

async function loadMerchants() {
  const data = await api.post<Merchant[]>('/v1/admin/pay/merchants/list', {})
  merchants.value = Array.isArray(data) ? data : []
}

async function load() {
  loading.value = true
  try {
    await loadMerchants()
    const data = await api.post<Row[]>('/v1/admin/pay/methods/list', {})
    rows.value = Array.isArray(data) ? data : []
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('admin_user_weipin_00026', null, '加载失败'))
  } finally {
    loading.value = false
  }
}

function merchantName(id: number) {
  return merchants.value.find((m) => m.id === id)?.name || String(id)
}

function openAdd() {
  const first = merchants.value[0]
  form.id = 0
  form.merchant_id = first?.id || 0
  form.code = 'stripe'
  form.name = 'Stripe'
  form.status = 'paused'
  form.sort = 0
  form.secret_key = ''
  form.webhook_secret = ''
  form.currency = ''
  editOpen.value = true
}

function openEdit(row: Row) {
  form.id = row.id
  form.merchant_id = row.merchant_id
  form.code = row.code
  form.name = row.name
  form.status = row.status
  form.sort = row.sort
  form.secret_key = ''
  form.webhook_secret = ''
  form.currency = row.currency
  editOpen.value = true
}

watch(
  () => form.code,
  (code) => {
    if (form.id > 0) return
    if (code === 'stripe') form.name = 'Stripe'
    if (code === 'gcash') form.name = 'GCash'
    if (code === 'paymaya') form.name = 'PayMaya'
  },
)

async function save() {
  if (!form.merchant_id) {
    ElMessage.warning(lc('admin_pay_pick_merchant', null, '请选择商户'))
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/pay/methods/save', {
      id: form.id || undefined,
      merchant_id: form.merchant_id,
      code: form.code,
      name: form.name.trim(),
      status: form.status,
      sort: Number(form.sort) || 0,
      secret_key: form.secret_key.trim(),
      webhook_secret: form.webhook_secret.trim(),
      currency: form.currency.trim(),
    })
    editOpen.value = false
    ElMessage.success(lc('wap_00225', null, '已保存'))
    await load()
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('wap_00225', null, '保存失败'))
  } finally {
    loading.value = false
  }
}

async function setStatus(row: Row, status: string) {
  loading.value = true
  try {
    await api.post('/v1/admin/pay/methods/status', { id: row.id, status })
    ElMessage.success(lc('wap_00225', null, '已保存'))
    await load()
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('wap_00225', null, '保存失败'))
  } finally {
    loading.value = false
  }
}

async function removeRow(row: Row) {
  try {
    await ElMessageBox.confirm(
      lc('admin_vue_00137', [row.name], `确定删除 ${row.name}？`),
      lc('common_01520', null, '提示'),
      { type: 'warning' },
    )
  } catch {
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/pay/methods/delete', { id: row.id })
    ElMessage.success(lc('wap_js_00077', null, '已删除'))
    await load()
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('admin_user_00186', null, '删除失败'))
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div class="moduleElenAl">
    <div class="moduleSeachs">
      <div>
        {{ lc('admin_pay_methods_title', null, '支付方式') }}
        <span style="margin-left: 12px; color: #909399; font-weight: 400">
          {{ lc('admin_pay_methods_hint', null, 'Stripe 可真收。GCash / PayMaya 本轮仅配置，点支付提示未接入。密钥只保存在服务器。') }}
        </span>
      </div>
      <div class="nrtopbtn">
        <el-button size="small" type="primary" @click="openAdd">{{ lc('admin_00197', null, '添加') }}</el-button>
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
        <el-table-column prop="id" label="ID" width="70" />
        <el-table-column :label="lc('admin_pay_merchant', null, '商户')" width="140">
          <template #default="{ row }">{{ merchantName(row.merchant_id) }}</template>
        </el-table-column>
        <el-table-column prop="code" :label="lc('admin_pay_method', null, '支付方式')" width="120" />
        <el-table-column prop="name" :label="lc('admin_seeker_vip_col_name', null, '名称')" />
        <el-table-column prop="status" :label="lc('admin_pay_status', null, '状态')" width="100" />
        <el-table-column :label="lc('admin_pay_charge_ready', null, '可收款')" width="100">
          <template #default="{ row }">
            {{ row.charge_ready ? lc('common_02085', null, '是') : lc('common_02063', null, '否') }}
          </template>
        </el-table-column>
        <el-table-column
          fixed="right"
          :label="lc('member_user_00048', null, '操作')"
          width="260"
          align="right"
        >
          <template #default="{ row }">
            <el-button type="primary" size="small" @click="openEdit(row)">{{ lc('admin_seeker_vip_edit', null, '编辑') }}</el-button>
            <el-button
              v-if="row.status === 'active'"
              size="small"
              @click="setStatus(row, 'paused')"
            >{{ lc('admin_pay_pause', null, '暂停') }}</el-button>
            <el-button
              v-else
              size="small"
              type="success"
              @click="setStatus(row, 'active')"
            >{{ lc('admin_pay_enable', null, '启用') }}</el-button>
            <el-button type="danger" size="small" @click="removeRow(row)">{{ lc('admin_seeker_vip_delete', null, '删除') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
    <el-dialog v-model="editOpen" :title="lc('admin_pay_methods_title', null, '支付方式')" width="520px">
      <el-form label-width="110px">
        <el-form-item :label="lc('admin_pay_merchant', null, '商户')">
          <el-select v-model="form.merchant_id" :disabled="form.id > 0" style="width: 100%">
            <el-option v-for="m in merchants" :key="m.id" :label="`${m.name} (${m.code})`" :value="m.id" />
          </el-select>
        </el-form-item>
        <el-form-item :label="lc('admin_pay_method', null, '支付方式')">
          <el-select v-model="form.code" :disabled="form.id > 0" style="width: 100%">
            <el-option value="stripe" label="Stripe" />
            <el-option value="gcash" label="GCash" />
            <el-option value="paymaya" label="PayMaya" />
          </el-select>
        </el-form-item>
        <el-form-item :label="lc('admin_seeker_vip_col_name', null, '名称')">
          <el-input v-model="form.name" maxlength="128" />
        </el-form-item>
        <el-form-item :label="lc('admin_pay_status', null, '状态')">
          <el-select v-model="form.status" style="width: 100%">
            <el-option value="active" :label="lc('admin_pay_enable', null, '启用')" />
            <el-option value="paused" :label="lc('admin_pay_pause', null, '暂停')" />
          </el-select>
        </el-form-item>
        <el-form-item :label="lc('admin_system_00067', null, '排序')">
          <el-input-number v-model="form.sort" :min="0" />
        </el-form-item>
        <el-form-item :label="lc('admin_payset_stripe_sk', null, 'Secret key')">
          <el-input v-model="form.secret_key" type="password" show-password maxlength="256" :placeholder="lc('admin_pay_secret_keep', null, '留空则不改')" />
        </el-form-item>
        <el-form-item :label="lc('admin_pay_webhook_secret', null, 'Webhook secret')">
          <el-input v-model="form.webhook_secret" type="password" show-password maxlength="256" :placeholder="lc('admin_pay_secret_keep', null, '留空则不改')" />
        </el-form-item>
        <el-form-item :label="lc('admin_payset_stripe_currency', null, 'Currency')">
          <el-input v-model="form.currency" maxlength="16" placeholder="usd" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editOpen = false">{{ lc('wap_js_00075', null, '取消') }}</el-button>
        <el-button type="primary" :loading="loading" @click="save">{{ lc('wap_00225', null, '确定') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>
