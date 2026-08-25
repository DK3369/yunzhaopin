<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-reports', () =>
  api.post('/v1/admin/reports', { page: 1, page_size: 20 }),
)
async function setStatus(row: { id: number }, status: number) {
  await api.post('/v1/admin/reports/status', { id: row.id, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>举报</h1>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="target_kind_n" label="类型" />
      <el-table-column prop="reason_code" label="原因" />
      <el-table-column prop="status_n" label="状态" />
      <el-table-column label="操作">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">处理</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
