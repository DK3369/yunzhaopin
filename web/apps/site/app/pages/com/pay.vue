<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { data: packs, error } = await useAsyncData('com-vip-packs', () =>
  api.post('/v1/mcenter/vip/packages', {}),
)
const { data: orders, refresh } = await useAsyncData('com-vip-orders', () =>
  api.post('/v1/mcenter/vip/orders/list', { page: 1, page_size: 20 }),
)
const msg = ref('')
const channel = ref('alipay')
const bankOrderNo = ref('')
const bankForm = reactive({
  bank_name: '',
  bank_number: '',
  bank_price: '',
  bank_time: '',
  order_remark: '',
  order_pic: '',
})
const { data: banks } = await useAsyncData('com-vip-banks', () =>
  api.post<{ id: number; name: string; bank_name: string; bank_number: string; bank_address: string }[]>(
    '/v1/mcenter/vip/bank-accounts',
    {},
  ).catch(() => []),
)
const bankList = computed(() => (Array.isArray(banks.value) ? banks.value : []))
const wxPayOn = computed(() =>
  Boolean(settings.value.sy_wxpayid || settings.value.sy_wxpaykey || settings.value.wx_appid),
)
function payChannel() {
  if (channel.value === 'bank') return 'bank'
  if (channel.value !== 'wxpay') return channel.value
  if (import.meta.client && /Android|iPhone|iPad|Mobile|MicroMessenger/i.test(navigator.userAgent)) return 'wxh5'
  return 'wxpay'
}
type PackDesc = {
  job_num?: number
  breakjob_num?: number
  resume?: number
  interview?: number
  top_num?: number
  rec_num?: number
  urgent_num?: number
  zph_num?: number
  part_num?: number
}
const packages = computed(() => (Array.isArray(packs.value) ? packs.value : packs.value?.list || []))
function quotaLines(p: { desc?: PackDesc | null }) {
  const raw = p.desc
  const d = raw && typeof raw === 'object' && !Array.isArray(raw) ? raw : {}
  const rows: Array<[number, string, string?]> = [
    [Number(d.job_num || 0), 'wap_com_00106'],
    [Number(d.breakjob_num || 0), 'wap_com_00029'],
    [Number(d.resume || 0), 'wap_00451'],
    [Number(d.interview || 0), 'wap_user_00216'],
    [Number(d.top_num || 0), 'wap_com_00238', 'common_02067'],
    [Number(d.rec_num || 0), 'wap_com_00237', 'common_02067'],
    [Number(d.urgent_num || 0), 'member_com_00613', 'common_02067'],
    [Number(d.zph_num || 0), 'member_com_00293'],
    [Number(d.part_num || 0), 'wap_user_00271'],
  ]
  return rows.filter(([n]) => n > 0)
}
async function buy(code: string) {
  msg.value = ''
  try {
    const created = await api.post('/v1/mcenter/vip/orders', { package_code: code, channel: payChannel() })
    if (channel.value === 'bank') {
      bankOrderNo.value = String(created?.order_no || '')
      bankForm.bank_price = String(created?.amount_yuan ?? '')
      msg.value = created?.order_no || t('common.success')
      await refresh()
      return
    }
    if (created?.pay_url) {
      window.location.href = created.pay_url
      return
    }
    msg.value = created?.msg || created?.order_no || t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function onVoucher(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/cert', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    bankForm.order_pic = r.key || r.url || ''
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function submitBank() {
  msg.value = ''
  if (!bankOrderNo.value) return
  try {
    let bank_time = 0
    if (bankForm.bank_time) {
      bank_time = Math.floor(new Date(`${bankForm.bank_time}T00:00:00`).getTime() / 1000)
    }
    await api.post('/v1/mcenter/vip/orders/paybank', {
      order_no: bankOrderNo.value,
      bank_name: bankForm.bank_name,
      bank_number: bankForm.bank_number,
      bank_price: bankForm.bank_price,
      bank_time,
      order_remark: bankForm.order_remark,
      order_pic: bankForm.order_pic,
    })
    msg.value = t('common.success')
    bankOrderNo.value = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function canFillBank(o: { channel?: string; status?: number; status_n?: string }) {
  return o.channel === 'bank' && (o.status === 0 || o.status === 3 || o.status_n === 'awaiting_confirm')
}
function canCancel(o: { status?: number }) {
  return o.status === 0
}
async function cancelOrder(o: { order_no?: string }) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/vip/orders/cancel', { order_no: String(o.order_no || '') })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function fillBank(o: { order_no?: string; amount_yuan?: number }) {
  bankOrderNo.value = String(o.order_no || '')
  bankForm.bank_price = String(o.amount_yuan ?? '')
}
useSeoMeta({ title: t('common_01946') })
</script>

<template>
  <MemberPanel :title="$t('common_01946')">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <div class="payment_list">
      <div class="payment_list_s mt10">{{ $t('member_com_00317') }}：</div>
      <div class="payment_list_r">
        <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
        <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
        <label v-if="bankList.length"><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
      </div>
    </div>
    <div v-if="channel === 'bank' && bankList.length" class="wxts_box">
      <div v-for="b in bankList" :key="b.id" class="wxts">{{ b.name }} {{ b.bank_name }} {{ b.bank_number }}</div>
    </div>
    <div class="payment_list">
      <div class="payment_list_s mt10">{{ $t('member_com_00316') }}：</div>
      <div class="payment_list_r">
        <span v-for="p in packages" :key="p.code" class="payment_list_text">
          <div class="payment_list_text_n">
            {{ p.name }}
            <em class="payment_list_text_dw">{{ p.price_yuan }} {{ $t('wap_00925') }}</em>
          </div>
          <input type="button" class="payment_list_other" :value="$t('common.submit')" @click="buy(p.code)" />
        </span>
      </div>
    </div>
    <form v-if="bankOrderNo" class="com_release_box" @submit.prevent="submitBank">
      <ul>
        <MemberReleaseRow :label="$t('ui.order_no')"><span>{{ bankOrderNo }}</span></MemberReleaseRow>
        <MemberReleaseRow :label="$t('model_00022')" required><input v-model="bankForm.bank_name" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('model_00023')" required><input v-model="bankForm.bank_number" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('model_00024')" required><input v-model="bankForm.bank_price" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00106')" required><input v-model="bankForm.bank_time" type="date" required /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00345')"><input v-model="bankForm.order_remark" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.image')"><input type="file" accept="image/jpeg,image/png,image/webp" @change="onVoucher" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
    </form>
    <MemberResumeH1 :title="$t('common_02029')" />
    <div v-if="(orders?.list || []).length" class="site-pc paylist_tit">
      <span class="paylist_span paylist_span_dh">{{ $t('ui.order_no') }}</span>
      <span class="paylist_span paylist_span_money">{{ $t('wap_00925') }}</span>
      <span class="paylist_span paylist_span_zt">{{ $t('member_user_00104') }}</span>
    </div>
    <div v-for="o in orders?.list || []" :key="o.order_no" class="site-pc paylist_list">
      <span class="paylist_span paylist_span_dh">{{ o.order_no }}</span>
      <span class="paylist_span paylist_span_money">{{ o.amount_yuan }}</span>
      <span class="paylist_span paylist_span_zt">{{ o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n }}</span>
      <span class="paylist_span paylist_span_cz">
        <a v-if="canFillBank(o)" href="javascript:;" class="cblue" @click="fillBank(o)">{{ $t('wap_01805') }}</a>
        <a v-if="canCancel(o)" href="javascript:;" class="cblue" @click="cancelOrder(o)">{{ $t('common.cancel') }}</a>
      </span>
    </div>
    <div class="site-h5 m_cardbox">
      <MemberSxNewsCard
        v-for="o in orders?.list || []"
        :key="'h5-' + o.order_no"
        :title="String(o.order_no)"
        :sub="String(o.amount_yuan)"
        :time="o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n"
      />
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
