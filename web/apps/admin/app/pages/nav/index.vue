<script setup lang="ts">
const api = useApi()
const position = ref('')
const form = reactive({ position: 'top', label: '', url: '/', icon: '', parent_id: 0, sort: 0 })
const { data, refresh } = await useAsyncData('admin-nav', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/nav/list', {
    position: position.value || undefined,
  }),
)
async function create() {
  const body: Record<string, unknown> = {
    position: form.position,
    label: form.label,
    url: form.url,
    icon: form.icon,
    sort: form.sort,
  }
  if (form.parent_id > 0) body.parent_id = form.parent_id
  await api.post('/v1/admin/nav', body)
  refresh()
}
async function setStatus(row: { id: number }, status: number) {
  await api.post('/v1/admin/nav/update', { id: row.id, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>导航</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.position" placeholder="position" /></el-form-item>
      <el-form-item><el-input v-model="form.label" placeholder="文案" /></el-form-item>
      <el-form-item><el-input v-model="form.url" placeholder="URL" /></el-form-item>
      <el-button type="primary" @click="create">新增</el-button>
    </el-form>
    <el-table :data="Array.isArray(data) ? data : []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="position" label="位置" />
      <el-table-column prop="label" label="文案" />
      <el-table-column prop="url" label="URL" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">上线</el-button>
          <el-button size="small" @click="setStatus(row, 0)">下线</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 2)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
