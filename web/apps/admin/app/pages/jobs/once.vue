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
    <h1>店铺招聘审核</h1>
    <p>PHP status：1 通过 / 0 待审 / 2 过期</p>
    <el-radio-group v-model="status" style="margin-bottom: 12px">
      <el-radio-button :value="0">待审</el-radio-button>
      <el-radio-button :value="1">已通过</el-radio-button>
      <el-radio-button :value="2">过期</el-radio-button>
    </el-radio-group>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="companyname" label="企业" />
      <el-table-column prop="linkman" label="联系人" />
      <el-table-column prop="status" label="status" width="80" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="review(row, 1)">通过</el-button>
          <el-button size="small" @click="review(row, 0)">待审</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
