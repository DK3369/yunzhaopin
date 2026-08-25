<script setup lang="ts">
const api = useApi()
const form = reactive({
  platform: 'android',
  version: '',
  version_code: 1,
  is_force: false,
  download_url: '',
  changelog: '',
  released_at: 0,
})
const { data, refresh } = await useAsyncData('admin-app-versions', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/app-versions/list', { page: 1, page_size: 20 }),
)
async function create() {
  await api.post('/v1/admin/app-versions', { ...form })
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/app-versions/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>App 版本</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.platform" placeholder="platform" /></el-form-item>
      <el-form-item><el-input v-model="form.version" placeholder="version" /></el-form-item>
      <el-form-item><el-input-number v-model="form.version_code" :min="0" /></el-form-item>
      <el-form-item><el-input v-model="form.download_url" placeholder="下载地址" /></el-form-item>
      <el-button type="primary" @click="create">发布</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="platform" label="平台" />
      <el-table-column prop="version" label="版本" />
      <el-table-column prop="version_code" label="code" width="90" />
      <el-table-column prop="is_force" label="强制" width="80" />
      <el-table-column label="操作" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
