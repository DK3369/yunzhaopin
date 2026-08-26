<script setup lang="ts">
const api = useApi()
const form = reactive({
  id: 0,
  link_name: '',
  link_url: '',
  pic: '',
  link_type: '',
  link_sorting: 0,
  link_state: 1,
})
const { data, refresh } = await useAsyncData('admin-friend-links', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/friend-links/list', {
    page: 1,
    page_size: 50,
  }),
)
async function save() {
  await api.post('/v1/admin/friend-links', { ...form, id: form.id || undefined })
  form.id = 0
  form.link_name = ''
  form.link_url = ''
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/friend-links/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>友情链接</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.link_name" placeholder="link_name" /></el-form-item>
      <el-form-item><el-input v-model="form.link_url" placeholder="link_url" /></el-form-item>
      <el-form-item><el-input v-model="form.link_type" placeholder="link_type" /></el-form-item>
      <el-form-item><el-input-number v-model="form.link_state" :min="0" :max="1" /></el-form-item>
      <el-button type="primary" @click="save">保存</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="名称" />
      <el-table-column prop="url" label="URL" />
      <el-table-column prop="status" label="link_state" width="100" />
      <el-table-column label="操作" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
