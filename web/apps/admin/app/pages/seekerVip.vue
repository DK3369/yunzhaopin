<script setup lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus'
import { lc } from '~/utils/phpLc'

type Row = {
  id: number
  code: string
  name: string
  months: number
  price_cents: number
  price_yuan: number
  chat: number
  resume_top: number
  tpl_all: number
  refresh_free: number
  sort: number
  display: number
}

const api = useApi()
const loading = ref(false)
const rows = ref<Row[]>([])
const editOpen = ref(false)
const form = reactive({
  id: 0,
  code: '',
  name: '',
  months: 1,
  price_yuan: 29,
  sort: 0,
  display: 1,
})

async function load() {
  loading.value = true
  try {
    const data = await api.post<Row[]>('/v1/admin/seeker/vip/packages/list', {})
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
  form.months = 1
  form.price_yuan = 29
  form.sort = 0
  form.display = 1
  editOpen.value = true
}

function openEdit(row: Row) {
  form.id = row.id
  form.code = row.code
  form.name = row.name
  form.months = row.months
  form.price_yuan = row.price_yuan
  form.sort = row.sort
  form.display = row.display
  editOpen.value = true
}

async function save() {
  const code = form.code.trim().toLowerCase()
  if (!/^[a-z][a-z0-9_]{0,63}$/.test(code)) {
    ElMessage.warning(lc('admin_seeker_vip_code', null, '代码须小写字母开头，只含字母数字下划线'))
    return
  }
  if (!form.name.trim()) {
    ElMessage.warning(lc('admin_country_name', null, '请填写名称'))
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/seeker/vip/packages', {
      id: form.id || undefined,
      code,
      name: form.name.trim(),
      months: Number(form.months),
      price_cents: Math.round(Number(form.price_yuan) * 100),
      chat: 1,
      resume_top: 1,
      tpl_all: 1,
      refresh_free: 1,
      sort: Number(form.sort) || 0,
      display: Number(form.display) === 1 ? 1 : 0,
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
    await api.post('/v1/admin/seeker/vip/packages/delete', { id: row.id })
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
        {{ lc('admin_seeker_vip_title', null, '求职包月会员') }}
        <span style="margin-left: 12px; color: #909399; font-weight: 400">
          {{ lc('admin_seeker_vip_hint', null, '按时长卖。权益默认全开：私聊、置顶、模板、刷新不限流。招聘前台卖 VIP 1–6 套餐，不是时间会员。') }}
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
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="code" :label="lc('admin_seeker_vip_col_code', null, '代码')" width="140" />
        <el-table-column prop="name" :label="lc('admin_seeker_vip_col_name', null, '名称')" />
        <el-table-column prop="months" :label="lc('admin_seeker_vip_col_months', null, '月')" width="80" />
        <el-table-column prop="price_yuan" :label="lc('admin_seeker_vip_col_price', null, '价格')" width="100" />
        <el-table-column :label="lc('member_com_00023', null, '前台显示')" width="110">
          <template #default="{ row }">
            {{ Number(row.display) === 1 ? lc('common_02085', null, '是') : lc('common_02063', null, '否') }}
          </template>
        </el-table-column>
        <el-table-column
          fixed="right"
          :label="lc('member_user_00048', null, '操作')"
          width="160"
          align="right"
        >
          <template #default="{ row }">
            <el-button type="primary" size="small" @click="openEdit(row)">{{ lc('admin_seeker_vip_edit', null, '编辑') }}</el-button>
            <el-button type="danger" size="small" @click="removeRow(row)">{{ lc('admin_seeker_vip_delete', null, '删除') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
    <el-dialog v-model="editOpen" :title="lc('admin_seeker_vip_title', null, '求职包月会员')" width="460px">
      <el-form label-width="90px">
        <el-form-item :label="lc('admin_seeker_vip_col_code', null, '代码')">
          <el-input v-model="form.code" maxlength="64" :disabled="form.id > 0" placeholder="month_1" />
        </el-form-item>
        <el-form-item :label="lc('admin_seeker_vip_col_name', null, '名称')">
          <el-input v-model="form.name" maxlength="64" />
        </el-form-item>
        <el-form-item :label="lc('admin_seeker_vip_col_months', null, '月')">
          <el-input-number v-model="form.months" :min="1" :max="36" />
        </el-form-item>
        <el-form-item :label="lc('admin_seeker_vip_col_price', null, '价格')">
          <el-input-number v-model="form.price_yuan" :min="0.01" :precision="2" />
        </el-form-item>
        <el-form-item :label="lc('admin_system_00067', null, '排序')">
          <el-input-number v-model="form.sort" :min="0" />
        </el-form-item>
        <el-form-item :label="lc('member_com_00023', null, '前台显示')">
          <el-switch v-model="form.display" :active-value="1" :inactive-value="0" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editOpen = false">{{ lc('wap_js_00075', null, '取消') }}</el-button>
        <el-button type="primary" :loading="loading" @click="save">{{ lc('wap_00225', null, '确定') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>
