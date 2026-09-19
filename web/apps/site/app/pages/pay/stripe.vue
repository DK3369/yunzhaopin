<script setup lang="ts">
import { ApiError } from '~/utils/envelope'

const { t } = useI18n()
const route = useRoute()
const api = useApi()
const { data: me } = await useAuthMe()

const orderNo = String(route.query.order_no || '').trim()
const sessionId = String(route.query.session_id || '').trim()
const canceled = String(route.query.canceled || '') === '1'
const msg = ref(t('ui.stripe_pay_wait'))

function cashierPath(usertype: number, no: string) {
  const base = Number(usertype) === 1 ? '/user/cashier' : '/com/cashier'
  return `${base}/${no}`
}

if (!orderNo) {
  msg.value = t('ui.failed')
} else if (!me.value?.uid) {
  await navigateTo({ path: '/login', query: { next: route.fullPath } })
} else if (canceled) {
  msg.value = t('ui.stripe_pay_cancel')
  await navigateTo(cashierPath(Number(me.value.usertype || 0), orderNo))
} else {
  if (sessionId.startsWith('cs_')) {
    try {
      await api.post('/v1/mcenter/orders/stripe-return', {
        order_no: orderNo,
        session_id: sessionId,
      })
      msg.value = t('ui.stripe_pay_ok')
    } catch (e: unknown) {
      msg.value = e instanceof ApiError || e instanceof Error ? e.message : t('ui.failed')
    }
  }
  await navigateTo(cashierPath(Number(me.value.usertype || 0), orderNo))
}

useSeoMeta({ title: t('ui.stripe_pay_title') })
</script>

<template>
  <section class="password_box">
    <h1>{{ $t('ui.stripe_pay_title') }}</h1>
    <p>{{ msg }}</p>
    <p v-if="!me?.uid">
      <NuxtLink :to="{ path: '/login', query: { next: route.fullPath } }">{{ $t('ui.stripe_pay_login') }}</NuxtLink>
    </p>
  </section>
</template>
