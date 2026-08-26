<script setup lang="ts">
const api = useApi()
const form = reactive({ key: '', value: '', description: '', is_public: false })
const { data, refresh } = await useAsyncData('admin-settings', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/site-settings/list', {}),
)
const groups: Array<{ name: string; prefixes: string[] }> = [
  { name: '站点 sy_web / sy_webname', prefixes: ['sy_web', 'sy_webname', 'sy_weburl', 'sy_webemail'] },
  { name: '注册 sy_reg', prefixes: ['sy_reg'] },
  { name: '积分 sy_integral', prefixes: ['sy_integral'] },
  { name: '支付 sy_alipay / sy_wxpay', prefixes: ['sy_alipay', 'sy_wxpay', 'sy_tenpay'] },
  { name: 'SEO sy_seo', prefixes: ['sy_seo'] },
  { name: '采集 locoy_', prefixes: ['locoy_'] },
]
function rowsOf(prefixes: string[]) {
  const all = Array.isArray(data.value) ? data.value : []
  return all.filter((r) => prefixes.some((p) => String(r.key || '').startsWith(p)))
}
const ungrouped = computed(() => {
  const all = Array.isArray(data.value) ? data.value : []
  const prefixed = groups.flatMap((g) => g.prefixes)
  return all.filter((r) => !prefixed.some((p) => String(r.key || '').startsWith(p)))
})
async function upsert() {
  await api.post('/v1/admin/site-settings', { ...form })
  refresh()
}
async function remove(row: { key: string }) {
  await api.post('/v1/admin/site-settings/delete', { key: row.key })
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
    <h1>站点配置（分屏 KV）</h1>
    <p>键名保持 PHP <code>sy_*</code> / <code>locoy_*</code>，走已有 site-settings，不是新表。</p>
    <el-form inline>
      <el-form-item><el-input v-model="form.key" placeholder="key" /></el-form-item>
      <el-form-item><el-input v-model="form.value" placeholder="value" /></el-form-item>
      <el-form-item><el-input v-model="form.description" placeholder="说明" /></el-form-item>
      <el-form-item><el-checkbox v-model="form.is_public">公开</el-checkbox></el-form-item>
      <el-button type="primary" @click="upsert">保存</el-button>
    </el-form>
    <section v-for="g in groups" :key="g.name" style="margin-top: 20px">
      <h2 style="font-size: 15px">{{ g.name }}</h2>
      <el-table :data="rowsOf(g.prefixes)" size="small">
        <el-table-column prop="key" label="key" />
        <el-table-column prop="value" label="value" />
        <el-table-column label="操作" width="160">
          <template #default="{ row }">
            <el-button size="small" @click="fill(row)">填入</el-button>
            <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </section>
    <h2 style="font-size: 15px; margin-top: 20px">其他键</h2>
    <el-table :data="ungrouped" size="small">
      <el-table-column prop="key" label="key" />
      <el-table-column prop="value" label="value" />
      <el-table-column label="操作" width="160">
        <template #default="{ row }">
          <el-button size="small" @click="fill(row)">填入</el-button>
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
