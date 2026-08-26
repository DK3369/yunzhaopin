<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('admin-cron', () =>
  api.post<Array<{ name: string; schedule: string; kind: string }>>('/v1/admin/cron', {}),
)
useSeoMeta({ title: t('ui.cron') })
</script>

<template>
  <div>
    <h1>{{ $t('ui.cron') }}</h1>
    <AdminState :error="error" :empty="!error && !(data || []).length" />
    <el-table v-if="!error && (data || []).length" :data="data || []">
      <el-table-column prop="name" :label="$t('ui.name')" />
      <el-table-column prop="schedule" :label="$t('ui.schedule')" />
      <el-table-column prop="kind" :label="$t('ui.kind')" width="120" />
    </el-table>
  </div>
</template>
