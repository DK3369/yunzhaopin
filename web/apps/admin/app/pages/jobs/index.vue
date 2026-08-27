<script setup lang="ts">
const api = useApi()
const state = ref<number | undefined>(0)
const status = ref<number | undefined>()
const jtype = ref<string | undefined>()
const keyword = ref('')
const jobClass = ref<number | undefined>()
const cityClass = ref<number | undefined>()
const page = ref(1)
const selected = ref<number[]>([])
const days = ref(7)
const { data: stats } = await useAsyncData('admin-job-stats', () =>
  api.post<Record<string, number>>('/v1/admin/jobs/stats', {}),
)
const { data, error, refresh } = await useAsyncData(
  () =>
    `admin-jobs-${state.value}-${status.value}-${jtype.value}-${page.value}-${keyword.value}-${jobClass.value}-${cityClass.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>>; total: number; perPage?: number }>('/v1/admin/jobs', {
      page: page.value,
      page_size: 20,
      state: state.value,
      status: status.value,
      jtype: jtype.value,
      keyword: keyword.value || undefined,
      job_class: jobClass.value,
      city_class: cityClass.value,
      type: 1,
    }),
)
watch([state, status, jtype, page, jobClass, cityClass], () => refresh())
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
async function publish(row: { id: number; status: number }) {
  await api.post('/v1/admin/jobs/publish', { id: row.id, status: Number(row.status) === 0 ? 1 : 0 })
  refresh()
}
async function promote(kind: string, on: boolean) {
  const ids = selected.value
  if (!ids.length) return
  await api.post('/v1/admin/jobs/promote', { ids, kind, on, days: days.value })
  refresh()
}
async function refreshJobs() {
  if (!selected.value.length) return
  await api.post('/v1/admin/jobs/refresh', { ids: selected.value })
  refresh()
}
async function remove() {
  if (!selected.value.length) return
  await api.post('/v1/admin/jobs/delete', { ids: selected.value })
  selected.value = []
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.jobs_audit') }}</h1>
    <p v-if="stats" class="muted">
      {{ $t('ui.waiting') }} {{ stats.dsh || 0 }} · {{ $t('ui.rejected') }} {{ stats.wtg || 0 }} ·
      {{ $t('ui.offline') || 'offline' }} {{ stats.xj || 0 }}
    </p>
    <el-form inline @submit.prevent="refresh">
      <el-form-item>
        <el-input v-model="keyword" :placeholder="$t('common.job')" clearable />
      </el-form-item>
      <el-form-item>
        <el-select v-model="state" clearable style="width: 140px" :placeholder="$t('ui.status')">
          <el-option :value="0" :label="$t('ui.waiting')" />
          <el-option :value="1" :label="$t('ui.passed')" />
          <el-option :value="3" :label="$t('ui.rejected')" />
          <el-option :value="2" :label="$t('ui.freeze')" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-select v-model="status" clearable style="width: 140px" :placeholder="$t('ui.open')">
          <el-option :value="1" :label="$t('ui.open_on')" />
          <el-option :value="2" :label="$t('common.close')" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-select v-model="jtype" clearable style="width: 140px">
          <el-option value="urgent" :label="$t('common.hot')" />
          <el-option value="rec" :label="$t('common.recommended')" />
          <el-option value="xuanshang" label="top" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-input-number v-model="jobClass" :min="0" controls-position="right" placeholder="job_class" />
      </el-form-item>
      <el-form-item>
        <el-input-number v-model="cityClass" :min="0" controls-position="right" placeholder="city_class" />
      </el-form-item>
      <el-button type="primary" @click="refresh">{{ $t('ui.query') }}</el-button>
    </el-form>
    <div style="margin-bottom: 12px">
      <el-input-number v-model="days" :min="1" :max="365" size="small" />
      <el-button size="small" type="primary" @click="batch(1)">{{ $t('ui.batch_approve') }}</el-button>
      <el-button size="small" type="danger" @click="batch(3)">{{ $t('ui.batch_reject') }}</el-button>
      <el-button size="small" @click="promote('rec', true)">{{ $t('common.recommended') }}</el-button>
      <el-button size="small" @click="promote('urgent', true)">urgent</el-button>
      <el-button size="small" @click="promote('xuanshang', true)">top</el-button>
      <el-button size="small" @click="refreshJobs">{{ $t('ui.refresh') }}</el-button>
      <el-button size="small" type="danger" @click="remove">{{ $t('common.delete') }}</el-button>
    </div>
    <AdminState :error="error" :empty="!error && !(data?.list || []).length" />
    <el-table v-if="!error && (data?.list || []).length" :data="data?.list || []" @selection-change="onSelect">
      <el-table-column type="selection" width="48" />
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" :label="$t('common.job')" />
      <el-table-column prop="com_name" :label="$t('common.company')" />
      <el-table-column prop="snum" label="snum" width="70" />
      <el-table-column prop="jobhits" label="hits" width="70" />
      <el-table-column prop="jobexpoure" label="expoure" width="80" />
      <el-table-column prop="edu_n" :label="$t('home.education_suffix')" width="90" />
      <el-table-column prop="sdate_n" :label="$t('ui.time')" width="140" />
      <el-table-column prop="lastupdate_n_n" width="140" />
      <el-table-column prop="xsdate" label="xsdate" width="90" />
      <el-table-column prop="isrec" label="isrec" width="70" />
      <el-table-column :label="$t('ui.status')" width="90">
        <template #default="{ row }">{{ row.state }} / {{ row.r_status }}</template>
      </el-table-column>
      <el-table-column :label="$t('ui.action')" width="280">
        <template #default="{ row }">
          <el-button size="small" @click="publish(row)">{{ $t('ui.open') }}</el-button>
          <el-button size="small" type="primary" @click="review(row, 1)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" type="danger" @click="review(row, 3)">{{ $t('ui.reject') }}</el-button>
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
      @current-change="(p: number) => (page = p)"
    />
  </div>
</template>

<style scoped>
.muted {
  color: #909399;
  font-size: 12px;
}
</style>
