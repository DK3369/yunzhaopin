<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'

type Current = {
  active: boolean
  package_code?: string | null
  started_at?: number | null
  expires_at?: number | null
}
type Pack = {
  id: number
  code: string
  name: string
  duration_days: number
  price_yuan: number
  desc?: unknown
}

const api = useApi()
const { t } = useI18n()

const { data: current, error } = await useAsyncData('com-vip-current', () =>
  api.post<Current>('/v1/mcenter/vip/current', {}),
)
const { data: packs } = await useAsyncData('com-vip-packages', () =>
  api.post<Pack[]>('/v1/mcenter/vip/packages', {}).catch(() => [] as Pack[]),
)

const packList = computed<Pack[]>(() => (Array.isArray(packs.value) ? packs.value : []))

function descLines(desc: unknown): string[] {
  if (!desc) return []
  if (Array.isArray(desc)) return desc.map((d) => String(d))
  if (typeof desc === 'string') return [desc]
  if (typeof desc === 'object') {
    return Object.entries(desc as Record<string, unknown>).map(([k, v]) => `${k}: ${v}`)
  }
  return []
}

useSeoMeta({ title: t('wap_com_00097') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00097')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <div class="com_vip_box">
        <h2>{{ $t('wap_01229') }}</h2>
        <template v-if="current?.active">
          <p>{{ $t('wap_00025') }}: {{ current.package_code }}</p>
          <p>
            {{ $t('member_com_00315') }}:
            {{ formatUnixDate(current.started_at) }} ~ {{ formatUnixDate(current.expires_at) }}
          </p>
        </template>
        <template v-else>
          <p class="muted">{{ current?.expires_at ? $t('wap_com_00319') : $t('ui.no_data') }}</p>
          <p v-if="current?.expires_at" class="muted">
            {{ $t('wap_01394') }}{{ formatUnixDate(current.expires_at) }}
          </p>
        </template>
        <p>
          <NuxtLink to="/com/pay" class="com_topbth">{{ $t('member_com_00041') }}</NuxtLink>
          <NuxtLink to="/com/added" class="com_topbth">{{ $t('wap_com_00393') }}</NuxtLink>
        </p>
      </div>
      <MemberResumeH1 :title="$t('member_com_00610')" />
      <div class="payment_list site-pc">
        <div v-for="p in packList" :key="p.id" class="payment_list_text">
          <div class="payment_list_text_n">
            {{ p.name }}
            <em class="payment_list_text_dw">¥{{ p.price_yuan }} / {{ p.duration_days }}d</em>
          </div>
          <ul v-if="descLines(p.desc).length">
            <li v-for="(line, i) in descLines(p.desc)" :key="i">{{ line }}</li>
          </ul>
        </div>
      </div>
      <div class="site-h5 issue_post_body">
        <div v-for="p in packList" :key="'h5-' + p.id" class="issue_post_body_card">
          <div class="Posted_card_top">
            <div class="Posted_card_name">{{ p.name }}</div>
            <div class="Posted_card_pay">¥{{ p.price_yuan }} / {{ p.duration_days }}d</div>
          </div>
          <ul v-if="descLines(p.desc).length">
            <li v-for="(line, i) in descLines(p.desc)" :key="i">{{ line }}</li>
          </ul>
        </div>
      </div>
    </template>
    <p>
      <NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink>
    </p>
  </MemberPanel>
</template>
