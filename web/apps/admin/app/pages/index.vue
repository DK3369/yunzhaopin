<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('admin-msg-num', () =>
  api.post<Record<string, number>>('/v1/admin/dashboard/msg-num', {}),
)
const { data: recent } = await useAsyncData('admin-recent', () =>
  api.post<Array<{ uid: number; username: string; usertype_n: string; status_n: string; reg_date_n: string }>>(
    '/v1/admin/dashboard/recent-signups',
    { limit: 10 },
  ),
)
const labels: Record<string, string> = {
  msg_num: t('ui.dashboard'),
  company: t('ui.companies'),
  company_job: t('ui.jobs_audit'),
  partjob: t('ui.parts_audit'),
  company_cert: t('ui.certs'),
  resume_expect: t('ui.resume_audit'),
  once_job: t('ui.once_audit'),
  tiny: t('ui.tiny_audit'),
  ask: t('ui.questions'),
  order: t('ui.orders'),
  reportjob: t('ui.reports'),
  reportresume: t('ui.reports'),
  redeem: t('ui.redeem_orders'),
  logout: t('ui.accounts'),
  warning: t('ui.warnings'),
  link_num: t('ui.links'),
}
const cards = computed(() => {
  const raw = data.value || {}
  return Object.entries(raw).filter(([k, v]) => k !== 'msg_num' && Number(v) > 0)
})
</script>

<template>
  <div>
    <h1>{{ $t('ui.dashboard') }}</h1>
    <AdminState :error="error" :empty="false" />
    <p v-if="data" class="muted">{{ labels.msg_num }}: {{ data.msg_num || 0 }}</p>
    <el-row :gutter="16">
      <el-col v-for="[k, v] in cards" :key="k" :span="6" style="margin-bottom: 12px">
        <el-card>
          <div class="muted">{{ labels[k] || k }}</div>
          <strong>{{ v }}</strong>
        </el-card>
      </el-col>
    </el-row>
    <h2 style="margin-top: 24px">{{ $t('ui.recent') }}</h2>
    <el-table :data="recent || []">
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="username" :label="$t('ui.username')" />
      <el-table-column prop="usertype_n" :label="$t('ui.type')" width="120" />
      <el-table-column prop="status_n" :label="$t('ui.status')" width="120" />
      <el-table-column prop="reg_date_n" :label="$t('common.register')" />
    </el-table>
  </div>
</template>

<style scoped>
.muted {
  color: #909399;
  font-size: 12px;
}
</style>
