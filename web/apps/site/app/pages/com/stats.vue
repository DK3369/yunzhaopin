<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-dashboard', () => api.post('/v1/mcenter/com-dashboard', {}))
const { data: year } = await useAsyncData('com-year', () => api.post('/v1/mcenter/dashboard/year-report', {}))
useSeoMeta({ title: t('admin_tool_00224') })
</script>

<template>
  <section>
    <h1>{{ $t('admin_tool_00224') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <ul class="stack">
        <li>{{ $t('ui.recv_resume_n') }} {{ data?.applies_received ?? 0 }}（{{ $t('ui.unread') }} {{ data?.applies_unread ?? 0 }}）</li>
        <li>{{ $t('ui.sent_interview') }} {{ data?.interviews_sent ?? 0 }}</li>
        <li>{{ $t('ui.dl_resume') }} {{ data?.resume_downloads ?? 0 }}</li>
        <li>{{ $t('wap_user_00008') }} {{ data?.integral_balance ?? 0 }}</li>
      </ul>
      <h2>{{ $t('ui.year_report') }}</h2>
      <pre>{{ JSON.stringify(year, null, 2) }}</pre>
    </template>
  </section>
</template>
