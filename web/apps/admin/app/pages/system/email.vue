<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-email', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/site-settings/list', {}),
)
const { data: logs } = await useAsyncData('admin-email-logs', () =>
  api
    .post<{ list: Array<Record<string, unknown>>; total: number }>('/v1/admin/email-logs', { page: 1, page_size: 20 })
    .catch(() => ({ list: [], total: 0 })),
)
const form = reactive({ key: 'sy_email_online', value: '', description: '', is_public: false })
const rows = computed(() => {
  const all = Array.isArray(data.value) ? data.value : []
  return all.filter((r) => {
    const k = String(r.key || '')
    return (
      k.startsWith('sy_email') ||
      k === 'accesskey' ||
      k === 'accesssecret' ||
      k.startsWith('ali_email') ||
      k.startsWith('ali_tag') ||
      k.startsWith('ali_name')
    )
  })
})
async function upsert() {
  await api.post('/v1/admin/site-settings', { ...form })
  refresh()
}
function fill(row: Record<string, unknown>) {
  form.key = String(row.key || '')
  form.value = String(row.value || '')
  form.description = String(row.description || '')
  form.is_public = Boolean(row.is_public)
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.email') }}</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.key" placeholder="sy_email_*" /></el-form-item>
      <el-form-item><el-input v-model="form.value" placeholder="value" /></el-form-item>
      <el-button type="primary" @click="upsert">{{ $t('common.save') }}</el-button>
    </el-form>
    <el-table :data="rows">
      <el-table-column prop="key" label="key" />
      <el-table-column prop="value" label="value" />
      <el-table-column :label="$t('ui.action')" width="120">
        <template #default="{ row }">
          <el-button size="small" @click="fill(row)">{{ $t('ui.fill') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
    <h2 style="margin-top: 24px">{{ $t('ui.email_log') }}</h2>
    <el-table :data="logs?.list || []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="email" label="email" />
      <el-table-column prop="title" :label="$t('ui.name')" />
      <el-table-column prop="state" :label="$t('ui.status')" width="90" />
      <el-table-column prop="ctime" label="ctime" width="120" />
    </el-table>
  </div>
</template>
