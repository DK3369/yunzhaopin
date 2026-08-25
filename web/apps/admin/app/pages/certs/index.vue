<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-certs', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/company-certs', { page: 1, page_size: 20 }),
)
async function review(row: { uid: number }, approve: boolean) {
  await api.post('/v1/admin/company-certs/review', { uid: row.uid, approve, note: '' })
  refresh()
}
</script>

<template>
  <div>
    <h1>企业认证</h1>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="status_n" label="状态" width="110" />
      <el-table-column prop="note" label="备注" />
      <el-table-column prop="submitted_at_n" label="提交时间" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="review(row, true)">通过</el-button>
          <el-button size="small" type="danger" @click="review(row, false)">拒绝</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
