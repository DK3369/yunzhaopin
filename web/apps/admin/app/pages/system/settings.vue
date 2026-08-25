<script setup lang="ts">
const api = useApi()
const form = reactive({ key: '', value: '', description: '', is_public: false })
const { data, refresh } = await useAsyncData('admin-settings', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/site-settings/list', {}),
)
async function upsert() {
  await api.post('/v1/admin/site-settings', { ...form })
  refresh()
}
async function remove(row: { key: string }) {
  await api.post('/v1/admin/site-settings/delete', { key: row.key })
  refresh()
}
</script>

<template>
  <div>
    <h1>站点配置（KV）</h1>
    <p>对应 PHP 后台多屏配置的通用存储，不是 38 个 set_* 分屏 1:1。</p>
    <el-form inline>
      <el-form-item><el-input v-model="form.key" placeholder="key" /></el-form-item>
      <el-form-item><el-input v-model="form.value" placeholder="value" /></el-form-item>
      <el-form-item><el-input v-model="form.description" placeholder="说明" /></el-form-item>
      <el-form-item><el-checkbox v-model="form.is_public">公开</el-checkbox></el-form-item>
      <el-button type="primary" @click="upsert">保存</el-button>
    </el-form>
    <el-table :data="Array.isArray(data) ? data : []">
      <el-table-column prop="key" label="key" />
      <el-table-column prop="value" label="value" />
      <el-table-column prop="description" label="说明" />
      <el-table-column prop="is_public" label="公开" width="80" />
      <el-table-column label="操作" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
