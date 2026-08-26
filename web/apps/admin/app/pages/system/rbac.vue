<script setup lang="ts">
const api = useApi()
const { data: groups } = await useAsyncData('rbac-groups', () =>
  api.post<Array<{ id: number; group_name: string }>>('/v1/admin/rbac/groups', {}),
)
const { data, refresh } = await useAsyncData('rbac-users', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/rbac/users', {
    page: 1,
    page_size: 20,
  }),
)
async function setStatus(row: { uid: number }, status: number) {
  await api.post('/v1/admin/rbac/users/status', { uid: row.uid, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>后台角色</h1>
    <p>
      读 PHP <code>phpyun_admin_user</code> / <code>phpyun_admin_user_group</code>。
      进后台仍靠 JWT <code>usertype=3</code>，不解析 <code>group_power</code>。
    </p>
    <h2>用户组</h2>
    <el-table :data="Array.isArray(groups) ? groups : []" size="small" style="max-width: 480px">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="group_name" label="group_name" />
    </el-table>
    <h2 style="margin-top: 20px">管理员账号</h2>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="uid" width="80" />
      <el-table-column prop="username" label="username" />
      <el-table-column prop="name" label="name" />
      <el-table-column prop="group_name" label="组" />
      <el-table-column prop="status" label="status" width="80" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">启用</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 0)">停用</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
