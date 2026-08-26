<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-redeem-orders', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/redeem-orders', { page: 1, page_size: 20 }),
)
async function approve(row: { id: number }) {
  await api.post('/v1/admin/redeem-orders/approve', { id: row.id })
  refresh()
}
async function reject(row: { id: number }) {
  await api.post('/v1/admin/redeem-orders/reject', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.redeem_orders') }}</h1>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="status" :label="$t('ui.status')" width="90" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="approve(row)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" type="danger" @click="reject(row)">{{ $t('ui.reject') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
