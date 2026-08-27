<script setup lang="ts">
const api = useApi()
const rStatus = ref<number | undefined>()
const keyword = ref('')
const page = ref(1)
const ratingId = ref<number | undefined>()
const ratingUid = ref(0)
const showRating = ref(false)
const { data: ratings } = await useAsyncData('admin-company-ratings', () =>
  api.post<Array<{ id: number; name: string }>>('/v1/admin/companies/ratings', {}),
)
const { data, error, refresh } = await useAsyncData(
  () => `admin-companies-${page.value}-${rStatus.value ?? 'all'}-${keyword.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>>; total: number }>('/v1/admin/companies', {
      page: page.value,
      page_size: 20,
      r_status: rStatus.value,
      keyword: keyword.value || undefined,
    }),
)
watch([rStatus, keyword], () => {
  page.value = 1
  refresh()
})
async function setStatus(row: { uid: number }, r_status: number) {
  await api.post('/v1/admin/companies/status', { uid: row.uid, r_status })
  refresh()
}
async function impersonate(row: { uid: number }) {
  const r = await api.post<{ access_token: string }>('/v1/admin/users/impersonate', { uid: row.uid })
  await navigator.clipboard?.writeText(r.access_token).catch(() => undefined)
}
async function openRating(row: { uid: number; rating: number }) {
  ratingUid.value = row.uid
  ratingId.value = Number(row.rating) || undefined
  showRating.value = true
}
async function saveRating() {
  if (!ratingUid.value || !ratingId.value) return
  await api.post('/v1/admin/companies/rating', { uid: ratingUid.value, rating: ratingId.value })
  showRating.value = false
  ratingUid.value = 0
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
      <el-table-column prop="rating_name" :label="$t('ui.category')" />
      <el-table-column prop="yyzz_status" label="yyzz" width="80" />
      <el-table-column prop="hits" label="hits" width="80" />
      <el-table-column prop="login_date" label="login_date" width="120" />
      <el-table-column :label="$t('ui.status')" width="110">
        <template #default="{ row }">
          {{
            Number(row.r_status) === 1
              ? $t('ui.approved')
              : Number(row.r_status) === 2
                ? $t('ui.freeze')
                : $t('ui.waiting')
          }}
        </template>
      </el-table-column>
      <el-table-column :label="$t('ui.action')" width="360">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 2)">{{ $t('ui.freeze') }}</el-button>
          <el-button size="small" type="primary" @click="impersonate(row)">{{ $t('ui.impersonate') }}</el-button>
          <el-button size="small" @click="openRating(row)">{{ $t('ui.rating') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
    <el-pagination
      v-if="(data?.total || 0) > 20"
      style="margin-top: 12px"
      layout="total, prev, pager, next"
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
    <el-dialog v-model="showRating" :title="$t('ui.rating')" width="360px">
      <el-select v-model="ratingId" style="width: 100%">
        <el-option v-for="r in ratings || []" :key="r.id" :value="r.id" :label="r.name" />
      </el-select>
      <template #footer>
        <el-button type="primary" @click="saveRating">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>
