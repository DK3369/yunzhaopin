<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-oauth-bindings', () =>
  api.post<{ providers?: string[] }>('/v1/mcenter/oauth-bindings', {}),
)
const mobile = ref('')
const mobileCode = ref('')
const email = ref('')
const msg = ref('')

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}

async function unbind(provider: string) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/oauth-bindings/unbind', { provider })
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function sendMobile() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/mobile/send', { moblie: mobile.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function bindMobile() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/mobile/verify', { moblie: mobile.value, moblie_code: mobileCode.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function sendEmail() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/email/send', { email: email.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

useSeoMeta({ title: t('member_user_00059') })
</script>

<template>
  <section>
    <h1>{{ $t('member_user_00059') }}</h1>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <nav class="stack">
        <NuxtLink to="/com/profile" class="job-card">{{ $t('wap_user_00341') }}</NuxtLink>
        <NuxtLink to="/com/cert" class="job-card">{{ $t('wap_com_00075') }}</NuxtLink>
        <NuxtLink to="/com/password" class="job-card">{{ $t('member_com_00070') }}</NuxtLink>
        <NuxtLink to="/user/account" class="job-card">{{ $t('member_user_00220') }} / {{ $t('member_com_00538') }}</NuxtLink>
      </nav>
      <h2>{{ $t('wap_00389') }}</h2>
      <p v-if="!(data?.providers || []).length" class="muted">{{ $t('ui.no_binding') }}</p>
      <ul v-else class="stack">
        <li v-for="p in data?.providers || []" :key="p">
          {{ p }}
          <button type="button" @click="unbind(p)">{{ $t('wap_js_00065') }}</button>
        </li>
      </ul>
      <form class="form" @submit.prevent="bindMobile">
        <input v-model="mobile" :placeholder="$t('common.phone')" />
        <button type="button" @click="sendMobile">{{ $t('common.submit') }}</button>
        <input v-model="mobileCode" :placeholder="$t('wap_01371')" />
        <button type="submit">{{ $t('common.save') }}</button>
      </form>
      <form class="form" @submit.prevent="sendEmail">
        <input v-model="email" :placeholder="$t('member_user_00282')" />
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </section>
</template>
