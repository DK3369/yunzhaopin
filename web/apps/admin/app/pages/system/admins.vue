<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-admins', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/users', {
    page: 1,
    page_size: 50,
    usertype: 3,
  }),
)
</script>

<template>
  <div>
    <h1>管理员</h1>
    <p>
      RBAC 后置：当前仍仅校验 JWT <code>usertype=3</code>，不改对外字段。完整
      <code>role_user</code> / <code>role_ugroup</code> 待 PHP 权限表对接。
    </p>
    <el-button size="small" @click="refresh">刷新</el-button>
    <el-table :data="data?.list || []" style="margin-top: 12px">
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="username" label="用户名" />
      <el-table-column prop="usertype" label="usertype" width="100" />
      <el-table-column prop="status" label="status" width="80" />
    </el-table>
  </div>
</template>
