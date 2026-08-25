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
    <h1>警告</h1>
    <el-form inline>
      <el-form-item><el-input-number v-model="form.target_uid" :min="1" /></el-form-item>
      <el-form-item>
        <el-select v-model="form.target_kind" style="width: 120px">
          <el-option :value="1" label="用户" />
          <el-option :value="2" label="企业" />
          <el-option :value="3" label="职位" />
          <el-option :value="4" label="简历" />
        </el-select>
      </el-form-item>
      <el-form-item><el-input v-model="form.reason" placeholder="原因" /></el-form-item>
      <el-button type="primary" @click="issue">下发</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="target_uid" label="对象UID" />
      <el-table-column prop="target_kind_n" label="类型" />
      <el-table-column prop="reason" label="原因" />
      <el-table-column prop="created_at_n" label="时间" />
    </el-table>
  </div>
</template>
