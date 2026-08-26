<script setup lang="ts">
const api = useApi()
const form = reactive({
  id: 0,
  title: '',
  keyword: '',
  description: '',
  content: '',
  pic: '',
  startime: 0,
  endtime: 0,
})
const { data, refresh } = await useAsyncData('admin-gongzhao', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/gongzhao/list', {
    page: 1,
    page_size: 20,
  }),
)
async function save() {
  await api.post('/v1/admin/gongzhao', { ...form, id: form.id || undefined })
  form.id = 0
  form.title = ''
  form.content = ''
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/gongzhao/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>公招</h1>
    <el-form label-width="90px" style="max-width: 720px">
      <el-form-item label="标题"><el-input v-model="form.title" /></el-form-item>
      <el-form-item label="startime"><el-input-number v-model="form.startime" :min="0" /></el-form-item>
      <el-form-item label="endtime"><el-input-number v-model="form.endtime" :min="0" /></el-form-item>
      <el-form-item label="内容"><el-input v-model="form.content" type="textarea" :rows="4" /></el-form-item>
      <el-button type="primary" @click="save">保存</el-button>
    </el-form>
    <el-table :data="data?.list || []" style="margin-top: 16px">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" label="标题" />
      <el-table-column label="操作" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
