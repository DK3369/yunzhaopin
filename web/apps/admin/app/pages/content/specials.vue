<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-specials', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/specials', {
    page: 1,
    page_size: 20,
  }),
)
async function setDisplay(row: { id: number }, display: number) {
  await api.post('/v1/admin/specials/display', { id: row.id, display })
  refresh()
}
</script>

<template>
  <div>
    <h1>专题招聘</h1>
    <p>PHP 列 display</p>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" label="标题" />
      <el-table-column prop="status" label="display" width="90" />
      <el-table-column label="操作" width="180">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="setDisplay(row, 1)">展示</el-button>
          <el-button size="small" @click="setDisplay(row, 0)">隐藏</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
