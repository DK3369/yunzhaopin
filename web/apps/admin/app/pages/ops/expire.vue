<script setup lang="ts">
const api = useApi()
const expiredOnly = ref(true)
const { data, refresh } = await useAsyncData(
  () => `admin-expire-${expiredOnly.value}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/company-expire', {
      page: 1,
      page_size: 20,
      expired_only: expiredOnly.value,
    }),
)
watch(expiredOnly, () => refresh())
</script>

<template>
  <div>
    <h1>套餐到期</h1>
    <p>读 phpyun_company_statis.vip_etime，不改 JWT</p>
    <el-checkbox v-model="expiredOnly" style="margin-bottom: 12px">仅已过期</el-checkbox>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="name" label="企业" />
      <el-table-column prop="rating_name" label="套餐" />
      <el-table-column prop="vip_etime" label="vip_etime" width="120" />
    </el-table>
  </div>
</template>
