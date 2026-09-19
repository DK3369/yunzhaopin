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
  status: string
  ctime_n: string
}

const api = useApi()
const loading = ref(false)
const rows = ref<Row[]>([])
const editOpen = ref(false)
const secretOnce = ref('')
const form = reactive({
  id: 0,
  code: '',
  name: '',
  notify_url: '',
  return_url: '',
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
  form.status = 'active'
  form.rotate_secret = false
  secretOnce.value = ''
  editOpen.value = true
}

function openEdit(row: Row) {
  form.id = row.id
  form.code = row.code
  form.name = row.name
  form.notify_url = row.notify_url
  form.return_url = row.return_url
  form.status = row.status
  form.rotate_secret = false
  secretOnce.value = ''
  editOpen.value = true
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
        status: form.status,
        rotate_secret: form.rotate_secret,
      },
    )
    if (data?.api_secret) {
      secretOnce.value = data.api_secret
      await ElMessageBox.alert(
        `${lc('admin_pay_secret_once', null, '请立即保存 Secret，只显示一次')}：\nkey_id=${data.api_key}\nsecret=${data.api_secret}`,
        lc('admin_pay_hmac', null, 'HMAC'),
        { type: 'warning' },
      )
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
  <div class="moduleElenAl">
    <div class="moduleSeachs">
      <div>
        {{ lc('admin_pay_merchants_title', null, '支付商户') }}
        <span style="margin-left: 12px; color: #909399; font-weight: 400">
          {{ lc('admin_pay_merchants_hint', null, 'OV6 自用。外部商户用 HMAC 调 /v1/pay/orders。Secret 只在创建或轮换时显示一次。') }}
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
        <el-table-column prop="code" :label="lc('admin_pay_merchant_code', null, '商户代码')" width="120" />
        <el-table-column prop="name" :label="lc('admin_seeker_vip_col_name', null, '名称')" />
        <el-table-column prop="api_key" label="key_id" width="180" />
        <el-table-column prop="status" :label="lc('admin_pay_status', null, '状态')" width="100" />
        <el-table-column prop="ctime_n" :label="lc('admin_pay_ctime', null, '订单创建时间')" width="180" />
        <el-table-column
          fixed="right"
          :label="lc('member_user_00048', null, '操作')"
          width="220"
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
          </template>
        </el-table-column>
      </el-table>
    </div>
    <el-dialog v-model="editOpen" :title="lc('admin_pay_merchants_title', null, '支付商户')" width="520px">
      <el-form label-width="110px">
        <el-form-item :label="lc('admin_pay_merchant_code', null, '商户代码')">
          <el-input v-model="form.code" maxlength="64" :disabled="form.id > 0" placeholder="acme" />
        </el-form-item>
        <el-form-item :label="lc('admin_seeker_vip_col_name', null, '名称')">
          <el-input v-model="form.name" maxlength="128" />
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
      </el-form>
      <template #footer>
        <el-button @click="editOpen = false">{{ lc('wap_js_00075', null, '取消') }}</el-button>
        <el-button type="primary" :loading="loading" @click="save">{{ lc('wap_00225', null, '确定') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>
