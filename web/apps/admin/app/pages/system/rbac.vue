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
    <h1>{{ $t('ui.rbac') }}</h1>
    <p>{{ $t('ui.rbac_hint') }}</p>
    <h2>{{ $t('ui.users') }}</h2>
    <el-table :data="Array.isArray(groups) ? groups : []" size="small" style="max-width: 480px">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="group_name" label="group_name" />
    </el-table>
    <h2 style="margin-top: 20px">{{ $t('ui.admins') }}</h2>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="uid" width="80" />
      <el-table-column prop="username" label="username" />
      <el-table-column prop="name" label="name" />
      <el-table-column prop="group_name" :label="$t('ui.category')" />
      <el-table-column prop="status" label="status" width="80" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">{{ $t('ui.enable') }}</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 0)">{{ $t('ui.disable') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
