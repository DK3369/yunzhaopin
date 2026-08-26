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
    <h1>{{ $t('ui.admins') }}</h1>
    <p>{{ $t('ui.rbac_hint') }}</p>
    <el-button size="small" @click="refresh">{{ $t('ui.refresh') }}</el-button>
    <el-table :data="data?.list || []" style="margin-top: 12px">
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="username" :label="$t('ui.username')" />
      <el-table-column prop="usertype" label="usertype" width="100" />
      <el-table-column prop="status" label="status" width="80" />
    </el-table>
  </div>
</template>
