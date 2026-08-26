<script setup lang="ts">
const api = useApi()
const status = ref<number | undefined>(0)
const { data, refresh } = await useAsyncData(
  () => `admin-questions-${status.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/questions', {
      page: 1,
      page_size: 20,
      status: status.value,
    }),
)
watch(status, () => refresh())
async function setState(row: { id: number }, state: number) {
  await api.post('/v1/admin/questions/state', { id: row.id, state })
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/questions/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.questions') }}</h1>
    <el-radio-group v-model="status" style="margin-bottom: 12px">
      <el-radio-button :value="0">{{ $t('ui.waiting') }}</el-radio-button>
      <el-radio-button :value="1">{{ $t('ui.passed') }}</el-radio-button>
      <el-radio-button :value="2">{{ $t('ui.deleted') }}</el-radio-button>
    </el-radio-group>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" :label="$t('ui.title')" />
      <el-table-column prop="status" label="state" width="80" />
      <el-table-column :label="$t('ui.action')" width="220">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="setState(row, 1)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" @click="setState(row, 0)">{{ $t('ui.waiting') }}</el-button>
          <el-button size="small" type="danger" @click="remove(row)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
