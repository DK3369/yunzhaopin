<script setup lang="ts">
const api = useApi()
const kind = ref('job')
const form = reactive({ kind: 'job', name: '', parent_id: 0, sort: 0 })
const { data, refresh } = await useAsyncData(
  () => `admin-cats-${kind.value}`,
  () => api.post<Array<Record<string, unknown>>>('/v1/admin/categories/list', { kind: kind.value }),
)
watch(kind, () => refresh())
async function create() {
  const body: Record<string, unknown> = { kind: kind.value, name: form.name, sort: form.sort }
  if (form.parent_id > 0) body.parent_id = form.parent_id
  await api.post('/v1/admin/categories', body)
  refresh()
}
async function setStatus(row: { id: number }, status: number) {
  await api.post('/v1/admin/categories/update', { id: row.id, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.categories') }}</h1>
    <el-form inline>
      <el-form-item>
        <el-input v-model="kind" placeholder="kind" />
      </el-form-item>
      <el-form-item><el-input v-model="form.name" :placeholder="$t('ui.name')" /></el-form-item>
      <el-button type="primary" @click="create">{{ $t('ui.add') }}</el-button>
    </el-form>
    <el-table :data="Array.isArray(data) ? data : []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="kind" label="kind" />
      <el-table-column prop="name" :label="$t('ui.name')" />
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
