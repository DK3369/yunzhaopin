<script setup lang="ts">
const api = useApi()
const classId = ref<number | undefined>()
const className = ref('')
const form = reactive({
  class_id: 1,
  title: '',
  content: '<p></p>',
  is_type: 1,
  link_url: '',
  sort: 0,
  status: 1,
})
const { data: classes, refresh: refreshClasses } = await useAsyncData('admin-desc-classes', () =>
  api.post<Array<{ id: number; name: string; sort: number }>>('/v1/admin/desc-classes/list', {}),
)
const { data, refresh } = await useAsyncData('admin-descriptions', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/descriptions/list', {
    page: 1,
    page_size: 20,
    class_id: classId.value,
  }),
)
async function addClass() {
  await api.post('/v1/admin/desc-classes', { name: className.value, sort: 0 })
  className.value = ''
  refreshClasses()
}
async function deleteClass(row: { id: number }) {
  await api.post('/v1/admin/desc-classes/update', { id: row.id, status: 2 })
  refreshClasses()
}
async function upsert() {
  await api.post('/v1/admin/descriptions', { ...form })
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/descriptions/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.pages') }}</h1>
    <h2>{{ $t('ui.category') }}</h2>
    <el-form inline>
      <el-form-item><el-input v-model="className" :placeholder="$t('ui.class_name')" /></el-form-item>
      <el-button @click="addClass">{{ $t('ui.add_class') }}</el-button>
    </el-form>
    <el-table :data="Array.isArray(classes) ? classes : []" style="margin-bottom: 24px">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" :label="$t('ui.name')" />
      <el-table-column :label="$t('ui.action')" width="120">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="deleteClass(row)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
    <h2>{{ $t('ui.pages_list') }}</h2>
    <el-form label-width="80px" style="max-width: 720px">
      <el-form-item :label="$t('ui.class_id')"><el-input-number v-model="form.class_id" :min="1" /></el-form-item>
      <el-form-item :label="$t('ui.title')"><el-input v-model="form.title" /></el-form-item>
      <el-form-item :label="$t('ui.type')">
        <el-select v-model="form.is_type">
          <el-option :value="1" :label="$t('ui.custom_page')" />
          <el-option :value="2" :label="$t('ui.internal_link')" />
          <el-option :value="3" :label="$t('ui.external_link')" />
        </el-select>
      </el-form-item>
      <el-form-item :label="$t('ui.link')"><el-input v-model="form.link_url" /></el-form-item>
      <el-form-item :label="$t('ui.content')"><el-input v-model="form.content" type="textarea" :rows="6" /></el-form-item>
      <el-button type="primary" @click="upsert">{{ $t('common.save') }}</el-button>
    </el-form>
    <el-table :data="data?.list || []" style="margin-top: 16px">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" :label="$t('ui.title')" />
      <el-table-column prop="class_id" :label="$t('ui.category')" width="90" />
      <el-table-column prop="status" :label="$t('ui.status')" width="80" />
      <el-table-column :label="$t('ui.action')" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
