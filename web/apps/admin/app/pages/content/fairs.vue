<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-fairs', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/fairs', {
    page: 1,
    page_size: 20,
  }),
)
async function setOpen(row: { id: number }, is_open: number) {
  await api.post('/v1/admin/fairs/open', { id: row.id, is_open })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.fairs') }}</h1>
      <el-table-column prop="title" :label="$t('ui.title')" />
      <el-table-column prop="is_open" label="is_open" width="90" />
      <el-table-column :label="$t('ui.action')" width="180">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="setOpen(row, 1)">{{ $t('ui.open_on') }}</el-button>
          <el-button size="small" @click="setOpen(row, 0)">{{ $t('common.close') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
