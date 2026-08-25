<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-orders', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/orders', { page: 1, page_size: 20 }),
)
async function setStatus(row: { order_no: string }, status: number) {
  await api.post('/v1/admin/orders/status', { order_no: row.order_no, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>VIP 订单</h1>
    <el-table :data="data?.list || []">
      <el-table-column prop="order_no" label="订单号" />
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="price" label="金额" width="100" />
      <el-table-column prop="status" label="状态" width="90" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 2)">退款</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 3)">取消</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
