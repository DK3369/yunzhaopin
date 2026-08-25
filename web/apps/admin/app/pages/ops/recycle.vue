<script setup lang="ts">
const api = useApi()
const tablename = ref('')
const { data, refresh } = await useAsyncData('admin-recycle', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/recycle-bin', {
    page: 1,
    page_size: 20,
    tablename: tablename.value || undefined,
  }),
)
const detail = ref<unknown>(null)
async function show(row: { id: number }) {
  detail.value = await api.post('/v1/admin/recycle-bin/detail', { id: row.id })
}
async function purge(row: { id: number }) {
  await api.post('/v1/admin/recycle-bin/purge', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>回收站</h1>
    <el-form inline>
      <el-form-item><el-input v-model="tablename" placeholder="表名" /></el-form-item>
      <el-button @click="refresh()">查询</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="tablename" label="表" />
      <el-table-column prop="row_id" label="行 ID" width="90" />
      <el-table-column prop="note" label="备注" />
      <el-table-column label="操作" width="180">
        <template #default="{ row }">
          <el-button size="small" @click="show(row)">详情</el-button>
          <el-button size="small" type="danger" @click="purge(row)">彻底删除</el-button>
        </template>
      </el-table-column>
    </el-table>
    <pre v-if="detail" style="margin-top: 16px; white-space: pre-wrap">{{ JSON.stringify(detail, null, 2) }}</pre>
  </div>
</template>
