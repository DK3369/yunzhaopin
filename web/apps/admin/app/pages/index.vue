<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('admin-overview', () =>
  api.post<Record<string, number>>('/v1/admin/dashboard/overview', {}),
)
const { data: recent } = await useAsyncData('admin-recent', () =>
  api.post<Array<{ uid: number; username: string; usertype_n: string; status_n: string; reg_date_n: string }>>(
    '/v1/admin/dashboard/recent-signups',
    { limit: 10 },
  ),
)
const labels: Record<string, string> = {
  pending_company_certs: '待审认证',
  pending_jobs: '待审职位',
  pending_reports: '待处理举报',
  pending_feedback: '待处理反馈',
  total_users: '用户总数',
  active_companies: '在营企业',
  active_jobs: '在招职位',
  active_resumes: '有效简历',
  today_new_jobs: '今日新职位',
  today_new_resumes: '今日新简历',
}
</script>

<template>
  <div>
    <h1>仪表盘</h1>
    <el-row :gutter="16">
      <el-col :span="6" v-for="(v, k) in data || {}" :key="k" style="margin-bottom: 12px">
        <el-card>
          <div class="muted">{{ labels[k] || k }}</div>
          <strong>{{ v }}</strong>
        </el-card>
      </el-col>
    </el-row>
    <h2 style="margin-top: 24px">最近注册</h2>
    <el-table :data="recent || []">
      <el-table-column prop="uid" label="UID" width="90" />
      <el-table-column prop="username" label="用户名" />
      <el-table-column prop="usertype_n" label="类型" width="120" />
      <el-table-column prop="status_n" label="状态" width="120" />
      <el-table-column prop="reg_date_n" label="注册时间" />
    </el-table>
  </div>
</template>

<style scoped>
.muted {
  color: #909399;
  font-size: 12px;
}
</style>
