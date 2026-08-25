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
    <h1>系统广播</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.title" placeholder="标题" /></el-form-item>
      <el-form-item><el-input v-model="form.body" placeholder="正文" /></el-form-item>
      <el-form-item>
        <el-select v-model="form.target_usertype" style="width: 140px">
          <el-option :value="0" label="全部" />
          <el-option :value="1" label="求职者" />
          <el-option :value="2" label="企业" />
        </el-select>
      </el-form-item>
      <el-button type="primary" @click="create">发送</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" label="标题" />
      <el-table-column prop="target_usertype_n" label="对象" width="110" />
      <el-table-column prop="created_at_n" label="时间" />
      <el-table-column label="操作" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
