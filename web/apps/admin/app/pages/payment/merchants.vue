<script setup lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus'
import { lc } from '~/utils/phpLc'

type Row = {
  id: number
  code: string
  name: string
  api_key: string
  notify_url: string
  return_url: string
  allow_ips: string
  ip_count: number
  hmac_enabled: boolean
  gateway_base: string
  status: string
  ctime_n: string
}

const api = useApi()
const loading = ref(false)
const rows = ref<Row[]>([])
const editOpen = ref(false)
const guideOpen = ref(false)
const guideRow = ref<Row | null>(null)
const secretOnce = ref('')
const lastKeyId = ref('')
const form = reactive({
  id: 0,
  code: '',
  name: '',
  notify_url: '',
  return_url: '',
  allow_ips: '',
  status: 'active',
  rotate_secret: false,
})

async function load() {
  loading.value = true
  try {
    const data = await api.post<Row[]>('/v1/admin/pay/merchants/list', {})
    rows.value = Array.isArray(data) ? data : []
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('admin_user_weipin_00026', null, '加载失败'))
  } finally {
    loading.value = false
  }
}

function openAdd() {
  form.id = 0
  form.code = ''
  form.name = ''
  form.notify_url = ''
  form.return_url = ''
  form.allow_ips = ''
  form.status = 'active'
  form.rotate_secret = false
  secretOnce.value = ''
  lastKeyId.value = ''
  editOpen.value = true
}

function openEdit(row: Row) {
  form.id = row.id
  form.code = row.code
  form.name = row.name
  form.notify_url = row.notify_url
  form.return_url = row.return_url
  form.allow_ips = row.allow_ips || ''
  form.status = row.status
  form.rotate_secret = false
  secretOnce.value = ''
  lastKeyId.value = row.api_key
  editOpen.value = true
}

function openGuide(row: Row) {
  guideRow.value = row
  guideOpen.value = true
}

function guideText(row: Row) {
  const base = (row.gateway_base || '').replace(/\/$/, '')
  return [
    `${lc('admin_pay_base_url', null, '网关地址')}: ${base}`,
    `POST ${base}/v1/pay/methods/list`,
    `POST ${base}/v1/pay/orders`,
    `POST ${base}/v1/pay/orders/detail`,
    'Authorization: HMAC-SHA256 key_id=... ts=... sign=hex(hmac(secret, ts + "\\n" + POST + "\\n" + path + "\\n" + sha256hex(body)))',
    lc('admin_pay_hmac_clock', null, '时钟差不超过 300 秒'),
    lc('admin_pay_hmac_ip', null, '必须从白名单 IP 出站；名单为空则拒绝'),
    row.code === 'ov6' ? lc('admin_pay_ov6_internal', null, 'OV6 内部收银台走 JWT，不走 HMAC') : '',
  ]
    .filter(Boolean)
    .join('\n')
}

async function copyText(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    ElMessage.success(lc('admin_pay_copied', null, '已复制'))
  } catch {
    ElMessage.warning(text)
  }
}

async function save() {
  const code = form.code.trim().toLowerCase()
  if (!form.id && !/^[a-z][a-z0-9_]{0,63}$/.test(code)) {
    ElMessage.warning(lc('admin_seeker_vip_code', null, '代码须小写字母开头，只含字母数字下划线'))
    return
  }
  if (!form.name.trim()) {
    ElMessage.warning(lc('admin_country_name', null, '请填写名称'))
    return
  }
  if (code !== 'ov6' && !form.allow_ips.trim()) {
    ElMessage.warning(lc('admin_pay_allow_ips_req', null, '请填写 IP 白名单'))
    return
  }
  loading.value = true
  try {
    const data = await api.post<{ id: number; code: string; api_key: string; api_secret: string }>(
      '/v1/admin/pay/merchants/save',
      {
        id: form.id || undefined,
        code,
        name: form.name.trim(),
        notify_url: form.notify_url.trim(),
        return_url: form.return_url.trim(),
        allow_ips: form.allow_ips,
        status: form.status,
        rotate_secret: form.rotate_secret,
      },
    )
    if (data?.api_secret) {
      secretOnce.value = data.api_secret
      lastKeyId.value = data.api_key
      const copy = `key_id=${data.api_key}\napi_secret=${data.api_secret}\n${guideText({
        ...form,
        id: data.id,
        api_key: data.api_key,
        ip_count: form.allow_ips.split('\n').filter((x) => x.trim()).length,
        hmac_enabled: true,
        gateway_base: rows.value[0]?.gateway_base || window.location.origin,
        status: form.status,
        ctime_n: '',
        code: data.code,
        name: form.name,
        notify_url: form.notify_url,
        return_url: form.return_url,
        allow_ips: form.allow_ips,
      })}`
      await ElMessageBox.alert(copy, lc('admin_pay_secret_once', null, '请立即保存 Secret，只显示一次'), {
        type: 'warning',
        confirmButtonText: lc('admin_pay_copy', null, '复制'),
      })
      await copyText(copy)
    } else {
      ElMessage.success(lc('wap_00225', null, '已保存'))
      editOpen.value = false
    }
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
    await api.post('/v1/admin/pay/merchants/status', { id: row.id, status })
    ElMessage.success(lc('wap_00225', null, '已保存'))
    await load()
  } catch (e: unknown) {
    ElMessage.error(e instanceof Error ? e.message : lc('wap_00225', null, '保存失败'))
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div class="moduleElenAl pay-console">
    <div class="moduleSeachs">
      <div class="pay-bar">
        <span>{{ lc('admin_pay_merchants_title', null, '支付商户') }}</span>
        <span class="pay-hint">{{ lc('admin_pay_merchants_hint', null, '添加对接方、填 IP、发密钥。OV6 自用走 JWT。') }}</span>
        <el-button size="small" type="primary" @click="openAdd">{{ lc('admin_pay_add_partner', null, '添加对接方') }}</el-button>
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
        <el-table-column prop="code" :label="lc('admin_pay_merchant_code', null, '商户代码')" width="120" />
        <el-table-column prop="name" :label="lc('admin_seeker_vip_col_name', null, '名称')" min-width="140" />
        <el-table-column prop="api_key" label="key_id" min-width="170" show-overflow-tooltip />
        <el-table-column :label="lc('admin_pay_ip_count', null, 'IP')" width="70">
          <template #default="{ row }">{{ row.ip_count }}</template>
        </el-table-column>
        <el-table-column :label="lc('admin_pay_hmac', null, 'HMAC')" width="110">
          <template #default="{ row }">
            {{ row.hmac_enabled ? lc('admin_pay_hmac_on', null, 'HMAC已启用') : lc('admin_pay_hmac_off', null, 'HMAC未启用') }}
          </template>
        </el-table-column>
        <el-table-column :label="lc('admin_pay_status', null, '状态')" width="90">
          <template #default="{ row }">
            {{ row.status === 'active' ? lc('admin_pay_enable', null, '启用') : lc('admin_pay_pause', null, '暂停') }}
          </template>
        </el-table-column>
        <el-table-column
          fixed="right"
          :label="lc('member_user_00048', null, '操作')"
          width="280"
          align="right"
        >
          <template #default="{ row }">
            <el-button size="small" @click="openGuide(row)">{{ lc('admin_pay_guide', null, '对接说明') }}</el-button>
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
          </template>
        </el-table-column>
      </el-table>
    </div>
    <el-dialog v-model="editOpen" :title="lc('admin_pay_add_partner', null, '添加对接方')" width="560px">
      <el-form label-width="110px">
        <el-form-item :label="lc('admin_pay_merchant_code', null, '商户代码')">
          <el-input v-model="form.code" maxlength="64" :disabled="form.id > 0" placeholder="acme" />
        </el-form-item>
        <el-form-item :label="lc('admin_seeker_vip_col_name', null, '名称')">
          <el-input v-model="form.name" maxlength="128" />
        </el-form-item>
        <el-form-item :label="lc('admin_pay_allow_ips', null, 'IP白名单')">
          <el-input
            v-model="form.allow_ips"
            type="textarea"
            :rows="5"
            maxlength="2048"
            :placeholder="lc('admin_pay_allow_ips_ph', null, '一行一个 IPv4，可写 10.0.0.0/24')"
          />
        </el-form-item>
        <el-form-item label="notify_url">
          <el-input v-model="form.notify_url" maxlength="512" />
        </el-form-item>
        <el-form-item label="return_url">
          <el-input v-model="form.return_url" maxlength="512" />
        </el-form-item>
        <el-form-item :label="lc('admin_pay_status', null, '状态')">
          <el-select v-model="form.status" style="width: 100%">
            <el-option value="active" :label="lc('admin_pay_enable', null, '启用')" />
            <el-option value="paused" :label="lc('admin_pay_pause', null, '暂停')" />
          </el-select>
        </el-form-item>
        <el-form-item v-if="form.id > 0" :label="lc('admin_pay_hmac', null, 'HMAC')">
          <el-checkbox v-model="form.rotate_secret">{{ lc('admin_pay_rotate_secret', null, '轮换 Secret（只显示一次）') }}</el-checkbox>
        </el-form-item>
        <el-alert
          v-if="secretOnce"
          type="warning"
          show-icon
          :closable="false"
          :title="lc('admin_pay_secret_once', null, '请立即保存 Secret，只显示一次')"
        >
          <div>key_id={{ lastKeyId }}</div>
          <div>api_secret={{ secretOnce }}</div>
        </el-alert>
      </el-form>
      <template #footer>
        <el-button @click="editOpen = false">{{ lc('wap_js_00075', null, '取消') }}</el-button>
        <el-button type="primary" :loading="loading" @click="save">{{ lc('wap_00225', null, '确定') }}</el-button>
      </template>
    </el-dialog>
    <el-drawer
      v-model="guideOpen"
      :title="lc('admin_pay_guide', null, '对接说明')"
      size="520px"
      append-to-body
    >
      <pre v-if="guideRow" class="pay-guide">{{ guideText(guideRow) }}</pre>
      <el-button v-if="guideRow" type="primary" size="small" @click="copyText(guideText(guideRow))">
        {{ lc('admin_pay_copy', null, '复制') }}
      </el-button>
    </el-drawer>
  </div>
</template>

<style scoped>
.pay-console {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.pay-console :deep(.moduleSeachs) {
  height: auto !important;
  min-height: 50px;
  flex-shrink: 0;
  flex-wrap: wrap;
}
.pay-console :deep(.moduleElTable) {
  flex: 1;
  min-height: 280px;
  height: auto !important;
  overflow: auto;
}
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
.pay-guide {
  white-space: pre-wrap;
  word-break: break-all;
  font-size: 13px;
  line-height: 1.6;
  background: #f5f7fa;
  padding: 12px;
  border-radius: 4px;
}
</style>
