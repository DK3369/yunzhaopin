<script setup lang="ts">
const api = useApi()
const status = ref(0)
const { data, refresh } = await useAsyncData(
  () => `admin-once-${status.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/once-jobs', {
      page: 1,
      page_size: 20,
      status: status.value,
    }),
)
watch(status, () => refresh())
async function review(row: { id: number }, next: number) {
  await api.post('/v1/admin/once-jobs/status', { id: row.id, status: next })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.once_audit') }}</h1>
    <el-radio-group v-model="status" style="margin-bottom: 12px">
      <el-radio-button :value="0">{{ $t('ui.waiting') }}</el-radio-button>
      <el-radio-button :value="1">{{ $t('ui.passed') }}</el-radio-button>
      <el-radio-button :value="2">{{ $t('ui.expire') }}</el-radio-button>
    </el-radio-group>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="companyname" :label="$t('common.company')" />
      <el-table-column prop="linkman" :label="$t('ui.linkman')" />
      <el-table-column prop="status" label="status" width="80" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="review(row, 1)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" @click="review(row, 0)">{{ $t('ui.waiting') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
