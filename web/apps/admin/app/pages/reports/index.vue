<script setup lang="ts">
const api = useApi()
const selected = ref<number[]>([])
const { data, refresh } = await useAsyncData('admin-reports', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/reports', { page: 1, page_size: 20 }),
)
function onSelect(rows: Array<{ id: number }>) {
  selected.value = rows.map((r) => r.id)
}
async function setStatus(row: { id: number }, status: number) {
  await api.post('/v1/admin/reports/status', { id: row.id, status })
  refresh()
}
async function batch(status: number) {
  if (!selected.value.length) return
  await api.post('/v1/admin/reports/batch/status', { ids: selected.value, status })
  selected.value = []
  refresh()
}
</script>

<template>
  <div>
    <h1>举报</h1>
    <div style="margin-bottom: 12px">
      <el-button size="small" @click="batch(1)">批量处理</el-button>
      <el-button size="small" type="danger" @click="batch(2)">批量驳回</el-button>
    </div>
    <el-table :data="data?.list || []" @selection-change="onSelect">
      <el-table-column type="selection" width="48" />
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="target_kind_n" label="类型" />
      <el-table-column prop="reason_code" label="原因" />
      <el-table-column prop="status_n" label="状态" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">处理</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 2)">驳回</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
