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
    <h1>{{ $t('ui.expire') }}</h1>
    <p>{{ $t('ui.expire_hint') }}</p>
    <el-checkbox v-model="expiredOnly" style="margin-bottom: 12px">{{ $t('ui.expired_only') }}</el-checkbox>
    <el-table :data="data?.list || []">
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="name" :label="$t('common.company')" />
      <el-table-column prop="rating_name" :label="$t('ui.package')" />
      <el-table-column prop="vip_etime" label="vip_etime" width="120" />
    </el-table>
  </div>
</template>
