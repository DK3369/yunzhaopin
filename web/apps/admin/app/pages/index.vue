<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('admin-overview', () =>
  api.post('/v1/admin/dashboard/overview', {}),
)
</script>

<template>
  <div>
    <h1>仪表盘</h1>
    <el-row :gutter="16">
      <el-col :span="6" v-for="(v, k) in data || {}" :key="k">
        <el-card>
          <div class="muted">{{ k }}</div>
          <strong>{{ v }}</strong>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>
