<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-jobs', () =>
  api.post('/v1/admin/jobs', { page: 1, page_size: 20, state: 0 }),
)
async function review(row: { id: number }, state: number) {
  await api.post('/v1/admin/jobs/state', { id: row.id, state })
  refresh()
}
</script>

<template>
  <div>
    <h1>职位审核</h1>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="职位" />
      <el-table-column prop="com_name" label="企业" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="review(row, 1)">通过</el-button>
          <el-button size="small" type="danger" @click="review(row, 2)">拒绝</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
