<script setup lang="ts">
const api = useApi()
const form = reactive({ title: '', body: '', target_usertype: 0 })
const { data, refresh } = await useAsyncData('admin-broadcasts', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/broadcasts/list', { page: 1, page_size: 20 }),
)
async function create() {
  await api.post('/v1/admin/broadcasts', { ...form })
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/broadcasts/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.broadcasts') }}</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.title" :placeholder="$t('ui.title')" /></el-form-item>
      <el-form-item><el-input v-model="form.body" :placeholder="$t('ui.body')" /></el-form-item>
      <el-form-item>
        <el-select v-model="form.target_usertype" style="width: 140px">
          <el-option :value="0" :label="$t('common.all')" />
          <el-option :value="1" :label="$t('ui.jobseeker')" />
          <el-option :value="2" :label="$t('common.company')" />
        </el-select>
      </el-form-item>
      <el-button type="primary" @click="create">{{ $t('ui.send') }}</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" :label="$t('ui.title')" />
      <el-table-column prop="target_usertype_n" :label="$t('ui.target')" width="110" />
      <el-table-column prop="created_at_n" :label="$t('ui.time')" />
      <el-table-column :label="$t('ui.action')" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
