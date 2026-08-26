<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const keyword = ref('')
const usertype = ref<number | undefined>()
const token = ref('')
const { data, refresh } = await useAsyncData('admin-users', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/users', {
    page: 1,
    page_size: 20,
    keyword: keyword.value || undefined,
    usertype: usertype.value,
  }),
)
async function search() {
  await refresh()
}
async function setStatus(row: { uid: number }, status: number) {
  await api.post('/v1/admin/users/status', { uid: row.uid, status })
  refresh()
}
async function impersonate(row: { uid: number }) {
  const r = await api.post<{ uid: number; usertype: number; access_token: string }>(
    '/v1/admin/users/impersonate',
    { uid: row.uid },
  )
  token.value = r.access_token
  await navigator.clipboard?.writeText(r.access_token).catch(() => undefined)
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.users') }}</h1>
    <el-form inline @submit.prevent="search">
      <el-form-item>
        <el-input v-model="keyword" :placeholder="$t('ui.username')" clearable />
      </el-form-item>
      <el-form-item>
        <el-select v-model="usertype" :placeholder="$t('ui.type')" clearable style="width: 140px">
          <el-option :value="1" :label="$t('ui.jobseeker')" />
          <el-option :value="2" :label="$t('common.company')" />
          <el-option :value="3" :label="$t('ui.admin')" />
        </el-select>
      </el-form-item>
      <el-button type="primary" @click="search">{{ $t('ui.query') }}</el-button>
    </el-form>
    <p v-if="token" class="muted">{{ $t('ui.copy_token') }}: {{ token }}</p>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="username" :label="$t('ui.username')" />
      <el-table-column prop="usertype_n" :label="$t('ui.type')" width="110" />
      <el-table-column prop="status_n" :label="$t('ui.status')" width="110" />
      <el-table-column prop="moblie" :label="$t('ui.mobile')" />
      <el-table-column :label="$t('ui.action')" width="280">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">{{ $t('ui.unfreeze') }}</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 0)">{{ $t('ui.freeze') }}</el-button>
          <el-button v-if="row.usertype !== 3" size="small" type="primary" @click="impersonate(row)">{{
            $t('ui.impersonate')
          }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
