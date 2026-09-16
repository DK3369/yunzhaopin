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
const { data, error, refresh } = await useAsyncData('com-packs', () =>
  api.post<Group[]>('/v1/mcenter/packs/list', {}).catch(() => [] as Group[]),
)
const groups = computed(() => (Array.isArray(data.value) ? data.value : []) as Group[])
const msg = ref('')

async function buy(detailId: number) {
  msg.value = ''
  try {
    const created = await api.post<{ order_no?: string; pay_url?: string; msg?: string }>('/v1/mcenter/packs/orders', {
      detail_id: detailId,
      channel: 'alipay',
    })
    if (created?.pay_url) {
      window.location.href = created.pay_url
      return
    }
    msg.value = created?.msg || created?.order_no || t('ui.load_failed')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_com_00393') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00393')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !groups.length">
    <p>
      <NuxtLink to="/com/member-right">{{ $t('wap_com_00097') }}</NuxtLink>
    </p>
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
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
