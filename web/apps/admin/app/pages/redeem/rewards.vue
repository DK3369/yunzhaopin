<script setup lang="ts">
const api = useApi()
const form = reactive({
  name: '',
  pic: '',
  content: '',
  integral: 1,
  stock: 0,
  restriction: 0,
  nid: 0,
  tnid: 0,
})
const { data, refresh } = await useAsyncData('admin-rewards', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/rewards/list', { page: 1, page_size: 20 }),
)
async function create() {
  const body: Record<string, unknown> = {
    name: form.name,
    pic: form.pic,
    content: form.content,
    integral: form.integral,
    stock: form.stock,
    restriction: form.restriction,
  }
  if (form.nid > 0) body.nid = form.nid
  if (form.tnid > 0) body.tnid = form.tnid
  await api.post('/v1/admin/rewards', body)
  refresh()
}
async function setStatus(row: { id: number }, status: number) {
  await api.post('/v1/admin/rewards/status', { id: row.id, status })
  refresh()
}
async function setFlags(row: { id: number }, is_rec: number, is_hot: number) {
  await api.post('/v1/admin/rewards/flags', { id: row.id, is_rec, is_hot })
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/rewards/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>积分商品</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.name" placeholder="名称" /></el-form-item>
      <el-form-item><el-input-number v-model="form.integral" :min="1" /></el-form-item>
      <el-form-item><el-input v-model="form.content" placeholder="说明" /></el-form-item>
      <el-button type="primary" @click="create">新增</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="名称" />
      <el-table-column prop="integral" label="积分" width="90" />
      <el-table-column prop="stock" label="库存" width="80" />
      <el-table-column prop="status" label="状态" width="80" />
      <el-table-column label="操作" width="320">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">上架</el-button>
          <el-button size="small" @click="setStatus(row, 0)">下架</el-button>
          <el-button size="small" @click="setFlags(row, 1, 0)">推荐</el-button>
          <el-button size="small" @click="setFlags(row, 0, 1)">热门</el-button>
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
