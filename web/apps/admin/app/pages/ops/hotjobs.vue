<script setup lang="ts">
const api = useApi()
const form = reactive({
  id: 0,
  uid: 1,
  username: '',
  hot_pic: '',
  time_start: 0,
  time_end: 0,
  sort: 0,
  beizhu: '',
})
const { data, refresh } = await useAsyncData('admin-hotjobs', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/hotjobs/list', {
    page: 1,
    page_size: 20,
  }),
)
async function save() {
  await api.post('/v1/admin/hotjobs', { ...form, id: form.id || undefined })
  form.id = 0
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/hotjobs/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>名企招聘 / 热招</h1>
    <el-form inline>
      <el-form-item><el-input-number v-model="form.uid" :min="1" /></el-form-item>
      <el-form-item><el-input v-model="form.username" placeholder="username" /></el-form-item>
      <el-form-item><el-input-number v-model="form.time_start" :min="0" /></el-form-item>
      <el-form-item><el-input-number v-model="form.time_end" :min="0" /></el-form-item>
      <el-button type="primary" @click="save">保存</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="username" label="企业" />
      <el-table-column prop="time_end" label="time_end" width="120" />
      <el-table-column label="操作" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
