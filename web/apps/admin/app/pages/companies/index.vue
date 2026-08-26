<script setup lang="ts">
const api = useApi()
const rStatus = ref<number | undefined>()
const keyword = ref('')
const page = ref(1)
const { data, error, refresh } = await useAsyncData(
  () => `admin-companies-${page.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>>; total: number }>('/v1/admin/companies', {
      page: page.value,
      page_size: 20,
      r_status: rStatus.value,
      keyword: keyword.value || undefined,
    }),
)
async function setStatus(row: { uid: number }, r_status: number) {
  await api.post('/v1/admin/companies/status', { uid: row.uid, r_status })
  refresh()
}
async function exportCsv() {
  const r = await api.post<{ filename: string; csv: string }>('/v1/admin/companies/export', {
    r_status: rStatus.value,
    keyword: keyword.value || undefined,
  })
  const blob = new Blob(['\ufeff' + (r.csv || '')], { type: 'text/csv;charset=utf-8' })
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  a.download = r.filename || 'companies.csv'
  a.click()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.companies') }}</h1>
    <el-form inline>
      <el-form-item>
        <el-input v-model="keyword" :placeholder="$t('ui.company_name')" clearable />
      </el-form-item>
      <el-form-item>
        <el-select v-model="rStatus" placeholder="r_status" clearable style="width: 140px">
          <el-option :value="0" :label="$t('ui.waiting')" />
          <el-option :value="1" :label="$t('ui.approved')" />
          <el-option :value="2" :label="$t('ui.freeze')" />
        </el-select>
      </el-form-item>
      <el-button type="primary" @click="refresh">{{ $t('ui.query') }}</el-button>
      <el-button @click="exportCsv">{{ $t('ui.export_csv') }}</el-button>
    </el-form>
    <AdminState :error="error" :empty="!error && !(data?.list || []).length" />
    <el-table v-if="!error && (data?.list || []).length" :data="data?.list || []">
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="name" :label="$t('ui.name')" />
      <el-table-column prop="r_status" label="r_status" width="90" />
      <el-table-column prop="cityid" label="cityid" width="90" />
      <el-table-column :label="$t('ui.action')" width="220">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 2)">{{ $t('ui.freeze') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
    <el-pagination
      v-if="(data?.total || 0) > 20"
      style="margin-top: 12px"
      layout="prev, pager, next"
      :page-size="20"
      :current-page="page"
      :total="data?.total || 0"
      @current-change="
        (p: number) => {
          page = p
          refresh()
        }
      "
    />
  </div>
</template>
