<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-sms', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/site-settings/list', {}),
)
const { data: logs, error: logError } = await useAsyncData('admin-sms-logs', () =>
  api.post<{ list: Array<Record<string, unknown>>; total: number }>('/v1/admin/sms-logs', { page: 1, page_size: 20 }),
)
const form = reactive({ key: 'sy_msg_appkey', value: '', description: '', is_public: false })
const rows = computed(() => {
  const all = Array.isArray(data.value) ? data.value : []
  return all.filter((r) => {
    const k = String(r.key || '')
    return k.startsWith('sy_msg') || k.startsWith('sy_kh') || k.includes('sms')
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
    <h1>{{ $t('ui.sms') }}</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.key" placeholder="sy_msg_*" /></el-form-item>
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
    <h2 style="margin-top: 24px">{{ $t('ui.sms_log') }}</h2>
    <AdminState :error="logError" :empty="!logError && !(logs?.list || []).length" />
    <el-table v-if="!logError && (logs?.list || []).length" :data="logs?.list || []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="moblie" :label="$t('ui.mobile')" />
      <el-table-column prop="content" :label="$t('ui.content')" />
      <el-table-column prop="state" :label="$t('ui.status')" width="90" />
      <el-table-column prop="ctime" label="ctime" width="120" />
    </el-table>
  </div>
</template>
