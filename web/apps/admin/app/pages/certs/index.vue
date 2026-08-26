<script setup lang="ts">
const api = useApi()
const page = ref(1)
const { data, error, refresh } = await useAsyncData(
  () => `admin-certs-${page.value}`,
  () => api.post<{ list: Array<Record<string, unknown>>; total: number }>('/v1/admin/company-certs', { page: page.value, page_size: 20 }),
)
async function review(row: { uid: number }, approve: boolean) {
  await api.post('/v1/admin/company-certs/review', { uid: row.uid, approve, note: '' })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.certs') }}</h1>
    <AdminState :error="error" :empty="!error && !(data?.list || []).length" />
    <el-table v-if="!error && (data?.list || []).length" :data="data?.list || []">
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="status_n" :label="$t('ui.status')" width="110" />
      <el-table-column prop="note" :label="$t('ui.note')" />
      <el-table-column prop="submitted_at_n" :label="$t('ui.time')" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="review(row, true)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" type="danger" @click="review(row, false)">{{ $t('ui.reject') }}</el-button>
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
