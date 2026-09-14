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
  <MemberPanel :title="$t('admin_tool_00224')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <div v-if="today" class="membSubGuaTwo">
        <ul>
          <li>
            <div class="twoDivTite"><span>{{ $t('member_com_00371') }}</span></div>
            <div class="twoDivNum"><span>{{ today.look_resume?.num ?? 0 }}</span><b>{{ jzrText(today.look_resume?.jzr) }}</b></div>
          </li>
          <li>
            <div class="twoDivTite"><span>{{ $t('member_com_00372') }}</span></div>
            <div class="twoDivNum"><span>{{ today.look_job?.num ?? 0 }}</span><b>{{ jzrText(today.look_job?.jzr) }}</b></div>
          </li>
          <li>
            <div class="twoDivTite"><span>{{ $t('wap_00451') }}</span></div>
            <div class="twoDivNum"><span>{{ today.down_resume?.num ?? 0 }}</span><b>{{ jzrText(today.down_resume?.jzr) }}</b></div>
          </li>
          <li>
            <div class="twoDivTite"><span>{{ $t('wap_com_00235') }}</span></div>
            <div class="twoDivNum"><span>{{ today.apply?.num ?? 0 }}</span><b>{{ jzrText(today.apply?.jzr) }}</b></div>
          </li>
          <li>
            <div class="twoDivTite"><span>{{ $t('wap_user_00216') }}</span></div>
            <div class="twoDivNum"><span>{{ today.invite?.num ?? 0 }}</span><b>{{ jzrText(today.invite?.jzr) }}</b></div>
          </li>
        </ul>
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
  </MemberPanel>
</template>
