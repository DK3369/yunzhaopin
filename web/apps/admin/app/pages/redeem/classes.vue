<script setup lang="ts">
const api = useApi()
const form = reactive({ parent_id: 0, name: '', sort: 0 })
const { data, refresh } = await useAsyncData('admin-redeem-classes', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/redeem-classes/list', {}),
)
async function create() {
  const body: Record<string, unknown> = { name: form.name, sort: form.sort }
  if (form.parent_id > 0) body.parent_id = form.parent_id
  await api.post('/v1/admin/redeem-classes', body)
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/redeem-classes/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.redeem_classes') }}</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.name" :placeholder="$t('ui.name')" /></el-form-item>
      <el-form-item><el-input-number v-model="form.parent_id" :min="0" /></el-form-item>
      <el-button type="primary" @click="create">{{ $t('ui.add') }}</el-button>
    </el-form>
    <el-table :data="Array.isArray(data) ? data : []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="parent_id" :label="$t('ui.parent')" width="80" />
      <el-table-column prop="name" :label="$t('ui.name')" />
      <el-table-column :label="$t('ui.action')" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
