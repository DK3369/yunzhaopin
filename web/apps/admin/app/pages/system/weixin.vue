<script setup lang="ts">
const api = useApi()
const { data, refresh } = await useAsyncData('admin-weixin', () =>
  api.post<Array<Record<string, unknown>>>('/v1/admin/site-settings/list', {}),
)
const { data: menus, error: menuError, refresh: refreshMenus } = await useAsyncData(
  'admin-wx-navs',
  () => api.post<Array<Record<string, unknown>>>('/v1/admin/wx-navs', {}),
)
const form = reactive({ key: 'wx_appid', value: '', description: '', is_public: false })
const menuForm = reactive({ id: 0, name: '', keyid: 0, key: '', url: '', type: 'view', sort: 0 })
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
async function saveMenu() {
  await api.post('/v1/admin/wx-navs/upsert', {
    id: menuForm.id || undefined,
    name: menuForm.name,
    keyid: menuForm.keyid,
    key: menuForm.key,
    url: menuForm.url,
    type: menuForm.type,
    sort: menuForm.sort,
  })
  menuForm.id = 0
  menuForm.name = ''
  refreshMenus()
}
async function removeMenu(row: { id: number }) {
  await api.post('/v1/admin/wx-navs/delete', { id: row.id })
  refreshMenus()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.weixin') }}</h1>
    <el-form inline>
      <el-form-item><el-input v-model="form.key" placeholder="wx_*" /></el-form-item>
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
    <h2 style="margin-top: 24px">{{ $t('ui.wx_menu') }}</h2>
    <el-form inline>
      <el-form-item><el-input v-model="menuForm.name" :placeholder="$t('ui.name')" /></el-form-item>
      <el-form-item><el-input v-model="menuForm.url" placeholder="url" /></el-form-item>
      <el-form-item><el-input v-model="menuForm.type" placeholder="type" /></el-form-item>
      <el-button type="primary" @click="saveMenu">{{ $t('common.save') }}</el-button>
    </el-form>
    <AdminState :error="menuError" :empty="!menuError && !(menus || []).length" />
    <el-table v-if="!menuError && (menus || []).length" :data="menus || []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="name" :label="$t('ui.name')" />
      <el-table-column prop="keyid" label="keyid" width="90" />
      <el-table-column prop="key" label="key" />
      <el-table-column prop="type" :label="$t('ui.type')" width="100" />
      <el-table-column prop="url" label="url" />
      <el-table-column prop="sort" label="sort" width="80" />
      <el-table-column :label="$t('ui.action')" width="100">
        <template #default="{ row }">
          <el-button size="small" type="danger" @click="removeMenu(row)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
