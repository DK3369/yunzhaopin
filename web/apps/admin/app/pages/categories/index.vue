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
    <h1>分类</h1>
    <el-form inline>
      <el-form-item>
        <el-input v-model="kind" placeholder="kind，如 job / industry" />
      </el-form-item>
      <el-form-item><el-input v-model="form.name" placeholder="名称" /></el-form-item>
      <el-button type="primary" @click="create">新增</el-button>
    </el-form>
    <el-table :data="Array.isArray(data) ? data : []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="kind" label="kind" />
      <el-table-column prop="name" label="名称" />
      <el-table-column prop="status" label="状态" width="80" />
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
