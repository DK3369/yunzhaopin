<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-admins', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/rbac/users', {
    page: 1,
    page_size: 50,
  }),
)
</script>

<template>
  <div>
    <h1>{{ $t('ui.admins') }}</h1>
    <el-button size="small" @click="refresh">{{ $t('ui.refresh') }}</el-button>
    <el-table :data="data?.list || []" style="margin-top: 12px">
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="username" :label="$t('ui.username')" />
      <el-table-column prop="name" :label="$t('ui.name')" />
      <el-table-column prop="group_name" :label="$t('ui.category')" />
      <el-table-column prop="status" label="status" width="80" />
    </el-table>
  </div>
</template>
