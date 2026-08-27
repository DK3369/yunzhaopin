<script setup lang="ts">
const api = useApi()
const selected = ref<number[]>([])
const status = ref<number | undefined>(0)
const { data, refresh } = await useAsyncData(
  () => `admin-reports-${status.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/reports', {
      page: 1,
      page_size: 20,
      status: status.value,
    }),
)
watch(status, () => refresh())
function onSelect(rows: Array<{ id: number }>) {
  selected.value = rows.map((r) => r.id)
}
async function setStatus(row: { id: number }, status: number) {
  await api.post('/v1/admin/reports/status', { id: row.id, status })
  refresh()
}
async function batch(status: number) {
  if (!selected.value.length) return
  await api.post('/v1/admin/reports/batch/status', { ids: selected.value, status })
  selected.value = []
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.reports') }}</h1>
    <el-radio-group v-model="status" style="margin-bottom: 12px">
      <el-radio-button :value="0">{{ $t('ui.waiting') }}</el-radio-button>
      <el-radio-button :value="1">{{ $t('ui.handle') }}</el-radio-button>
      <el-radio-button :value="2">{{ $t('ui.dismiss') }}</el-radio-button>
    </el-radio-group>
    <div style="margin-bottom: 12px">
      <el-button size="small" @click="batch(1)">{{ $t('ui.batch_handle') }}</el-button>
      <el-button size="small" type="danger" @click="batch(2)">{{ $t('ui.batch_dismiss') }}</el-button>
    </div>
    <el-table :data="data?.list || []" @selection-change="onSelect">
      <el-table-column type="selection" width="48" />
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="target_kind_n" :label="$t('ui.type')" />
      <el-table-column prop="reason_code" :label="$t('ui.reason')" />
      <el-table-column prop="status_n" :label="$t('ui.status')" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">{{ $t('ui.handle') }}</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 2)">{{ $t('ui.dismiss') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
