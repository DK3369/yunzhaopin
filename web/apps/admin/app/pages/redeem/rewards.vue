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
    <h1>{{ $t('ui.redeem_rewards') }}</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.name" :placeholder="$t('ui.name')" /></el-form-item>
      <el-form-item><el-input-number v-model="form.integral" :min="1" /></el-form-item>
      <el-form-item><el-input v-model="form.content" :placeholder="$t('ui.desc')" /></el-form-item>
      <el-button type="primary" @click="create">{{ $t('ui.add') }}</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" :label="$t('ui.name')" />
      <el-table-column prop="integral" :label="$t('ui.integral')" width="90" />
      <el-table-column prop="stock" :label="$t('ui.stock')" width="80" />
      <el-table-column prop="status" :label="$t('ui.status')" width="80" />
      <el-table-column :label="$t('ui.action')" width="320">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">{{ $t('ui.online') }}</el-button>
          <el-button size="small" @click="setStatus(row, 0)">{{ $t('ui.offline') }}</el-button>
          <el-button size="small" @click="setFlags(row, 1, 0)">{{ $t('common.recommended') }}</el-button>
          <el-button size="small" @click="setFlags(row, 0, 1)">{{ $t('common.hot') }}</el-button>
          <el-button size="small" type="danger" @click="remove(row)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
