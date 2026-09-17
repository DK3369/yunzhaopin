<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Cls = { id: number; integral: number; discount: number }
type ClassPack = {
  list?: Cls[]
  min_recharge?: number
  proportion?: number
  pricename?: string
  priceunit?: string
  balance?: number
}
type OrderRow = {
  order_no?: string
  amount_yuan?: number
  channel?: string
  status?: number
  status_n?: string
  order_kind?: number
}

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { data: pack, error, refresh: refreshPack } = await useAsyncData('user-integral-classes', () =>
  api.post<ClassPack>('/v1/mcenter/vip/integral-classes', {}),
)
const { data: orders, refresh } = await useAsyncData('user-recharge-orders', () =>
  api.post('/v1/mcenter/vip/orders/list', { page: 1, page_size: 20 }),
)
const msg = ref('')
const channel = ref('alipay')
const pickedId = ref(0)
const custom = ref('')
const remark = ref('')
const showCard = ref(false)
const cardNo = ref('')
const cardPw = ref('')
const bankOrderNo = ref('')
const bankForm = reactive({
  bank_name: '',
  bank_number: '',
  bank_price: '',
  bank_time: '',
  order_remark: '',
  order_pic: '',
})
const { data: banks } = await useAsyncData('user-vip-banks', () =>
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
const classes = computed(() => pack.value?.list || [])
const minRecharge = computed(() => Number(pack.value?.min_recharge || settings.value.integral_min_recharge || 0))
const proportion = computed(() => Math.max(1, Number(pack.value?.proportion || settings.value.integral_proportion || 1)))
const priceName = computed(() => String(pack.value?.pricename || settings.value.integral_pricename || t('wap_user_00008')))
const priceUnit = computed(() => String(pack.value?.priceunit || settings.value.integral_priceunit || ''))
const balance = computed(() => Number(pack.value?.balance || 0))
const picked = computed(() => classes.value.find((c) => c.id === pickedId.value) || null)
const priceInt = computed(() => {
  const n = Number(custom.value)
  if (Number.isFinite(n) && n > 0) return Math.floor(n)
  return Number(picked.value?.integral || 0)
})
const integralId = computed(() => {
  const n = priceInt.value
  if (picked.value && !custom.value) return picked.value.id
  let id = 0
  for (const c of classes.value) {
    if (n >= c.integral) id = c.id
  }
  return id
})
const discount = computed(() => {
  const id = integralId.value
  const c = classes.value.find((x) => x.id === id)
  if (c && priceInt.value >= c.integral) return Number(c.discount || 0)
  return 0
})
const payYuan = computed(() => {
  const n = priceInt.value
  if (n < 1) return 0
  let p = n / proportion.value
  if (discount.value > 0) p *= discount.value / 100
  return Math.round(p * 100) / 100
})
const rechargeOrders = computed(() =>
  ((orders.value?.list || []) as OrderRow[]).filter((o) => o.order_kind === 2),
)
watch(classes, (list) => {
  if (pickedId.value || custom.value) return
  const min = minRecharge.value
  const first = list.find((c) => !min || c.integral >= min) || list[0]
  if (first) pickedId.value = first.id
}, { immediate: true })
function pickClass(c: Cls) {
  const min = minRecharge.value
  if (min > 0 && c.integral < min) {
    msg.value = t('default_00088') + min + priceName.value
    return
  }
  msg.value = ''
  custom.value = ''
  pickedId.value = c.id
}
function onCustom() {
  pickedId.value = 0
  const n = Math.floor(Number(custom.value) || 0)
  const min = minRecharge.value
  if (n > 0 && min > 0 && n < min) {
    custom.value = String(min)
  }
}
async function buy() {
  msg.value = ''
  const n = priceInt.value
  const min = minRecharge.value
  if (min > 0 && n < min) {
    msg.value = t('default_00088') + min + priceName.value
    return
  }
  if (n < 1) {
    msg.value = t('common_00644')
    return
  }
  try {
    const created = await api.post('/v1/mcenter/vip/recharge', {
      price_int: n,
      integralid: integralId.value || undefined,
      channel: payChannel(),
      remark: remark.value,
    })
    if (created?.order_no) {
      await navigateTo(`/user/cashier/${created.order_no}`)
      return
    }
    msg.value = created?.msg || t('common.success')
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
function canFillBank(o: OrderRow) {
  return o.channel === 'bank' && (o.status === 0 || o.status === 3 || o.status_n === 'awaiting_confirm')
}
function canCancel(o: OrderRow) {
  return o.status === 0
}
async function cancelOrder(o: OrderRow) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/vip/orders/cancel', { order_no: String(o.order_no || '') })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function fillBank(o: OrderRow) {
  bankOrderNo.value = String(o.order_no || '')
  bankForm.bank_price = String(o.amount_yuan ?? '')
}
async function submitCard() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/vip/card', { card: cardNo.value, password: cardPw.value })
    msg.value = t('common.success')
    showCard.value = false
    cardNo.value = ''
    cardPw.value = ''
    await refreshPack()
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('common_01946') })
</script>

<template>
  <MemberPanel :title="$t('common_01946')">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <div class="com_new_tip">
      <span class="com_new_tip_h">{{ $t('member_com_00040') }}</span>
      {{ $t('common_01984') }}{{ priceName }}{{ balance }}，1{{ $t('common_02056') }}={{ proportion }}{{ priceUnit }}{{ priceName }}
      <template v-if="minRecharge > 0">，{{ $t('default_00088') }}{{ minRecharge }}{{ priceName }}</template>
      <a href="javascript:;" class="cblue" @click="showCard = true">{{ $t('member_com_00485') }}</a>
    </div>
    <div class="payment_list site-pc">
      <div class="payment_list_s mt10">{{ $t('wap_user_00313') }}：</div>
      <div class="payment_list_r">
        <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
        <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
        <label v-if="bankList.length"><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
      </div>
    </div>
    <div v-if="channel === 'bank' && bankList.length" class="wxts_box site-pc">
      <div v-for="b in bankList" :key="b.id" class="wxts">{{ b.name }} {{ b.bank_name }} {{ b.bank_number }}</div>
    </div>
    <div class="payment_list site-pc">
      <div class="payment_list_s mt10">{{ $t('member_com_00316') }}：</div>
      <div class="payment_list_r">
        <span
          v-for="c in classes"
          :key="c.id"
          class="payment_list_text"
          :class="{ payment_list_cur: pickedId === c.id && !custom }"
          :style="minRecharge > 0 && c.integral < minRecharge ? 'background-color:#E0E0E0' : undefined"
          @click="pickClass(c)"
        >
          <div class="payment_list_text_n">
            <i class="payment_list_text_icon" />{{ c.integral }}
            <em class="payment_list_text_dw">{{ priceName }}</em>
          </div>
          <em v-if="c.discount" class="payment_list_text_zk">{{ c.discount / 10 }}{{ $t('common_02080') }}</em>
        </span>
        <div class="payment_list_zdy" :class="{ payment_list_cur: !!custom }">
          <input
            v-model="custom"
            type="text"
            class="payment_list_input"
            :placeholder="$t('wap_user_00309')"
            maxlength="6"
            @focus="pickedId = 0"
            @blur="onCustom"
            @input="custom = custom.replace(/[^0-9]/g, '')"
          >
          <div class="payment_list_text_dw">{{ priceName }}</div>
        </div>
      </div>
    </div>
    <div class="payment_list site-pc">
      <div class="payment_list_s">{{ $t('wap_01032') }}</div>
      <div class="payment_list_r">
        <span class="payintegral">{{ payYuan }}</span>{{ $t('common_02056') }}
      </div>
    </div>
    <div class="payment_list site-pc">
      <div class="payment_list_s">{{ $t('member_com_00317') }}</div>
      <div class="payment_list_r">
        <textarea v-model="remark" class="payment_list_textarea" cols="40" />
      </div>
    </div>
    <div class="payment_list site-pc">
      <div class="payment_list_s">&nbsp;</div>
      <div class="payment_list_r">
        <input type="button" class="payment_list_other" :value="$t('member_user_00238')" @click="buy">
      </div>
    </div>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="buy">
        <MemberField wap :label="$t('wap_user_00313')">
          <label><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
          <label v-if="wxPayOn"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
          <label v-if="bankList.length"><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
        </MemberField>
        <div v-if="channel === 'bank' && bankList.length" class="issue_post_body_card">
          <div v-for="b in bankList" :key="'h5b-' + b.id">{{ b.name }} {{ b.bank_name }} {{ b.bank_number }}</div>
        </div>
        <div
          v-for="c in classes"
          :key="'h5c-' + c.id"
          class="issue_post_body_card"
          :class="{ payment_list_cur: pickedId === c.id && !custom }"
          @click="pickClass(c)"
        >
          <div class="Posted_card_top">
            <div class="Posted_card_name">{{ c.integral }} {{ priceName }}</div>
            <div v-if="c.discount" class="Posted_card_pay">{{ c.discount / 10 }}{{ $t('common_02080') }}</div>
          </div>
        </div>
        <MemberField wap :label="$t('wap_user_00309')">
          <input v-model="custom" type="text" maxlength="6" @focus="pickedId = 0" @blur="onCustom" @input="custom = custom.replace(/[^0-9]/g, '')">
        </MemberField>
        <MemberField wap :label="$t('wap_01032')">{{ payYuan }} {{ $t('common_02056') }}</MemberField>
        <MemberField wap :label="$t('member_com_00317')"><textarea v-model="remark" /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('member_user_00238') }}</button>
        <p><a href="javascript:;" @click="showCard = true">{{ $t('member_com_00485') }}</a></p>
      </form>
    </div>
    <form v-if="showCard" class="com_release_box" @submit.prevent="submitCard">
      <ul>
        <MemberReleaseRow :label="$t('admin_system_00534')" required>
          <input v-model="cardNo" class="com_release_textnew_text" required @input="cardNo = cardNo.replace(/[^0-9]/g, '')">
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00371')" required>
          <input v-model="cardPw" class="com_release_textnew_text" required @input="cardPw = cardPw.replace(/[^0-9]/g, '')">
        </MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.confirm') }}</button>
      <button type="button" class="btn_02" @click="showCard = false">{{ $t('common.cancel') }}</button>
    </form>
    <form v-if="bankOrderNo" class="com_release_box site-pc" @submit.prevent="submitBank">
      <ul>
        <MemberReleaseRow :label="$t('ui.order_no')"><span>{{ bankOrderNo }}</span></MemberReleaseRow>
        <MemberReleaseRow :label="$t('model_00022')" required><input v-model="bankForm.bank_name" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('model_00023')" required><input v-model="bankForm.bank_number" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('model_00024')" required><input v-model="bankForm.bank_price" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00106')" required><input v-model="bankForm.bank_time" type="date" required /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00345')"><input v-model="bankForm.order_remark" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.image')"><input type="file" accept="image/jpeg,image/png,image/webp" @change="onVoucher" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.submit') }}</button>
    </form>
    <div v-if="bankOrderNo" class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="submitBank">
        <MemberField wap :label="$t('ui.order_no')"><span>{{ bankOrderNo }}</span></MemberField>
        <MemberField wap :label="$t('model_00022')"><input v-model="bankForm.bank_name" required /></MemberField>
        <MemberField wap :label="$t('model_00023')"><input v-model="bankForm.bank_number" required /></MemberField>
        <MemberField wap :label="$t('model_00024')"><input v-model="bankForm.bank_price" required /></MemberField>
        <MemberField wap :label="$t('member_user_00106')"><input v-model="bankForm.bank_time" type="date" required /></MemberField>
        <MemberField wap :label="$t('wap_com_00345')"><input v-model="bankForm.order_remark" /></MemberField>
        <MemberField wap :label="$t('ui.image')"><input type="file" accept="image/jpeg,image/png,image/webp" @change="onVoucher" /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.submit') }}</button>
      </form>
    </div>
    <MemberResumeH1 :title="$t('common_02029')" />
    <div v-if="rechargeOrders.length" class="site-pc paylist_tit">
      <span class="paylist_span paylist_dh">{{ $t('ui.order_no') }}</span>
      <span class="paylist_span paylist_money">{{ $t('wap_00925') }}</span>
      <span class="paylist_span paylist_zt">{{ $t('member_user_00104') }}</span>
    </div>
    <div v-for="o in rechargeOrders" :key="o.order_no" class="site-pc paylist_list">
      <span class="paylist_span paylist_dh">{{ o.order_no }}</span>
      <span class="paylist_span paylist_money">{{ o.amount_yuan }}</span>
      <span class="paylist_span paylist_zt">{{ o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n }}</span>
      <span class="paylist_span paylist_cz">
        <NuxtLink v-if="o.status === 0 && o.order_no" :to="`/user/cashier/${o.order_no}`" class="cblue">{{ $t('wap_00401') }}</NuxtLink>
        <a v-if="canFillBank(o)" href="javascript:;" class="cblue" @click="fillBank(o)">{{ $t('wap_01805') }}</a>
        <a v-if="canCancel(o)" href="javascript:;" class="cblue" @click="cancelOrder(o)">{{ $t('common.cancel') }}</a>
      </span>
    </div>
    <div class="site-h5 detail_body">
      <div v-if="rechargeOrders.length" class="detail_body_card">
        <ul>
          <li v-for="o in rechargeOrders" :key="'h5-' + o.order_no">
            <div class="detail_box">
              <div class="detail_box_title">{{ o.order_no }}</div>
              <div class="detail_box_time">{{ o.status_n === 'awaiting_confirm' ? $t('admin_yunying_00086') : o.status_n }}</div>
            </div>
            <div class="detail_integral">{{ o.amount_yuan }}</div>
            <div class="detail_box_cz">
              <NuxtLink v-if="o.status === 0 && o.order_no" :to="`/user/cashier/${o.order_no}`">{{ $t('wap_00401') }}</NuxtLink>
              <a v-if="canFillBank(o)" href="javascript:;" @click="fillBank(o)">{{ $t('wap_01805') }}</a>
              <a v-if="canCancel(o)" href="javascript:;" @click="cancelOrder(o)">{{ $t('common.cancel') }}</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
    <p><NuxtLink to="/user/integral-rules">{{ $t('wap_user_00008') }}</NuxtLink></p>
  </MemberPanel>
</template>
