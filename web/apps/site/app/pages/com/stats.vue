<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Metric = { num?: number; jzr?: number }

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-dashboard', () => api.post('/v1/mcenter/com-dashboard', {}))
const { data: today } = await useAsyncData('com-stats-today', () =>
  api
    .post<{
      look_resume?: Metric
      look_job?: Metric
      down_resume?: Metric
      apply?: Metric
      invite?: Metric
    }>('/v1/mcenter/com-stats/today', {})
    .catch(() => null),
)
const { data: year } = await useAsyncData('com-year', () => api.post('/v1/mcenter/dashboard/year-report', {}))

function jzrText(n?: number) {
  const v = Number(n || 0)
  const sign = v > 0 ? '+' : ''
  return `${t('member_com_00373')} ${sign}${v}`
}

useSeoMeta({ title: t('admin_tool_00224') })
</script>

<template>
  <section>
    <h1>{{ $t('admin_tool_00224') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <div v-if="today" class="stack">
        <article class="job-card">
          <h2>{{ $t('member_com_00371') }}</h2>
          <p>{{ today.look_resume?.num ?? 0 }} · {{ jzrText(today.look_resume?.jzr) }}</p>
        </article>
        <article class="job-card">
          <h2>{{ $t('member_com_00372') }}</h2>
          <p>{{ today.look_job?.num ?? 0 }} · {{ jzrText(today.look_job?.jzr) }}</p>
        </article>
        <article class="job-card">
          <h2>{{ $t('wap_00451') }}</h2>
          <p>{{ today.down_resume?.num ?? 0 }} · {{ jzrText(today.down_resume?.jzr) }}</p>
        </article>
        <article class="job-card">
          <h2>{{ $t('wap_com_00235') }}</h2>
          <p>{{ today.apply?.num ?? 0 }} · {{ jzrText(today.apply?.jzr) }}</p>
        </article>
        <article class="job-card">
          <h2>{{ $t('wap_user_00216') }}</h2>
          <p>{{ today.invite?.num ?? 0 }} · {{ jzrText(today.invite?.jzr) }}</p>
        </article>
      </div>
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
