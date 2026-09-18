<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'

type Current = {
  active: boolean
  package_code?: string | null
  started_at?: number | null
  expires_at?: number | null
  rating_name?: string
  integral?: number
  can_chat?: boolean
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
const { settings } = useSiteChrome()

const { data: current, error, refresh: refreshCurrent } = await useAsyncData('user-vip-current', () =>
  api.post<Current>('/v1/mcenter/vip/current', {}),
)
const { data: packs } = await useAsyncData('user-vip-packages', () =>
  api.post<Pack[]>('/v1/mcenter/vip/packages', {}).catch(() => [] as Pack[]),
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

async function buy(p: Pack) {
  msg.value = ''
  picked.value = p.id
  try {
    const created = await api.post<{ pay_url?: string; order_no?: string; msg?: string }>(
      '/v1/mcenter/vip/orders',
      { package_code: p.code, channel: payChannel() },
    )
    if (created?.order_no) {
      await navigateTo(`/user/cashier/${created.order_no}`)
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
    <template #h5Tabs><MemberComVipTabs kind="user" /></template>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <p class="muted">
        {{ current?.rating_name || current?.package_code || $t('ui.no_data') }}
        <template v-if="current?.expires_at">
          · {{ formatUnixDate(current.expires_at) }}
        </template>
        · {{ $t('wap_user_00008') }} {{ current?.integral ?? 0 }}
        <span v-if="current?.can_chat"> · {{ $t('wap_user_00363') }}</span>
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
        <div class="vip_box_db">{{ $t('ui.monthly_vip') }}</div>
        <p v-if="!packList.length" class="muted">{{ $t('ui.no_packages') }}</p>
        <div v-else class="vip_timebox">
          <ul>
            <li v-for="p in packList" :key="'t-' + p.id" class="vip_time_list">
              <div class="vip_time_left">
                <div class="vip_time_leftname">
                  {{ p.name }}<i class="vip_box_left_name_line" />
                </div>
                <div class="vip_box_left_money_n">
                  <span>{{ p.price_yuan }} / {{ p.duration_days }}{{ $t('common_02067') }}</span>
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
          <div class="dredge_body_tab_body">
            <p v-if="!packList.length" class="muted">{{ $t('ui.no_packages') }}</p>
            <ul v-else>
              <li v-for="p in packList" :key="'h5-' + p.id">
                <div class="dredge_body_tab_body_box" @click="buy(p)">
                  <div class="dredge_body_tab_body_left">
                    <div class="tab_body_left_number">
                      <div class="vip_box_left_money_price">
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
