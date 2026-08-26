<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-weixin', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/site-settings/list', {}),
)
const form = reactive({ key: 'wx_appid', value: '', description: '', is_public: false })
const rows = computed(() => {
  const all = Array.isArray(data.value) ? data.value : []
  return all.filter((r) => {
    const k = String(r.key || '')
    return k.startsWith('wx_') || k.startsWith('sy_wx')
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
    <h1>微信配置</h1>
    <p>PHP <code>weixinmenu</code> 公众号参数在 config KV（<code>wx_*</code> / <code>sy_wx*</code>）。自定义菜单表本页不解析。</p>
    <el-form inline>
      <el-form-item><el-input v-model="form.key" placeholder="wx_*" /></el-form-item>
      <el-form-item><el-input v-model="form.value" placeholder="value" /></el-form-item>
      <el-button type="primary" @click="upsert">保存</el-button>
    </el-form>
    <el-table :data="rows">
      <el-table-column prop="key" label="key" />
      <el-table-column prop="value" label="value" />
      <el-table-column label="操作" width="120">
        <template #default="{ row }">
          <el-button size="small" @click="fill(row)">填入</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
