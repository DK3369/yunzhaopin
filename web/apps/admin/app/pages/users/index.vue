<script setup lang="ts">
const api = useApi()
const keyword = ref('')
const usertype = ref<number | undefined>()
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
</script>

<template>
  <div>
    <h1>用户管理</h1>
    <el-form inline @submit.prevent="search">
      <el-form-item>
        <el-input v-model="keyword" placeholder="用户名/手机/邮箱" clearable />
      </el-form-item>
      <el-form-item>
        <el-select v-model="usertype" placeholder="类型" clearable style="width: 140px">
          <el-option :value="1" label="求职者" />
          <el-option :value="2" label="企业" />
          <el-option :value="3" label="管理员" />
        </el-select>
      </el-form-item>
      <el-button type="primary" @click="search">查询</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="username" label="用户名" />
      <el-table-column prop="usertype_n" label="类型" width="110" />
      <el-table-column prop="status_n" label="状态" width="110" />
      <el-table-column prop="moblie" label="手机" />
      <el-table-column label="操作" width="200">
        <template #default="{ row }">
          <el-button size="small" @click="setStatus(row, 1)">解冻</el-button>
          <el-button size="small" type="danger" @click="setStatus(row, 0)">冻结</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
