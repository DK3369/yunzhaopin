<script setup lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus'
import { lc } from '~/utils/phpLc'

type Row = {
  id: number
  code: string
  name_zh: string
  name_en: string
  flag: string
  status: number
  sort: number
}

const api = useApi()
const loading = ref(false)
const rows = ref<Row[]>([])
const tableRef = ref<{
  clearSelection: () => void
  toggleRowSelection: (row: Row, selected?: boolean) => void
  getSelectionRows: () => Row[]
} | null>(null)
const addOpen = ref(false)
const addForm = reactive({ code: '', name_zh: '', name_en: '' })

async function load() {
  loading.value = true
  try {
    const data = await api.post<Row[]>('/v1/admin/countries/list', {})
    rows.value = Array.isArray(data) ? data : []
    await nextTick()
    restoreSelection()
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : lc('common_00001', null, '加载失败')
    ElMessage.error(msg)
  } finally {
    loading.value = false
  }
}

function restoreSelection() {
  const table = tableRef.value
  if (!table) return
  table.clearSelection()
  const picked = rows.value.filter((r) => Number(r.status) === 1)
  const show = picked.length ? picked : rows.value
  for (const r of show) table.toggleRowSelection(r, true)
}

async function save() {
  const selected = tableRef.value?.getSelectionRows?.() || []
  const ids = selected.map((r) => Number(r.id)).filter((id) => id > 0)
  if (!ids.length) {
    ElMessage.warning(lc('admin_country_pick_one', null, '请至少勾选一个国家'))
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/countries/select', { ids })
    ElMessage.success(lc('admin_country_saved', null, '已保存，前台只显示勾选的国家'))
    await load()
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : lc('wap_00225', null, '保存失败')
    ElMessage.error(msg)
  } finally {
    loading.value = false
  }
}

async function addCountry() {
  const code = addForm.code.trim().toUpperCase()
  const name_zh = addForm.name_zh.trim()
  if (code.length !== 2 || !/^[A-Z]{2}$/.test(code)) {
    ElMessage.warning(lc('admin_country_code', null, '请填写两位国家代码，例如 US'))
    return
  }
  if (!name_zh) {
    ElMessage.warning(lc('admin_country_name', null, '请填写中文名称'))
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/countries', {
      code,
      name_zh,
      name_en: addForm.name_en.trim() || undefined,
    })
    addOpen.value = false
    addForm.code = ''
    addForm.name_zh = ''
    addForm.name_en = ''
    ElMessage.success(lc('admin_00197', null, '已添加'))
    await load()
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : lc('common_00001', null, '添加失败')
    ElMessage.error(msg)
  } finally {
    loading.value = false
  }
}

async function removeRow(row: Row) {
  try {
    await ElMessageBox.confirm(
      lc('admin_vue_00137', [row.name_zh || row.code], `确定移除 ${row.name_zh || row.code}？`),
      lc('common_01520', null, '提示'),
      { type: 'warning' },
    )
  } catch {
    return
  }
  loading.value = true
  try {
    await api.post('/v1/admin/countries/delete', { id: row.id })
    ElMessage.success(lc('wap_js_00077', null, '已删除'))
    await load()
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : lc('common_00001', null, '删除失败')
    ElMessage.error(msg)
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
        {{ lc('admin_country_title', null, '前台国家') }}
        <span style="margin-left: 12px; color: #909399; font-weight: 400">
          {{
            lc(
              'admin_country_hint',
              null,
              '和国家分类、城市分类分开。没有省市区也可以勾选上架，前台只出现国家。',
            )
          }}
        </span>
      </div>
      <div class="nrtopbtn">
        <el-button size="small" type="primary" @click="save">{{ lc('wap_00225', null, '保存所选') }}</el-button>
        <el-button size="small" @click="addOpen = true">{{ lc('admin_00197', null, '添加国家') }}</el-button>
      </div>
    </div>
    <div class="moduleElTable">
      <el-table
        ref="tableRef"
        v-loading="loading"
        :data="rows"
        border
        height="100%"
        row-key="id"
        :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
        :empty-text="lc('common_00002', null, '暂无数据')"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="flag" :label="lc('admin_country_flag', null, '旗帜')" width="80">
          <template #default="{ row }">{{ row.flag }}</template>
        </el-table-column>
        <el-table-column prop="code" :label="lc('admin_system_00066', null, '代码')" width="90" />
        <el-table-column prop="name_zh" :label="lc('admin_system_00068', null, '中文名')" />
        <el-table-column prop="name_en" :label="lc('admin_system_00104', null, '英文名')" />
        <el-table-column :label="lc('member_com_00023', null, '前台显示')" width="110">
          <template #default="{ row }">
            {{ Number(row.status) === 1 ? lc('common_00003', null, '是') : lc('common_00004', null, '否') }}
          </template>
        </el-table-column>
        <el-table-column
          fixed="right"
          :label="lc('member_user_00048', null, '操作')"
          width="100"
          align="right"
        >
          <template #default="{ row }">
            <el-button type="danger" size="small" @click="removeRow(row)">{{
              lc('wap_js_00077', null, '删除')
            }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
    <el-dialog v-model="addOpen" :title="lc('admin_00197', null, '添加国家')" width="420px">
      <el-form label-width="90px">
        <el-form-item :label="lc('admin_system_00066', null, '代码')">
          <el-input v-model="addForm.code" maxlength="2" placeholder="US" />
        </el-form-item>
        <el-form-item :label="lc('admin_system_00068', null, '中文名')">
          <el-input v-model="addForm.name_zh" maxlength="120" />
        </el-form-item>
        <el-form-item :label="lc('admin_system_00104', null, '英文名')">
          <el-input v-model="addForm.name_en" maxlength="120" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addOpen = false">{{ lc('wap_js_00075', null, '取消') }}</el-button>
        <el-button type="primary" :loading="loading" @click="addCountry">{{
          lc('wap_00225', null, '确定')
        }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>
