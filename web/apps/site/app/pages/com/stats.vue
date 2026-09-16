<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Metric = { num?: number; jzr?: number }

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-dashboard-full', () =>
  api.post('/v1/mcenter/com-dashboard/full', {}),
)
const today = computed(
  () =>
    (data.value as {
      today?: {
        look_resume?: Metric
        look_job?: Metric
        down_resume?: Metric
        apply?: Metric
        invite?: Metric
      }
    } | null)?.today || null,
)
const year = computed(
  () => (data.value as { year_report?: Record<string, unknown> } | null)?.year_report || null,
)

function jzrText(n?: number) {
  const v = Number(n || 0)
  const sign = v > 0 ? '+' : ''
  return `${t('member_com_00373')} ${sign}${v}`
}

const yearMap = computed(() => {
  const raw = year.value
  if (!raw || typeof raw !== 'object' || Array.isArray(raw)) return [] as Array<{ k: string; v: string }>
  return Object.entries(raw as Record<string, unknown>).map(([k, v]) => ({
    k,
    v: typeof v === 'object' ? JSON.stringify(v) : String(v ?? ''),
  }))
})

useSeoMeta({ title: t('admin_tool_00224') })
</script>

<template>
  <MemberPanel :title="$t('admin_tool_00224')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <div v-if="today" class="membSubGuaTwo site-pc">
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
      <div v-if="today" class="site-h5">
        <div class="com_cardlist">
          <div class="com_cardlist_tit">{{ $t('member_com_00371') }}</div>
          <div class="com_cardlist_p">{{ today.look_resume?.num ?? 0 }} {{ jzrText(today.look_resume?.jzr) }}</div>
        </div>
        <div class="com_cardlist">
          <div class="com_cardlist_tit">{{ $t('member_com_00372') }}</div>
          <div class="com_cardlist_p">{{ today.look_job?.num ?? 0 }} {{ jzrText(today.look_job?.jzr) }}</div>
        </div>
        <div class="com_cardlist">
          <div class="com_cardlist_tit">{{ $t('wap_00451') }}</div>
          <div class="com_cardlist_p">{{ today.down_resume?.num ?? 0 }} {{ jzrText(today.down_resume?.jzr) }}</div>
        </div>
        <div class="com_cardlist">
          <div class="com_cardlist_tit">{{ $t('wap_com_00235') }}</div>
          <div class="com_cardlist_p">{{ today.apply?.num ?? 0 }} {{ jzrText(today.apply?.jzr) }}</div>
        </div>
        <div class="com_cardlist">
          <div class="com_cardlist_tit">{{ $t('wap_user_00216') }}</div>
          <div class="com_cardlist_p">{{ today.invite?.num ?? 0 }} {{ jzrText(today.invite?.jzr) }}</div>
        </div>
      </div>
      <ul class="stack">
        <li>{{ $t('ui.recv_resume_n') }} {{ data?.applies_received ?? 0 }}（{{ $t('ui.unread') }} {{ data?.applies_unread ?? 0 }}）</li>
        <li>{{ $t('ui.sent_interview') }} {{ data?.interviews_sent ?? 0 }}</li>
        <li>{{ $t('ui.dl_resume') }} {{ data?.resume_downloads ?? 0 }}</li>
        <li>{{ $t('wap_user_00008') }} {{ data?.integral_balance ?? 0 }}</li>
      </ul>
      <div v-if="yearMap.length" class="membSubGuaTwo site-pc">
        <ul>
          <li v-for="row in yearMap" :key="row.k">
            <div class="twoDivTite"><span>{{ row.k }}</span></div>
            <div class="twoDivNum"><span>{{ row.v }}</span></div>
          </li>
        </ul>
      </div>
      <div v-if="yearMap.length" class="site-h5">
        <div v-for="row in yearMap" :key="'h5y-' + row.k" class="com_cardlist">
          <div class="com_cardlist_tit">{{ row.k }}</div>
          <div class="com_cardlist_p">{{ row.v }}</div>
        </div>
      </div>
    </template>
  </MemberPanel>
</template>
