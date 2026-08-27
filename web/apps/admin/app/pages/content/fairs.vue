<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-fairs', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/fairs', {
    page: 1,
    page_size: 20,
  }),
)
const spaceForm = reactive({ id: 0, name: '', sort: 0, keyid: 0, price: 0, content: '' })
const { data: spaces, refresh: refreshSpaces } = await useAsyncData('admin-fair-spaces', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/fairs/spaces', { keyid: 0 }),
)
async function setOpen(row: { id: number }, is_open: number) {
  await api.post('/v1/admin/fairs/open', { id: row.id, is_open })
  refresh()
}
async function saveSpace() {
  await api.post('/v1/admin/fairs/spaces/upsert', {
    id: spaceForm.id || undefined,
    name: spaceForm.name,
    sort: spaceForm.sort,
    keyid: spaceForm.keyid,
    price: spaceForm.price,
    content: spaceForm.content,
  })
  spaceForm.id = 0
  spaceForm.name = ''
  refreshSpaces()
}
async function removeSpace(row: { id: number }) {
  await api.post('/v1/admin/fairs/spaces/delete', { id: row.id })
  refreshSpaces()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.fairs') }}</h1>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="title" :label="$t('ui.title')" />
      <el-table-column prop="is_open" label="is_open" width="90" />
      <el-table-column :label="$t('ui.action')" width="180">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="setOpen(row, 1)">{{ $t('ui.open_on') }}</el-button>
          <el-button size="small" @click="setOpen(row, 0)">{{ $t('common.close') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
    <h2 style="margin-top: 24px">{{ $t('ui.space') }}</h2>
    <el-form inline>
      <el-form-item><el-input v-model="spaceForm.name" :placeholder="$t('ui.name')" /></el-form-item>
      <el-form-item><el-input-number v-model="spaceForm.sort" :min="0" /></el-form-item>
      <el-form-item><el-input-number v-model="spaceForm.price" :min="0" /></el-form-item>
      <el-button type="primary" @click="saveSpace">{{ $t('common.save') }}</el-button>
    </el-form>
    <el-table :data="Array.isArray(spaces) ? spaces : []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="name" :label="$t('ui.name')" />
      <el-table-column prop="keyid" label="keyid" width="90" />
      <el-table-column prop="price" :label="$t('ui.amount')" width="90" />
      <el-table-column :label="$t('ui.action')" width="120">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="removeSpace(row)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
