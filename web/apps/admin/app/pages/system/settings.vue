<script setup lang="ts">
const route = useRoute()
const api = useApi()
const form = reactive({ key: '', value: '', description: '', is_public: false })
const { data, refresh } = await useAsyncData('admin-settings', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/site-settings/list', {}),
)
const { t } = useI18n()
const allGroups = computed(() => [
  { name: `${t('ui.site_sy')} sy_web`, prefixes: ['sy_web', 'sy_webname', 'sy_weburl', 'sy_webemail'], paths: ['/set'] },
  { name: `${t('ui.reg_sy')} sy_reg`, prefixes: ['sy_reg'], paths: ['/regset'] },
  { name: `${t('ui.integral_sy')} sy_integral`, prefixes: ['sy_integral'], paths: ['/jifenset'] },
  { name: `${t('ui.pay_sy')} sy_alipay / sy_wxpay`, prefixes: ['sy_alipay', 'sy_wxpay', 'sy_tenpay'], paths: ['/payset'] },
  { name: 'SEO sy_seo', prefixes: ['sy_seo'], paths: ['/seoset', '/yemainset'] },
  { name: `${t('ui.email')} sy_email`, prefixes: ['sy_email'], paths: ['/emailset'] },
  { name: `${t('ui.sms')} sy_msg`, prefixes: ['sy_msg'], paths: ['/messageset'] },
  { name: `${t('ui.weixin')} wx_ / sy_wx`, prefixes: ['wx_', 'sy_wx'], paths: ['/weixinmenu'] },
  { name: `${t('ui.locoy')} locoy_`, prefixes: ['locoy_'], paths: ['/moduleset'] },
  { name: 'code_', prefixes: ['code_'], paths: ['/set'] },
])
const groups = computed(() => {
  const p = route.path
  const matched = allGroups.value.filter((g) => g.paths.includes(p))
  if (matched.length) return matched
  return allGroups.value
})
const routeFiltered = computed(() => allGroups.value.some((g) => g.paths.includes(route.path)))
function rowsOf(prefixes: string[]) {
  const all = Array.isArray(data.value) ? data.value : []
  return all.filter((r) => prefixes.some((p) => String(r.key || '').startsWith(p)))
}
const ungrouped = computed(() => {
  const all = Array.isArray(data.value) ? data.value : []
  const prefixed = groups.value.flatMap((g) => g.prefixes)
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
    <h1>{{ $t('ui.settings') }}</h1>
    <p>{{ $t('ui.settings_hint') }}</p>
    <el-form inline>
      <el-form-item><el-input v-model="form.key" placeholder="key" /></el-form-item>
      <el-form-item><el-input v-model="form.value" placeholder="value" /></el-form-item>
      <el-form-item><el-input v-model="form.description" :placeholder="$t('ui.desc')" /></el-form-item>
      <el-form-item><el-checkbox v-model="form.is_public">{{ $t('ui.open') }}</el-checkbox></el-form-item>
      <el-button type="primary" @click="upsert">{{ $t('common.save') }}</el-button>
    </el-form>
    <el-tabs>
      <el-tab-pane v-for="g in groups" :key="g.name" :label="g.name">
        <el-table :data="rowsOf(g.prefixes)" size="small">
          <el-table-column prop="key" label="key" />
          <el-table-column prop="value" label="value" />
          <el-table-column :label="$t('ui.action')" width="160">
            <template #default="{ row }">
              <el-button size="small" @click="fill(row)">{{ $t('ui.fill') }}</el-button>
              <el-button size="small" type="danger" @click="remove(row)">{{ $t('common.delete') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
      <el-tab-pane v-if="!routeFiltered" :label="$t('ui.other_keys')">
        <el-table :data="ungrouped" size="small">
          <el-table-column prop="key" label="key" />
          <el-table-column prop="value" label="value" />
          <el-table-column :label="$t('ui.action')" width="160">
            <template #default="{ row }">
              <el-button size="small" @click="fill(row)">{{ $t('ui.fill') }}</el-button>
              <el-button size="small" type="danger" @click="remove(row)">{{ $t('common.delete') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>
