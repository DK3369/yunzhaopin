<script setup lang="ts">
import { ApiError } from '~/utils/envelope'

type Bank = { id: number; name: string; bank_name: string; bank_number: string; bank_address: string }
type Detail = {
  order_no: string
  type: number
  subject: string
  amount_yuan: number
  status: number
  status_n: string
  channel: string
  created_at_n: string
  payable: boolean
  channels: string[]
}

const props = defineProps<{ orderNo: string }>()
const api = useApi()
const { t } = useI18n()
const msg = ref('')
const channel = ref('alipay')
const { data, error, refresh } = await useAsyncData(
  () => `cashier-${props.orderNo}`,
  () => api.post<Detail>('/v1/mcenter/orders/detail', { order_no: props.orderNo }),
)
const detail = computed(() => data.value)
const errText = computed(() => {
  const e = error.value
  if (e instanceof ApiError) return e.message
  if (e instanceof Error) return e.message
  return t('ui.load_failed')
})
function statusLabel(d: Detail) {
  if (d.status_n === 'awaiting_confirm' || d.status === 3) return t('admin_yunying_00086')
  if (d.status === 0) return t('default_00032')
  if (d.status === 1) return t('admin_user_weipin_00045')
  if (d.status === 2) return t('common.cancel')
  return d.status_n
}
const channels = computed(() => detail.value?.channels || ['alipay'])
watch(channels, (list) => {
  if (!list.includes(channel.value) && list[0]) channel.value = list[0]
}, { immediate: true })
const bankList = ref<Bank[]>([])
const bankForm = reactive({
  bank_name: '',
  bank_number: '',
  bank_price: '',
  bank_time: '',
  order_remark: '',
  order_pic: '',
})

async function pay() {
  msg.value = ''
  try {
    const r = await api.post<{ pay_url?: string; channel?: string; bank_accounts?: Bank[] }>(
      '/v1/mcenter/orders/pay',
      { order_no: props.orderNo, channel: channel.value },
    )
    if (r.pay_url) {
      window.open(r.pay_url, '_blank')
      msg.value = t('common.success')
      return
    }
    if (channel.value === 'bank') {
      bankList.value = Array.isArray(r.bank_accounts) ? r.bank_accounts : []
      bankForm.bank_price = String(detail.value?.amount_yuan ?? '')
      return
    }
    msg.value = t('common.success')
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
  try {
    let bank_time = 0
    if (bankForm.bank_time) {
      bank_time = Math.floor(new Date(`${bankForm.bank_time}T00:00:00`).getTime() / 1000)
    }
    await api.post('/v1/mcenter/vip/orders/paybank', {
      order_no: props.orderNo,
      bank_name: bankForm.bank_name,
      bank_number: bankForm.bank_number,
      bank_price: bankForm.bank_price,
      bank_time,
      order_remark: bankForm.order_remark,
      order_pic: bankForm.order_pic,
    })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
</script>

<template>
  <p v-if="error" class="muted">{{ errText }}</p>
  <template v-else-if="detail">
    <div class="payment_list site-pc">
      <div class="payment_list_s">{{ $t('ui.order_no') }}</div>
      <div class="payment_list_r">{{ detail.order_no }}</div>
    </div>
    <div class="payment_list site-pc">
      <div class="payment_list_s">{{ $t('member_user_00039') }}</div>
      <div class="payment_list_r">{{ detail.subject }}</div>
    </div>
    <div class="payment_list site-pc">
      <div class="payment_list_s">{{ $t('ui.amount') }}</div>
      <div class="payment_list_r"><span class="payintegral">{{ detail.amount_yuan }}</span></div>
    </div>
    <div class="payment_list site-pc">
      <div class="payment_list_s">{{ $t('member_user_00181') }}</div>
      <div class="payment_list_r">{{ statusLabel(detail) }}</div>
    </div>
    <div v-if="detail.payable" class="payment_list site-pc">
      <div class="payment_list_s mt10">{{ $t('wap_user_00313') }}：</div>
      <div class="payment_list_r">
        <label v-if="channels.includes('alipay')"><input v-model="channel" type="radio" value="alipay" /> {{ $t('wap_00627') }}</label>
        <label v-if="channels.includes('wxpay')"><input v-model="channel" type="radio" value="wxpay" /> {{ $t('wap_user_00202') }}</label>
        <label v-if="channels.includes('bank')"><input v-model="channel" type="radio" value="bank" /> {{ $t('wap_01805') }}</label>
      </div>
    </div>
    <div v-if="detail.payable" class="payment_list site-pc">
      <div class="payment_list_s">&nbsp;</div>
      <div class="payment_list_r">
        <input type="button" class="payment_list_other" :value="$t('wap_00401')" @click="pay">
      </div>
    </div>
    <div class="site-h5 site-h5-pay">
      <div class="integral_body">
        <div class="integral_body_card">
          <div class="integral_body_pay">
            <div class="integral_body_pay_left">{{ $t('ui.order_no') }}</div>
            <div class="integral_body_pay_right">{{ detail.order_no }}</div>
          </div>
          <div class="integral_body_pay">
            <div class="integral_body_pay_left">{{ $t('member_user_00039') }}</div>
            <div class="integral_body_pay_right">{{ detail.subject }}</div>
          </div>
          <div class="integral_body_pay">
            <div class="integral_body_pay_left">{{ $t('ui.amount') }}</div>
            <div class="integral_body_pay_right">
              <i class="pay_right_number">{{ detail.amount_yuan }}</i>
            </div>
          </div>
          <div class="integral_body_pay">
            <div class="integral_body_pay_left">{{ $t('member_user_00181') }}</div>
            <div class="integral_body_pay_right">{{ statusLabel(detail) }}</div>
          </div>
          <div v-if="detail.payable" class="dredge_body_pay" style="padding: 0">
            <div v-if="channels.includes('alipay')" class="dredge_body_zfb" @click="channel = 'alipay'">
              <div class="dredge_body_wx_box">
                <div class="wx_box_icon">
                  <img src="/legacy/h5/images/dredge_zfb.png" alt="" width="100%" height="100%">
                </div>
                <div class="wx_box_name">{{ $t('wap_00627') }}</div>
              </div>
              <div class="dredge_body_wx_icon">
                <img
                  :src="channel === 'alipay' ? '/legacy/h5/images/dredge_affirm.png' : '/legacy/h5/images/dredge_To_confirm.png'"
                  alt=""
                  width="100%"
                  height="100%"
                >
              </div>
            </div>
            <div v-if="channels.includes('wxpay')" class="dredge_body_wx" @click="channel = 'wxpay'">
              <div class="dredge_body_wx_box">
                <div class="wx_box_icon">
                  <img src="/legacy/h5/images/dredge_wx.png" alt="" width="100%" height="100%">
                </div>
                <div class="wx_box_name">{{ $t('wap_user_00202') }}</div>
              </div>
              <div class="dredge_body_wx_icon">
                <img
                  :src="channel === 'wxpay' ? '/legacy/h5/images/dredge_affirm.png' : '/legacy/h5/images/dredge_To_confirm.png'"
                  alt=""
                  width="100%"
                  height="100%"
                >
              </div>
            </div>
            <div v-if="channels.includes('bank')" class="dredge_body_wx" @click="channel = 'bank'">
              <div class="dredge_body_wx_box">
                <div class="wx_box_name">{{ $t('wap_01805') }}</div>
              </div>
              <div class="dredge_body_wx_icon">
                <img
                  :src="channel === 'bank' ? '/legacy/h5/images/dredge_affirm.png' : '/legacy/h5/images/dredge_To_confirm.png'"
                  alt=""
                  width="100%"
                  height="100%"
                >
              </div>
            </div>
          </div>
        </div>
        <button v-if="detail.payable" type="button" class="integral_body_btn" @click="pay">{{ $t('wap_00401') }}</button>
      </div>
    </div>
    <div v-if="bankList.length" class="wxts_box">
      <div v-for="b in bankList" :key="b.id" class="wxts">{{ b.name }} {{ b.bank_name }} {{ b.bank_number }} {{ b.bank_address }}</div>
    </div>
    <form v-if="bankList.length" class="com_release_box site-pc" @submit.prevent="submitBank">
      <ul>
        <MemberReleaseRow :label="$t('model_00022')" required><input v-model="bankForm.bank_name" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('model_00023')" required><input v-model="bankForm.bank_number" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('model_00024')" required><input v-model="bankForm.bank_price" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00106')" required><input v-model="bankForm.bank_time" type="date" required /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00345')"><input v-model="bankForm.order_remark" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.image')"><input type="file" accept="image/jpeg,image/png,image/webp" @change="onVoucher" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.submit') }}</button>
    </form>
    <div v-if="bankList.length" class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="submitBank">
        <MemberField wap :label="$t('model_00022')"><input v-model="bankForm.bank_name" required /></MemberField>
        <MemberField wap :label="$t('model_00023')"><input v-model="bankForm.bank_number" required /></MemberField>
        <MemberField wap :label="$t('model_00024')"><input v-model="bankForm.bank_price" required /></MemberField>
        <MemberField wap :label="$t('member_user_00106')"><input v-model="bankForm.bank_time" type="date" required /></MemberField>
        <MemberField wap :label="$t('wap_com_00345')"><input v-model="bankForm.order_remark" /></MemberField>
        <MemberField wap :label="$t('ui.image')"><input type="file" accept="image/jpeg,image/png,image/webp" @change="onVoucher" /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.submit') }}</button>
      </form>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </template>
</template>
