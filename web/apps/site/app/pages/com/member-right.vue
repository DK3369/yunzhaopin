<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'

type Current = {
  active: boolean
  package_code?: string | null
  started_at?: number | null
  expires_at?: number | null
  rating?: number
  rating_name?: string
  rating_type?: number
  job_num?: number
  breakjob_num?: number
  down_resume?: number
  invite_resume?: number
  integral?: number
  com_vip_type?: number
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
const route = useRoute()
const { settings } = useSiteChrome()

const { data: current, error, refresh: refreshCurrent } = await useAsyncData('com-vip-current', () =>
  api.post<Current>('/v1/mcenter/vip/current', {}),
)

const vipType = computed(() => Number(current.value?.com_vip_type ?? 0))
const showPackage = computed(() => vipType.value !== 1)
const showTime = computed(() => vipType.value !== 2)
const showAdded = computed(
  () => Number(current.value?.rating_type) !== 2 && String(settings.value.com_integral_online || '') !== '4',
)
const tab = computed<'package' | 'time'>(() => {
  const q = String(route.query.kind || '')
  if (q === 'time' && showTime.value) return 'time'
  if (q === 'package' && showPackage.value) return 'package'
  return showPackage.value ? 'package' : 'time'
})
const isTime = computed(() => tab.value === 'time')

const { data: packs } = await useAsyncData(
  () => `com-vip-packages-${tab.value}`,
  () => api.post<Pack[]>('/v1/mcenter/vip/packages', { kind: tab.value }).catch(() => [] as Pack[]),
  { watch: [tab] },
)

const packList = computed<Pack[]>(() => (Array.isArray(packs.value) ? packs.value : []))
const channel = ref('alipay')
const wxPayOn = computed(() =>
  Boolean(settings.value.sy_wxpayid || settings.value.sy_wxpaykey || settings.value.wx_appid),
)
const msg = ref('')
const picked = ref(0)

function descLines(desc: unknown): string[] {
  if (!desc) return []
  if (Array.isArray(desc)) return desc.map((d) => String(d))
  if (typeof desc === 'string') return [desc]
  if (typeof desc === 'object') {
    return Object.entries(desc as Record<string, unknown>)
      .filter(([, v]) => v !== 0 && v !== '0' && v !== '')
      .map(([k, v]) => `${k}: ${v}`)
  }
  return []
}

function payChannel() {
  if (channel.value === 'bank') return 'bank'
  if (channel.value !== 'wxpay') return channel.value
  if (import.meta.client && /Android|iPhone|iPad|Mobile|MicroMessenger/i.test(navigator.userAgent)) return 'wxh5'
  return 'wxpay'
}

async function setTab(kind: 'package' | 'time') {
  await navigateTo({ path: '/com/member-right', query: { kind } })
}

async function buy(p: Pack) {
  msg.value = ''
  picked.value = p.id
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
      <MemberComScreen
        :tabs="[
          ...(showPackage
            ? [{ value: 'package', label: $t('wap_com_00380'), on: tab === 'package', select: () => setTab('package') }]
            : []),
          ...(showTime
            ? [{ value: 'time', label: $t('wap_com_00384'), on: tab === 'time', select: () => setTab('time') }]
            : []),
          ...(showAdded
            ? [{ value: 'added', label: $t('wap_com_00393'), on: false, select: () => navigateTo('/com/added') }]
            : []),
        ]"
      />
      <div class="com_new_tip site-pc">
        <span class="com_new_tip_h">{{ $t('member_com_00040') }}</span>
        {{ current?.rating_name || current?.package_code || $t('ui.no_data') }}
        <template v-if="current?.started_at || current?.expires_at">
          {{ $t('member_com_00315') }}
          {{ formatUnixDate(current?.started_at) }} ~
          {{ current?.expires_at === 0 ? $t('api_wxapp_00019') : formatUnixDate(current?.expires_at) }}
        </template>
        <NuxtLink v-if="current?.expires_at && !current?.active" to="/com/member-right" class="cblue">{{
          $t('wap_com_00066')
        }}</NuxtLink>
      </div>
      <p class="muted site-pc">
        {{ $t('wap_com_00106') }} {{ current?.job_num ?? 0 }} ·
        {{ $t('wap_com_00029') }} {{ current?.breakjob_num ?? 0 }} ·
        {{ $t('wap_00451') }} {{ current?.down_resume ?? 0 }} ·
        {{ $t('wap_user_00216') }} {{ current?.invite_resume ?? 0 }} ·
        {{ $t('wap_user_00008') }} {{ current?.integral ?? 0 }}
      </p>
      <div class="payment_list site-pc">
        <div class="payment_list_s mt10">{{ $t('wap_user_00313') }}：</div>
        <div class="payment_list_r">
          <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
          <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
          <label><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
        </div>
      </div>
      <div class="vip_box site-pc">
        <div class="vip_box_db">{{ $t('member_com_00610') }}</div>
        <p v-if="!packList.length" class="muted">{{ $t('ui.no_packages') }}</p>
        <ul v-else-if="!isTime">
          <li v-for="p in packList" :key="p.id" class="vip_box_list">
            <div class="vip_box_list_c">
              <div class="vip_box_left">
                <div class="vip_box_left_name">
                  {{ p.name }}<i class="vip_box_left_name_line" />
                </div>
                <div class="vip_box_left_money_b">
                  <span>¥{{ p.price_yuan }} / {{ p.duration_days }}{{ $t('common_02067') }}</span>
                </div>
              </div>
              <ul v-if="descLines(p.desc).length">
                <li v-for="(line, i) in descLines(p.desc)" :key="i">{{ line }}</li>
              </ul>
              <input type="button" class="btn_01" :value="$t('common.submit')" @click="buy(p)" />
            </div>
          </li>
        </ul>
        <div v-else class="vip_timebox">
          <ul>
            <li v-for="p in packList" :key="'t-' + p.id" class="vip_time_list">
              <div class="vip_time_left">
                <div class="vip_time_leftname">
                  {{ p.name }}<i class="vip_box_left_name_line" />
                </div>
                <div class="vip_box_left_money_n">
                  <span>¥{{ p.price_yuan }} / {{ p.duration_days }}{{ $t('common_02067') }}</span>
                </div>
              </div>
              <ul v-if="descLines(p.desc).length">
                <li v-for="(line, i) in descLines(p.desc)" :key="i">{{ line }}</li>
              </ul>
              <input type="button" class="btn_01" :value="$t('common.submit')" @click="buy(p)" />
            </li>
          </ul>
        </div>
      </div>
      <div class="dredge_body site-h5">
        <div class="dredge_body_tab">
          <div v-if="showPackage && showTime" class="dredge_body_tab_tetle">
            <ul>
              <li :class="{ pitch_on: tab === 'package' }" @click="setTab('package')">{{ $t('wap_com_00380') }}</li>
              <li :class="{ pitch_on: tab === 'time' }" @click="setTab('time')">{{ $t('wap_com_00384') }}</li>
              <li v-if="showAdded" @click="navigateTo('/com/added')">{{ $t('wap_com_00393') }}</li>
            </ul>
          </div>
          <div class="dredge_body_tab_body">
            <p v-if="!packList.length" class="muted">{{ $t('ui.no_packages') }}</p>
            <ul v-else>
              <li v-for="p in packList" :key="'h5-' + p.id">
                <div class="dredge_body_tab_body_box" @click="buy(p)">
                  <div class="dredge_body_tab_body_left">
                    <div class="tab_body_left_number">
                      <div class="vip_box_left_money_price">
                        <i>¥</i>
                        <i class="tab_body_left_number_monye">{{ p.price_yuan }}</i>
                      </div>
                    </div>
                    <div class="tab_body_left_text">
                      <div class="tab_body_left_text_top">
                        <div class="left_text_top_vip">{{ p.name }}</div>
                        <div class="left_text_top_time">{{ p.duration_days }}{{ $t('common_02067') }}</div>
                      </div>
                    </div>
                  </div>
                  <div class="dredge_body_tab_body_right">
                    <span v-if="picked === p.id">✓</span>
                  </div>
                </div>
                <ul v-if="descLines(p.desc).length">
                  <li v-for="(line, i) in descLines(p.desc)" :key="i">{{ line }}</li>
                </ul>
              </li>
            </ul>
          </div>
        </div>
        <div class="issue_post_body">
          <MemberField wap :label="$t('wap_user_00313')">
            <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
            <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
            <label><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
          </MemberField>
        </div>
      </div>
    </template>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
