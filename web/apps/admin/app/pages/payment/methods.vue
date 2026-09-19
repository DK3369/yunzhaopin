<script setup lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus'
import { lc } from '~/utils/phpLc'

type Channel = { code: string; name: string; live: boolean }
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
const channels = ref<Channel[]>([])
const merchantId = ref(0)
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

async function loadChannels() {
  const data = await api.post<Channel[]>('/v1/admin/pay/channels/list', {})
  channels.value = Array.isArray(data) ? data : []
}

async function load() {
  loading.value = true
  try {
    await loadMerchants()
    await loadChannels()
    const data = await api.post<Row[]>('/v1/admin/pay/methods/list', {
      merchant_id: merchantId.value || undefined,
    })
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

function yn(v: boolean) {
  return v ? lc('common_02085', null, '是') : lc('common_02063', null, '否')
}

function openAdd() {
  const first = merchants.value[0]
  const ch = channels.value[0]
  form.id = 0
  form.merchant_id = first?.id || 0
  form.code = ch?.code || 'stripe'
  form.name = ch?.name || 'Stripe'
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
    const ch = channels.value.find((c) => c.code === code)
    if (ch) form.name = ch.name
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
  <div class="pay-page">
    <div class="moduleSeachs">
      <div class="pay-bar">
        <span>{{ lc('admin_pay_methods_title', null, '支付方式') }}</span>
        <span class="pay-hint">{{ lc('admin_pay_methods_hint', null, 'Stripe 密钥只在本页编辑。PayPal / GrabPay / GCash / PayMaya 目录占位，不真收。') }}</span>
        <el-select
          v-model="merchantId"
          size="small"
          style="width: 180px"
          :placeholder="lc('admin_pay_merchant', null, '商户')"
          @change="load"
        >
          <el-option :label="lc('admin_pay_all', null, '全部')" :value="0" />
          <el-option v-for="m in merchants" :key="m.id" :label="`${m.name} (${m.code})`" :value="m.id" />
        </el-select>
        <el-button size="small" type="primary" @click="openAdd">{{ lc('admin_00197', null, '添加') }}</el-button>
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
        <el-table-column prop="id" label="ID" width="70" />
        <el-table-column :label="lc('admin_pay_merchant', null, '商户')" width="140">
          <template #default="{ row }">{{ merchantName(row.merchant_id) }}</template>
        </el-table-column>
        <el-table-column prop="code" :label="lc('admin_pay_method', null, '支付方式')" width="110" />
        <el-table-column prop="name" :label="lc('admin_seeker_vip_col_name', null, '名称')" min-width="120" />
        <el-table-column :label="lc('admin_pay_status', null, '状态')" width="90">
          <template #default="{ row }">
            {{ row.status === 'active' ? lc('admin_pay_enable', null, '启用') : lc('admin_pay_pause', null, '暂停') }}
          </template>
        </el-table-column>
        <el-table-column :label="lc('admin_pay_secret_set', null, '密钥')" width="80">
          <template #default="{ row }">{{ yn(row.secret_key_set) }}</template>
        </el-table-column>
        <el-table-column :label="lc('admin_pay_webhook_set', null, 'Webhook')" width="90">
          <template #default="{ row }">{{ yn(row.webhook_secret_set) }}</template>
        </el-table-column>
        <el-table-column prop="currency" :label="lc('admin_payset_stripe_currency', null, 'Currency')" width="90" />
        <el-table-column :label="lc('admin_pay_charge_ready', null, '可收款')" width="90">
          <template #default="{ row }">{{ yn(row.charge_ready) }}</template>
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
            <el-option v-for="c in channels" :key="c.code" :label="c.name" :value="c.code" />
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
        <el-form-item v-if="form.code !== 'stripe'">
          <span class="pay-hint">{{ lc('admin_pay_placeholder', null, '此渠道仅目录占位，create 返回 not_configured。') }}</span>
        </el-form-item>
        <el-form-item v-if="form.code === 'stripe'" :label="lc('admin_payset_stripe_sk', null, 'Secret key')">
          <el-input v-model="form.secret_key" type="password" show-password maxlength="256" :placeholder="lc('admin_pay_secret_keep', null, '留空则不改')" />
        </el-form-item>
        <el-form-item v-if="form.code === 'stripe'" :label="lc('admin_pay_webhook_secret', null, 'Webhook secret')">
          <el-input v-model="form.webhook_secret" type="password" show-password maxlength="256" :placeholder="lc('admin_pay_secret_keep', null, '留空则不改')" />
        </el-form-item>
        <el-form-item v-if="form.code === 'stripe'" :label="lc('admin_payset_stripe_currency', null, 'Currency')">
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
</style>
