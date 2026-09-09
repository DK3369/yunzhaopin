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
    <section v-for="g in groups" :key="g.id">
      <h2>{{ g.name }}</h2>
      <article v-for="d in g.details || []" :key="d.id" class="job-card">
        <p>¥{{ d.service_price }}</p>
        <ul class="muted">
          <li v-if="d.job_num">{{ $t('wap_com_00106') }} {{ d.job_num }}</li>
          <li v-if="d.breakjob_num">{{ $t('wap_com_00029') }} {{ d.breakjob_num }}</li>
          <li v-if="d.resume">{{ $t('wap_00451') }} {{ d.resume }}</li>
          <li v-if="d.interview">{{ $t('wap_user_00216') }} {{ d.interview }}</li>
          <li v-if="d.top_num">{{ $t('wap_com_00238') }} {{ d.top_num }}{{ $t('common_02067') }}</li>
          <li v-if="d.rec_num">{{ $t('wap_com_00237') }} {{ d.rec_num }}{{ $t('common_02067') }}</li>
          <li v-if="d.urgent_num">{{ $t('member_com_00613') }} {{ d.urgent_num }}{{ $t('common_02067') }}</li>
        </ul>
        <button type="button" @click="buy(d.id)">{{ $t('common.submit') }}</button>
      </article>
    </section>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
