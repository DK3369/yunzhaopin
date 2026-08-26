<script setup lang="ts">
const api = useApi()
const form = reactive({ target_uid: 1, target_kind: 1, target_id: 0, reason: '' })
const { data, refresh } = await useAsyncData('admin-warnings', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/warnings/list', { page: 1, page_size: 20 }),
)
async function issue() {
  const body: Record<string, unknown> = {
    target_uid: form.target_uid,
    target_kind: form.target_kind,
    reason: form.reason,
  }
  if (form.target_id > 0) body.target_id = form.target_id
  await api.post('/v1/admin/warnings', body)
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.warnings') }}</h1>
    <el-form inline>
      <el-form-item><el-input-number v-model="form.target_uid" :min="1" /></el-form-item>
      <el-form-item>
        <el-select v-model="form.target_kind" style="width: 120px">
          <el-option :value="1" :label="$t('ui.user_kind')" />
          <el-option :value="2" :label="$t('common.company')" />
          <el-option :value="3" :label="$t('common.job')" />
          <el-option :value="4" :label="$t('common.resume')" />
        </el-select>
      </el-form-item>
      <el-form-item><el-input v-model="form.reason" :placeholder="$t('ui.reason')" /></el-form-item>
      <el-button type="primary" @click="issue">{{ $t('ui.issue') }}</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="target_uid" :label="$t('ui.target')" />
      <el-table-column prop="target_kind_n" :label="$t('ui.type')" />
      <el-table-column prop="reason" :label="$t('ui.reason')" />
      <el-table-column prop="created_at_n" :label="$t('ui.time')" />
    </el-table>
  </div>
</template>
