<script setup lang="ts">
const api = useApi()
const orderStatus = ref<number | undefined>()
const { data, refresh } = await useAsyncData(
  () => `admin-orders-${orderStatus.value ?? 'all'}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/orders', {
      page: 1,
      page_size: 20,
      status: orderStatus.value,
    }),
)
watch(orderStatus, () => refresh())
async function setStatus(row: { order_no: string }, status: number) {
  await api.post('/v1/admin/orders/status', { order_no: row.order_no, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.orders') }}</h1>
    <el-select v-model="orderStatus" clearable style="width: 160px; margin-bottom: 12px" :placeholder="$t('ui.status')">
      <el-option :value="1" label="1" />
      <el-option :value="2" label="2" />
      <el-option :value="3" label="3" />
    </el-select>
    <el-table :data="data?.list || []">
      <el-table-column prop="order_no" :label="$t('ui.order_no')" />
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="price" :label="$t('ui.amount')" width="100" />
      <el-table-column prop="order_state" :label="$t('ui.status')" width="110" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 2)">{{ $t('ui.refund') }}</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 3)">{{ $t('common.cancel') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
