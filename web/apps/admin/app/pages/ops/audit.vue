<script setup lang="ts">
const api = useApi()
const actionPrefix = ref('')
const { data, refresh } = await useAsyncData('admin-audit', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/audit-log', {
    page: 1,
    page_size: 20,
    action_prefix: actionPrefix.value || undefined,
  }),
)
</script>

<template>
  <div>
    <h1>审计日志</h1>
    <el-form inline>
      <el-form-item><el-input v-model="actionPrefix" placeholder="action 前缀，如 admin." /></el-form-item>
      <el-button @click="refresh()">查询</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="actor_uid" label="操作人" width="90" />
      <el-table-column prop="action" label="动作" />
      <el-table-column prop="target" label="对象" />
      <el-table-column prop="success" label="成功" width="80" />
      <el-table-column prop="created_at_n" label="时间" />
    </el-table>
  </div>
</template>
