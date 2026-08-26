<script setup lang="ts">
const api = useApi()
const slot = ref('')
const form = reactive({ slot: '', title: '', image: '', link: '', weight: 0, start_at: 0, end_at: 0 })
const { data, refresh } = await useAsyncData('admin-ads', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/ads/list', {
    page: 1,
    page_size: 20,
    slot: slot.value || undefined,
  }),
)
async function create() {
  await api.post('/v1/admin/ads', { ...form })
  refresh()
}
async function setStatus(row: { id: number }, status: number) {
  await api.post('/v1/admin/ads/update', { id: row.id, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.ads') }}</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.slot" placeholder="slot" /></el-form-item>
      <el-form-item><el-input v-model="form.title" :placeholder="$t('ui.title')" /></el-form-item>
      <el-form-item><el-input v-model="form.image" :placeholder="$t('ui.image_url')" /></el-form-item>
      <el-form-item><el-input v-model="form.link" :placeholder="$t('ui.link')" /></el-form-item>
      <el-button type="primary" @click="create">{{ $t('ui.add') }}</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="slot" :label="$t('ui.slot')" />
      <el-table-column prop="title" :label="$t('ui.title')" />
      <el-table-column prop="status" :label="$t('ui.status')" width="80" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">{{ $t('ui.online') }}</el-button>
          <el-button size="small" @click="setStatus(row, 0)">{{ $t('ui.offline') }}</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 2)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
