<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Detail = {
  id: number
  service_price: string
  resume: number
  interview: number
  job_num: number
  breakjob_num: number
  zph_num: number
  top_num: number
  rec_num: number
  urgent_num: number
}
type Group = { id: number; name: string; details: Detail[] }

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { data: current } = await useAsyncData('com-vip-current', () =>
  api.post<{ rating_type?: number }>('/v1/mcenter/vip/current', {}).catch(() => null),
)
const blocked = computed(() => Number(current.value?.rating_type) === 2)
const { data, error, refresh } = await useAsyncData('com-packs', () =>
  blocked.value
    ? Promise.resolve([] as Group[])
    : api.post<Group[]>('/v1/mcenter/packs/list', {}).catch(() => [] as Group[]),
)
const groups = computed(() => (Array.isArray(data.value) ? data.value : []) as Group[])
const msg = ref('')
const channel = ref('alipay')
const wxPayOn = computed(() =>
  Boolean(settings.value.sy_wxpayid || settings.value.sy_wxpaykey || settings.value.wx_appid),
)
function payChannel() {
  if (channel.value === 'bank') return 'bank'
  if (channel.value !== 'wxpay') return channel.value
  if (import.meta.client && /Android|iPhone|iPad|Mobile|MicroMessenger/i.test(navigator.userAgent)) return 'wxh5'
  return 'wxpay'
}

async function buy(detailId: number) {
  msg.value = ''
  try {
    const q = await api.post<{ price?: number }>('/v1/mcenter/packs/quote', { detail_id: detailId }).catch(() => null)
    if (q?.price && !window.confirm(`${t('common_00696')}${q.price}${t('common_00757')}?`)) return
    const created = await api.post<{ order_no?: string; pay_url?: string; msg?: string }>('/v1/mcenter/packs/orders', {
      detail_id: detailId,
      channel: payChannel(),
    })
    if (created?.order_no) {
      await navigateTo(`/com/cashier/${created.order_no}`)
      return
    }
    msg.value = created?.msg || t('ui.load_failed')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_com_00393') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00393')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !blocked && !groups.length">
    <template #pcTabs><MemberComVipTabs /></template>
    <template #h5Tabs><MemberComVipTabs /></template>
    <p>
      <NuxtLink to="/com/member-right">{{ $t('wap_com_00097') }}</NuxtLink>
    </p>
    <p v-if="blocked" class="muted">
      {{ $t('member_com_00705') }}
      <NuxtLink to="/com/member-right" class="cblue">{{ $t('wap_com_00097') }}</NuxtLink>
    </p>
    <template v-if="!blocked">
    <div class="payment_list site-pc">
        <div class="payment_list_s mt10">{{ $t('wap_user_00313') }}：</div>
      <div class="payment_list_r">
        <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
        <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
        <label><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
      </div>
    </div>
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <div v-for="g in groups" :key="g.id" class="payment_list site-pc">
      <div class="payment_list_s">{{ g.name }}</div>
      <div class="payment_list_r">
        <span v-for="d in g.details || []" :key="d.id" class="payment_list_text">
          <div class="payment_list_text_n">
            ¥{{ d.service_price }}
            <em class="payment_list_text_dw">{{ $t('wap_00925') }}</em>
          </div>
          <p v-if="d.job_num" class="muted">{{ $t('wap_com_00106') }} {{ d.job_num }}</p>
          <p v-if="d.breakjob_num" class="muted">{{ $t('wap_com_00029') }} {{ d.breakjob_num }}</p>
          <p v-if="d.resume" class="muted">{{ $t('wap_00451') }} {{ d.resume }}</p>
          <p v-if="d.interview" class="muted">{{ $t('wap_user_00216') }} {{ d.interview }}</p>
          <p v-if="d.top_num" class="muted">{{ $t('wap_com_00238') }} {{ d.top_num }}{{ $t('common_02067') }}</p>
          <p v-if="d.rec_num" class="muted">{{ $t('wap_com_00237') }} {{ d.rec_num }}{{ $t('common_02067') }}</p>
          <p v-if="d.urgent_num" class="muted">{{ $t('member_com_00613') }} {{ d.urgent_num }}{{ $t('common_02067') }}</p>
          <input type="button" class="payment_list_other" :value="$t('common.submit')" @click="buy(d.id)" />
        </span>
      </div>
    </div>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent>
        <MemberField wap :label="$t('wap_user_00313')">
          <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
          <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
          <label><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
        </MemberField>
      </form>
      <div v-for="g in groups" :key="'h5g-' + g.id">
        <div class="comshowtip">{{ g.name }}</div>
        <div v-for="d in g.details || []" :key="'h5d-' + d.id" class="issue_post_body_card">
          <div class="Posted_card_top">
            <div class="Posted_card_name">¥{{ d.service_price }}</div>
            <div class="Posted_card_pay">{{ $t('wap_00925') }}</div>
          </div>
          <p v-if="d.job_num" class="muted">{{ $t('wap_com_00106') }} {{ d.job_num }}</p>
          <p v-if="d.breakjob_num" class="muted">{{ $t('wap_com_00029') }} {{ d.breakjob_num }}</p>
          <p v-if="d.resume" class="muted">{{ $t('wap_00451') }} {{ d.resume }}</p>
          <p v-if="d.interview" class="muted">{{ $t('wap_user_00216') }} {{ d.interview }}</p>
          <p v-if="d.top_num" class="muted">{{ $t('wap_com_00238') }} {{ d.top_num }}{{ $t('common_02067') }}</p>
          <p v-if="d.rec_num" class="muted">{{ $t('wap_com_00237') }} {{ d.rec_num }}{{ $t('common_02067') }}</p>
          <p v-if="d.urgent_num" class="muted">{{ $t('member_com_00613') }} {{ d.urgent_num }}{{ $t('common_02067') }}</p>
          <button type="button" class="issue_post_body_btn" @click="buy(d.id)">{{ $t('common.submit') }}</button>
        </div>
      </div>
    </div>
    </template>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
