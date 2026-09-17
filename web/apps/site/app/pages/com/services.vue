<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'

type Caps = {
  job_num?: number
  resume?: number
  interview?: number
  breakjob_num?: number
  top_num?: number
  urgent_num?: number
  rec_num?: number
  zph_num?: number
  sons_num?: number
}
type Vip = {
  rating_name?: string
  rating_type?: number
  started_at?: number
  expires_at?: number
  job_num?: number
  down_resume?: number
  invite_resume?: number
  breakjob_num?: number
  zph_num?: number
  top_num?: number
  urgent_num?: number
  rec_num?: number
  sons_num?: number
  caps?: Caps
}

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-vip-current', () =>
  api.post<Vip>('/v1/mcenter/vip/current', {}),
)

function capOf(v: number | string | undefined) {
  if (data.value?.rating_type === 2) return '-'
  return String(v ?? 0)
}
function bar(left: number, cap: number | string | undefined) {
  const c = Number(cap || 0)
  if (c <= 0) return 0
  return Math.min(100, Math.round((Number(left || 0) / c) * 100))
}

const rows = computed(() => {
  const v = data.value || {}
  const c = v.caps || {}
  return [
    { title: t('wap_com_00106'), left: v.job_num || 0, cap: c.job_num },
    { title: t('wap_00451'), left: v.down_resume || 0, cap: c.resume },
    { title: t('wap_user_00216'), left: v.invite_resume || 0, cap: c.interview },
    { title: t('wap_com_00029'), left: v.breakjob_num || 0, cap: c.breakjob_num },
    { title: t('wap_com_00238'), left: v.top_num || 0, cap: c.top_num },
    { title: t('member_com_00613'), left: v.urgent_num || 0, cap: c.urgent_num },
    { title: t('wap_com_00237'), left: v.rec_num || 0, cap: c.rec_num },
    { title: t('member_com_00293'), left: v.zph_num || 0, cap: c.zph_num },
    { title: t('common_01597'), left: v.sons_num || 0, cap: c.sons_num },
  ]
})

useSeoMeta({ title: t('wap_com_00064') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00064')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <div class="com_cardlist">
        <div class="com_cardlist_tit">{{ data?.rating_name || $t('wap_com_00097') }}</div>
        <div class="com_cardlist_p">
          {{ formatUnixDate(data?.started_at) }} — {{ formatUnixDate(data?.expires_at) }}
        </div>
      </div>
      <div v-for="row in rows" :key="row.title" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.title }} {{ row.left }} / {{ capOf(row.cap) }}</div>
        <div class="com_cardlist_p">
          <div class="pack-bar"><span :style="{ width: `${bar(row.left, row.cap)}%` }" /></div>
        </div>
      </div>
    </template>
  </MemberPanel>
</template>

<style scoped>
.pack-bar {
  height: 8px;
  background: #eee;
  border-radius: 4px;
  overflow: hidden;
}
.pack-bar span {
  display: block;
  height: 100%;
  background: #2e8ded;
}
</style>
