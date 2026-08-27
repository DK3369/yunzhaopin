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
const { data: phpLogs } = await useAsyncData('admin-php-logs', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/admin-logs', {
    page: 1,
    page_size: 20,
  }),
)
</script>

<template>
  <div>
    <h1>{{ $t('ui.audit') }}</h1>
    <el-form inline>
      <el-form-item><el-input v-model="actionPrefix" :placeholder="$t('ui.action_prefix')" /></el-form-item>
      <el-button @click="refresh()">{{ $t('ui.query') }}</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="actor_uid" :label="$t('ui.actor')" width="90" />
      <el-table-column prop="action" :label="$t('ui.action')" />
      <el-table-column prop="target" :label="$t('ui.target')" />
      <el-table-column prop="success" :label="$t('ui.success')" width="80" />
      <el-table-column prop="created_at_n" :label="$t('ui.time')" />
    </el-table>
    <h2 style="margin-top: 24px">{{ $t('ui.admin_log') }}</h2>
    <el-table :data="phpLogs?.list || []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="username" :label="$t('ui.username')" />
      <el-table-column prop="content" :label="$t('ui.content')" />
      <el-table-column prop="ctime" :label="$t('ui.time')" width="120" />
    </el-table>
  </div>
</template>
