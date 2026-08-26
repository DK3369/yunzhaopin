<script setup lang="ts">
const api = useApi()
const selected = ref<number[]>([])
const { data, refresh } = await useAsyncData('admin-feedback', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/feedback', { page: 1, page_size: 20 }),
)
function onSelect(rows: Array<{ id: number }>) {
  selected.value = rows.map((r) => r.id)
}
async function resolveOne(row: { id: number }) {
  await api.post('/v1/admin/feedback/status', { id: row.id, status: 1 })
  refresh()
}
async function batch() {
  if (!selected.value.length) return
  await api.post('/v1/admin/feedback/batch/status', { ids: selected.value, status: 1 })
  selected.value = []
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.feedback') }}</h1>
    <el-button size="small" style="margin-bottom: 12px" @click="batch">{{ $t('ui.batch_resolve') }}</el-button>
    <el-table :data="data?.list || []" @selection-change="onSelect">
      <el-table-column type="selection" width="48" />
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="category" :label="$t('ui.category')" />
      <el-table-column prop="content" :label="$t('ui.content')" />
      <el-table-column prop="contact" :label="$t('ui.contact')" />
      <el-table-column prop="status_n" :label="$t('ui.status')" width="110" />
      <el-table-column :label="$t('ui.action')" width="120">
        <template #default="{ row }">
          <el-button size="small" @click="resolveOne(row)">{{ $t('ui.resolved') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
