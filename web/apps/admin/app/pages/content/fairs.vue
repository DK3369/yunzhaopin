<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-fairs', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/fairs', {
    page: 1,
    page_size: 20,
  }),
)
async function setOpen(row: { id: number }, is_open: number) {
  await api.post('/v1/admin/fairs/open', { id: row.id, is_open })
  refresh()
}
</script>

<template>
  <div>
    <h1>招聘会</h1>
    <p>对齐 PHP is_open（上架开关）</p>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" label="标题" />
      <el-table-column prop="is_open" label="is_open" width="90" />
      <el-table-column label="操作" width="180">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="setOpen(row, 1)">开放</el-button>
          <el-button size="small" @click="setOpen(row, 0)">关闭</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
