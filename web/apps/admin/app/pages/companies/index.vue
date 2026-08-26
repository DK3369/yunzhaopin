<script setup lang="ts">
const api = useApi()
const rStatus = ref<number | undefined>()
const keyword = ref('')
const { data, refresh } = await useAsyncData('admin-companies', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/companies', {
    page: 1,
    page_size: 20,
    r_status: rStatus.value,
    keyword: keyword.value || undefined,
  }),
)
async function setStatus(row: { uid: number }, r_status: number) {
  await api.post('/v1/admin/companies/status', { uid: row.uid, r_status })
  refresh()
}
async function exportCsv() {
  const r = await api.post<{ filename: string; csv: string }>('/v1/admin/companies/export', {
    r_status: rStatus.value,
    keyword: keyword.value || undefined,
  })
  const blob = new Blob(['\ufeff' + (r.csv || '')], { type: 'text/csv;charset=utf-8' })
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  a.download = r.filename || 'companies.csv'
  a.click()
}
</script>

<template>
  <div>
    <h1>企业档案</h1>
    <p>PHP <code>user/company</code>：<code>r_status</code> 0 待审 / 1 通过 / 2 锁定。导出为 CSV（Excel 可开）。</p>
    <el-form inline>
      <el-form-item>
        <el-input v-model="keyword" placeholder="企业名" clearable />
      </el-form-item>
      <el-form-item>
        <el-select v-model="rStatus" placeholder="r_status" clearable style="width: 140px">
          <el-option :value="0" label="待审" />
          <el-option :value="1" label="通过" />
          <el-option :value="2" label="锁定" />
        </el-select>
      </el-form-item>
      <el-button type="primary" @click="refresh">查询</el-button>
      <el-button @click="exportCsv">导出 CSV</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="name" label="名称" />
      <el-table-column prop="r_status" label="r_status" width="90" />
      <el-table-column prop="cityid" label="cityid" width="90" />
      <el-table-column label="操作" width="220">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">通过</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 2)">锁定</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
