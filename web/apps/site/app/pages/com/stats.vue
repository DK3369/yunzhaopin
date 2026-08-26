<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('com-dashboard', () => api.post('/v1/mcenter/com-dashboard', {}))
const { data: year } = await useAsyncData('com-year', () => api.post('/v1/mcenter/dashboard/year-report', {}))
useSeoMeta({ title: '企业统计' })
</script>

<template>
  <section>
    <h1>企业统计</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <template v-else>
      <ul class="stack">
        <li>收到简历 {{ data?.applies_received ?? 0 }}（未读 {{ data?.applies_unread ?? 0 }}）</li>
        <li>已发面试 {{ data?.interviews_sent ?? 0 }}</li>
        <li>下载简历 {{ data?.resume_downloads ?? 0 }}</li>
        <li>积分 {{ data?.integral_balance ?? 0 }}</li>
      </ul>
      <h2>年报</h2>
      <pre>{{ JSON.stringify(year, null, 2) }}</pre>
    </template>
  </section>
</template>
