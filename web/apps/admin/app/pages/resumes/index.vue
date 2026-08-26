<script setup lang="ts">
const api = useApi()
const rStatus = ref<number | undefined>()
const keyword = ref('')
const { data, refresh } = await useAsyncData('admin-resumes', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/resumes', {
    page: 1,
    page_size: 20,
    r_status: rStatus.value,
    keyword: keyword.value || undefined,
  }),
)
async function setStatus(row: { uid: number }, r_status: number) {
  await api.post('/v1/admin/resumes/status', { uid: row.uid, r_status })
  refresh()
}
async function exportCsv() {
  const r = await api.post<{ filename: string; csv: string }>('/v1/admin/resumes/export', {
    r_status: rStatus.value,
    keyword: keyword.value || undefined,
  })
  const blob = new Blob(['\ufeff' + (r.csv || '')], { type: 'text/csv;charset=utf-8' })
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  a.download = r.filename || 'resumes.csv'
  a.click()
}
</script>

<template>
  <div>
    <h1>简历审核</h1>
    <p>PHP <code>users_resume</code>：列 <code>r_status</code>。不是工作/教育经历逐条编辑树。</p>
    <el-form inline>
      <el-form-item>
        <el-input v-model="keyword" placeholder="姓名" clearable />
      </el-form-item>
      <el-form-item>
        <el-select v-model="rStatus" placeholder="r_status" clearable style="width: 140px">
          <el-option :value="0" label="待审" />
          <el-option :value="1" label="通过" />
          <el-option :value="2" label="未通过" />
        </el-select>
      </el-form-item>
      <el-button type="primary" @click="refresh">查询</el-button>
      <el-button @click="exportCsv">导出 CSV</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="name" label="姓名" />
      <el-table-column prop="r_status" label="r_status" width="90" />
      <el-table-column prop="status" label="公开 status" width="110" />
      <el-table-column prop="lastupdate" label="lastupdate" width="120" />
      <el-table-column label="操作" width="220">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">通过</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 2)">拒绝</el-button>
          <el-button size="small" @click="navigateTo(`/resumes/tree?uid=${row.uid}`)">经历树</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
