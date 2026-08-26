<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data } = await useAsyncData('admin-overview', () =>
  api.post<Record<string, number>>('/v1/admin/dashboard/overview', {}),
)
const { data: recent } = await useAsyncData('admin-recent', () =>
  api.post<Array<{ uid: number; username: string; usertype_n: string; status_n: string; reg_date_n: string }>>(
    '/v1/admin/dashboard/recent-signups',
    { limit: 10 },
  ),
)
const labels = computed<Record<string, string>>(() => ({
  pending_company_certs: t('ui.certs'),
  pending_jobs: t('ui.jobs_audit'),
  pending_reports: t('ui.reports'),
  pending_feedback: t('ui.feedback'),
  total_users: t('ui.users'),
  active_companies: t('common.company'),
  active_jobs: t('common.job'),
  active_resumes: t('common.resume'),
  today_new_jobs: t('home.latest_jobs'),
  today_new_resumes: t('home.latest_resumes'),
}))
</script>

<template>
  <div>
    <h1>{{ $t('ui.dashboard') }}</h1>
    <el-row :gutter="16">
      <el-col v-for="(v, k) in data || {}" :key="k" :span="6" style="margin-bottom: 12px">
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
