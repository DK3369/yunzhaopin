<script setup lang="ts">
const api = useApi()
const state = ref(0)
const page = ref(1)
const selected = ref<number[]>([])
const { data, error, refresh } = await useAsyncData(
  () => `admin-jobs-${state.value}-${page.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>>; total: number }>('/v1/admin/jobs', {
      page: page.value,
      page_size: 20,
      state: state.value,
    }),
)
watch([state, page], () => refresh())
function onSelect(rows: Array<{ id: number }>) {
  selected.value = rows.map((r) => r.id)
}
async function review(row: { id: number }, next: number) {
  await api.post('/v1/admin/jobs/state', { id: row.id, state: next })
  refresh()
}
async function batch(next: number) {
  if (!selected.value.length) return
  await api.post('/v1/admin/jobs/batch/state', { ids: selected.value, state: next })
  selected.value = []
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.jobs_audit') }}</h1>
    <el-radio-group v-model="state" style="margin-bottom: 12px" @change="page = 1">
      <el-radio-button :value="0">{{ $t('ui.waiting') }}</el-radio-button>
      <el-radio-button :value="1">{{ $t('ui.passed') }}</el-radio-button>
      <el-radio-button :value="2">{{ $t('ui.rejected') }}</el-radio-button>
    </el-radio-group>
    <div style="margin-bottom: 12px">
      <el-button size="small" type="primary" @click="batch(1)">{{ $t('ui.batch_approve') }}</el-button>
      <el-button size="small" type="danger" @click="batch(2)">{{ $t('ui.batch_reject') }}</el-button>
    </div>
    <AdminState :error="error" :empty="!error && !(data?.list || []).length" />
    <el-table v-if="!error && (data?.list || []).length" :data="data?.list || []" @selection-change="onSelect">
      <el-table-column type="selection" width="48" />
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" :label="$t('common.job')" />
      <el-table-column prop="com_name" :label="$t('common.company')" />
      <el-table-column prop="edu_n" :label="$t('home.education_suffix')" width="110" />
      <el-table-column prop="lastupdate" :label="$t('ui.time')" width="120" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="review(row, 1)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" type="danger" @click="review(row, 2)">{{ $t('ui.reject') }}</el-button>
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
      @current-change="(p: number) => (page = p)"
    />
  </div>
</template>
