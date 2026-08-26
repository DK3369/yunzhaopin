<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-dashboard', () => api.post('/v1/mcenter/com-dashboard', {}))
const { data: year } = await useAsyncData('com-year', () => api.post('/v1/mcenter/dashboard/year-report', {}))
useSeoMeta({ title: t('ui.stats') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.stats') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.please_login_com') }}</p>
    <template v-else>
      <ul class="stack">
        <li>{{ $t('ui.recv_resume_n') }} {{ data?.applies_received ?? 0 }}（{{ $t('ui.unread') }} {{ data?.applies_unread ?? 0 }}）</li>
        <li>{{ $t('ui.sent_interview') }} {{ data?.interviews_sent ?? 0 }}</li>
        <li>{{ $t('ui.dl_resume') }} {{ data?.resume_downloads ?? 0 }}</li>
        <li>{{ $t('ui.integral') }} {{ data?.integral_balance ?? 0 }}</li>
      </ul>
      <h2>{{ $t('ui.year_report') }}</h2>
      <pre>{{ JSON.stringify(year, null, 2) }}</pre>
    </template>
  </section>
</template>
