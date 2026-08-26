<script setup lang="ts">
const api = useApi()
const status = ref<number | undefined>(0)
const { data, refresh } = await useAsyncData(
  () => `admin-questions-${status.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/questions', {
      page: 1,
      page_size: 20,
      status: status.value,
    }),
)
watch(status, () => refresh())
async function setState(row: { id: number }, state: number) {
  await api.post('/v1/admin/questions/state', { id: row.id, state })
  refresh()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/questions/delete', { id: row.id })
  refresh()
}
</script>

<template>
  <div>
    <h1>问答审核</h1>
    <p>PHP POST status 对应列 state：0 待审 / 1 通过 / 2 删除</p>
    <el-radio-group v-model="status" style="margin-bottom: 12px">
      <el-radio-button :value="0">待审</el-radio-button>
      <el-radio-button :value="1">已通过</el-radio-button>
      <el-radio-button :value="2">已删</el-radio-button>
    </el-radio-group>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" label="标题" />
      <el-table-column prop="status" label="state" width="80" />
      <el-table-column label="操作" width="220">
        <template #default="{ row }">
          <el-button size="small" type="primary" @click="setState(row, 1)">通过</el-button>
          <el-button size="small" @click="setState(row, 0)">待审</el-button>
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
