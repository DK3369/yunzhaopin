<script setup lang="ts">
const api = useApi()
const state = ref(0)
const { data, refresh } = await useAsyncData(
  () => `admin-parts-${state.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/parts', {
      page: 1,
      page_size: 20,
      state: state.value,
    }),
)
watch(state, () => refresh())
async function review(row: { id: number }, next: number) {
  await api.post('/v1/admin/parts/state', { pid: row.id, status: next, statusbody: '' })
  refresh()
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.parts_audit') }}</h1>
    <p>{{ $t('ui.parts_php_hint') }}</p>
    <el-radio-group v-model="state" style="margin-bottom: 12px">
      <el-radio-button :value="0">{{ $t('ui.waiting') }}</el-radio-button>
      <el-radio-button :value="1">{{ $t('ui.passed') }}</el-radio-button>
      <el-radio-button :value="2">{{ $t('ui.rejected') }}</el-radio-button>
    </el-radio-group>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" :label="$t('common.job')" />
      <el-table-column prop="com_name" :label="$t('common.company')" />
      <el-table-column prop="state" label="state" width="80" />
      <el-table-column :label="$t('ui.action')" width="200">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="review(row, 1)">{{ $t('ui.approved') }}</el-button>
          <el-button size="small" type="danger" @click="review(row, 2)">{{ $t('ui.reject') }}</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
