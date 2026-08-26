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
  </div>
</template>
