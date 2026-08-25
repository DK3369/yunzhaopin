<script setup lang="ts">
const api = useApi()
const slot = ref('')
const form = reactive({ slot: '', title: '', image: '', link: '', weight: 0, start_at: 0, end_at: 0 })
const { data, refresh } = await useAsyncData('admin-ads', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/ads/list', {
    page: 1,
    page_size: 20,
    slot: slot.value || undefined,
  }),
)
async function create() {
  await api.post('/v1/admin/ads', { ...form })
  refresh()
}
async function setStatus(row: { id: number }, status: number) {
  await api.post('/v1/admin/ads/update', { id: row.id, status })
  refresh()
}
</script>

<template>
  <div>
    <h1>广告位</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.slot" placeholder="slot" /></el-form-item>
      <el-form-item><el-input v-model="form.title" placeholder="标题" /></el-form-item>
      <el-form-item><el-input v-model="form.image" placeholder="图片 URL" /></el-form-item>
      <el-form-item><el-input v-model="form.link" placeholder="链接" /></el-form-item>
      <el-button type="primary" @click="create">新增</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="slot" label="槽位" />
      <el-table-column prop="title" label="标题" />
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
