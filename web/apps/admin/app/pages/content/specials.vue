<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-specials', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/specials', {
    page: 1,
    page_size: 20,
  }),
)
async function setDisplay(row: { id: number }, display: number) {
  await api.post('/v1/admin/specials/display', { id: row.id, display })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.specials') }}</h1>
      <el-table-column prop="title" :label="$t('ui.title')" />
      <el-table-column prop="status" label="display" width="90" />
      <el-table-column :label="$t('ui.action')" width="180">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="setDisplay(row, 1)">{{ $t('ui.display') }}</el-button>
          <el-button size="small" @click="setDisplay(row, 0)">{{ $t('ui.hidden') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
