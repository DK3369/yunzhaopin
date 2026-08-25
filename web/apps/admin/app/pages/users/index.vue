<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-users', () =>
  api.post('/v1/admin/users', { page: 1, page_size: 20 }),
)
async function setStatus(row: { uid: number }, status: number) {
  await api.post('/v1/admin/users/status', { uid: row.uid, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>用户管理</h1>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="username" label="用户名" />
      <el-table-column prop="usertype" label="类型" width="90" />
      <el-table-column prop="status" label="状态" width="90" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">解冻</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 0)">冻结</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
