<script setup lang="ts">
type Cls = { id: number; integral: number; discount: number }
type Bank = { id: number; name: string; bank_name: string; bank_number: string }

const props = defineProps<{
  classes: Cls[]
  pickedId: number
  custom: string
  payYuan: number
  priceName: string
  channel: string
  wxPayOn: boolean
  bankList: Bank[]
}>()
const emit = defineEmits<{
  pick: [Cls]
  custom: []
  'update:custom': [string]
  customBlur: []
  'update:channel': [string]
  buy: []
  card: []
}>()

const customOn = computed(() => props.pickedId === 0)
function onCustomInput(ev: Event) {
  const v = String((ev.target as HTMLInputElement).value || '').replace(/[^0-9]/g, '')
  emit('update:custom', v)
}
</script>

<template>
  <div class="site-h5 site-h5-pay">
    <div class="pay_header">
      <div class="pay_header_c">
        <div class="pay_header_p">
          <div class="pay_header_icon">
            <img src="/legacy/h5/images/integral_notice.png" alt="" width="100%">
          </div>
          <slot name="tip" />
          <a href="javascript:;" class="site-h5-pay__card" @click="emit('card')">{{ $t('member_com_00485') }}</a>
        </div>
      </div>
    </div>
    <div class="pay_cont">
      <div class="integral_body">
        <div class="integral_body_card">
          <ul>
            <li
              v-for="c in classes"
              :key="'h5c-' + c.id"
              :class="pickedId === c.id && !customOn ? 'discount_opt_for' : 'body_card_bumber'"
              @click="emit('pick', c)"
            >
              <i class="body_card_bumber" :class="{ currcolor: pickedId === c.id && !customOn }">{{ c.integral }}</i>
              <i :class="{ currcolor: pickedId === c.id && !customOn }">{{ priceName }}</i>
              <div v-if="c.discount" class="body_card_bumber_discount">{{ c.discount / 10 }}</div>
            </li>
            <li :class="customOn ? 'discount_opt_for' : 'body_card_bumber'" @click="emit('custom')">
              <i class="body_card_bumber" :class="{ currcolor: customOn }">{{ $t('wap_user_00309') }}</i>
              <i :class="{ currcolor: customOn }">{{ priceName }}</i>
            </li>
          </ul>
          <div v-if="customOn" class="integral_body_pay">
            <div class="integral_body_pay_left">{{ $t('wap_01031') }}</div>
            <div class="integral_body_pay_right">
              <input
                :value="custom"
                type="text"
                inputmode="numeric"
                maxlength="6"
                :placeholder="$t('wap_01033')"
                @input="onCustomInput"
                @blur="emit('customBlur')"
              >
            </div>
          </div>
          <div class="integral_body_pay">
            <div class="integral_body_pay_left">{{ $t('wap_01032') }}</div>
            <div class="integral_body_pay_right">
              <i class="pay_right_number">{{ payYuan }}</i>
            </div>
          </div>
          <div class="dredge_body_pay" style="padding: 0">
            <div class="dredge_body_zfb" @click="emit('update:channel', 'stripe')">
              <div class="dredge_body_wx_box">
                <div class="wx_box_name">Stripe</div>
              </div>
              <div class="dredge_body_wx_icon">
                <img
                  :src="channel === 'stripe' ? '/legacy/h5/images/dredge_affirm.png' : '/legacy/h5/images/dredge_To_confirm.png'"
                  alt=""
                  width="100%"
                  height="100%"
                >
              </div>
            </div>
          </div>
        </div>
        <button type="button" class="integral_body_btn" @click="emit('buy')">{{ $t('wap_user_00307') }}</button>
      </div>
    </div>
  </div>
</template>
