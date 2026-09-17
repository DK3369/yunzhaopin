<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'

type Current = {
  active: boolean
  package_code?: string | null
  started_at?: number | null
  expires_at?: number | null
  rating?: number
  rating_name?: string
  job_num?: number
  breakjob_num?: number
  down_resume?: number
  invite_resume?: number
  integral?: number
}
type Pack = {
  id: number
  code: string
  name: string
  duration_days: number
  price_yuan: number
  desc?: unknown
}
type Quote = {
  style: number
  price: number
  user_integral: number
}

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()

const { data: current, error, refresh: refreshCurrent } = await useAsyncData('com-vip-current', () =>
  api.post<Current>('/v1/mcenter/vip/current', {}),
)
const { data: packs } = await useAsyncData('com-vip-packages', () =>
  api.post<Pack[]>('/v1/mcenter/vip/packages', {}).catch(() => [] as Pack[]),
)

const packList = computed<Pack[]>(() => (Array.isArray(packs.value) ? packs.value : []))
const channel = ref('alipay')
const wxPayOn = computed(() =>
  Boolean(settings.value.sy_wxpayid || settings.value.sy_wxpaykey || settings.value.wx_appid),
)
const msg = ref('')

function descLines(desc: unknown): string[] {
  if (!desc) return []
  if (Array.isArray(desc)) return desc.map((d) => String(d))
  if (typeof desc === 'string') return [desc]
  if (typeof desc === 'object') {
    return Object.entries(desc as Record<string, unknown>).map(([k, v]) => `${k}: ${v}`)
  }
  return []
}

function payChannel() {
  if (channel.value === 'bank') return 'bank'
  if (channel.value !== 'wxpay') return channel.value
  if (import.meta.client && /Android|iPhone|iPad|Mobile|MicroMessenger/i.test(navigator.userAgent)) return 'wxh5'
  return 'wxpay'
}

async function buy(p: Pack) {
  msg.value = ''
  try {
    const q = await api.post<Quote>('/v1/mcenter/vip/quote', { kind: 'vip', id: p.id })
    if (q.style === 2) {
      const ok = window.confirm(`${t('common_00697')}${q.price}${t('common_01935')}?`)
      if (!ok) return
      await api.post('/v1/mcenter/vip/orders/integral', { package_code: p.code })
      msg.value = t('common.success')
      await refreshCurrent()
      return
    }
    if (q.style === 3) {
      const ok = window.confirm(`${t('common_00696')}${q.price}${t('common_00757')}?`)
      if (!ok) return
    }
    const created = await api.post<{ pay_url?: string; order_no?: string; msg?: string }>(
      '/v1/mcenter/vip/orders',
      { package_code: p.code, channel: payChannel() },
    )
    if (created?.order_no) {
      await navigateTo(`/com/cashier/${created.order_no}`)
      return
    }
    msg.value = created?.msg || t('common.success')
    await refreshCurrent()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_com_00097') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00097')" :error="error && !isUnauthErr(error) ? error : undefined">
    <template #pcTabs><MemberComVipTabs /></template>
    <template #h5Tabs><MemberComVipTabs /></template>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <div class="com_vip_box">
        <h2>{{ $t('wap_01229') }}</h2>
        <template v-if="current?.active">
          <p>{{ $t('wap_00025') }}: {{ current.rating_name || current.package_code }}</p>
          <p>
            {{ $t('member_com_00315') }}:
            {{ formatUnixDate(current.started_at) }} ~ {{ formatUnixDate(current.expires_at) }}
          </p>
          <p class="muted">
            {{ $t('wap_com_00106') }} {{ current.job_num ?? 0 }} ·
            {{ $t('wap_com_00029') }} {{ current.breakjob_num ?? 0 }} ·
            {{ $t('wap_00451') }} {{ current.down_resume ?? 0 }} ·
            {{ $t('wap_user_00216') }} {{ current.invite_resume ?? 0 }} ·
            {{ $t('wap_user_00008') }} {{ current.integral ?? 0 }}
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
      <div class="payment_list site-pc">
        <div class="payment_list_s mt10">{{ $t('wap_user_00313') }}：</div>
        <div class="payment_list_r">
          <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
          <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
          <label><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
        </div>
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
          <input type="button" class="payment_list_other" :value="$t('common.submit')" @click="buy(p)" />
        </div>
      </div>
      <div class="site-h5 issue_post_body">
        <MemberField wap :label="$t('wap_user_00313')">
          <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
          <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
          <label><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
        </MemberField>
        <div v-for="p in packList" :key="'h5-' + p.id" class="issue_post_body_card">
          <div class="Posted_card_top">
            <div class="Posted_card_name">{{ p.name }}</div>
            <div class="Posted_card_pay">¥{{ p.price_yuan }} / {{ p.duration_days }}d</div>
          </div>
          <ul v-if="descLines(p.desc).length">
            <li v-for="(line, i) in descLines(p.desc)" :key="i">{{ line }}</li>
          </ul>
          <button type="button" class="issue_post_body_btn" @click="buy(p)">{{ $t('common.submit') }}</button>
        </div>
      </div>
    </template>
    <p v-if="msg">{{ msg }}</p>
    <p>
      <NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink>
    </p>
  </MemberPanel>
</template>
