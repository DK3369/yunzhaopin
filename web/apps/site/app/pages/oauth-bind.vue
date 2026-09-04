<script setup lang="ts">
const route = useRoute()
const { t } = useI18n()
const api = useApi()
const ticket = computed(() => String(route.query.ticket || ''))
const tab = ref<'bind' | 'reg'>('reg')
const username = ref('')
const password = ref('')
const mobile = ref('')
const smsCode = ref('')
const usertype = ref(1)
const { data: captcha } = await useAsyncData('oauth-bind-captcha', () =>
  api.post<{ cid: string; image: string }>('/v1/wap/captcha').catch(() => null),
)
const authcode = ref('')
const err = ref('')

async function loadCaptcha() {
  try {
    captcha.value = await api.post('/v1/wap/captcha')
  } catch {
    captcha.value = null
  }
}
onMounted(() => {
  if (!ticket.value) err.value = t('ui.oauth_ticket_missing')
})

async function afterLogin(me: { uid: number; usertype: number }) {
  await navigateTo(me.usertype === 2 ? '/com' : '/user')
}

async function sendSms() {
  err.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: mobile.value,
      scene: 'register',
      captcha_cid: captcha.value?.cid,
      authcode: authcode.value,
    })
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : t('common_00888')
    loadCaptcha()
  }
}

async function submitReg() {
  err.value = ''
  try {
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/oauth-fast-reg', {
      method: 'POST',
      body: {
        ticket: ticket.value,
        moblie: mobile.value,
        moblie_code: smsCode.value,
        password: password.value,
        usertype: usertype.value,
      },
    })
    await afterLogin(me)
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || t('common_00888')
    loadCaptcha()
  }
}

async function submitBind() {
  err.value = ''
  try {
    await $fetch('/api/auth/login', {
      method: 'POST',
      body: {
        username: username.value,
        password: password.value,
        authcode: authcode.value,
        captcha_cid: captcha.value?.cid,
      },
    })
    await api.post('/v1/wap/oauth/bind-pending', { ticket: ticket.value })
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/me')
    await afterLogin(me)
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || (e instanceof Error ? e.message : t('common_00888'))
    loadCaptcha()
  }
}

useSeoMeta({ title: t('ui.bind_account') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ $t('ui.bind_account') }}</h1>
    <p class="muted">{{ $t('ui.oauth_need_bind') }}</p>
    <p>
      <button type="button" :class="{ on: tab === 'reg' }" @click="tab = 'reg'">{{ $t('ui.fast_reg') }}</button>
      <button type="button" :class="{ on: tab === 'bind' }" @click="tab = 'bind'">{{ $t('common.login') }}</button>
    </p>
    <form v-if="tab === 'reg'" class="form" @submit.prevent="submitReg">
      <select v-model.number="usertype">
        <option :value="1">{{ $t('wap_00686') }}</option>
        <option :value="2">{{ $t('ui.hire') }}</option>
      </select>
      <input v-model="mobile" :placeholder="$t('wap_01619')" autocomplete="tel" />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="authcode" :placeholder="$t('ui.image_captcha')" />
      <button type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
      <input v-model="smsCode" :placeholder="$t('wap_01371')" />
      <input v-model="password" type="password" :placeholder="$t('wap_user_00371')" autocomplete="new-password" />
      <button type="submit">{{ $t('common.register') }}</button>
    </form>
    <form v-else class="form" @submit.prevent="submitBind">
      <input v-model="username" :placeholder="$t('admin_user_00140')" />
      <input v-model="password" type="password" :placeholder="$t('wap_user_00371')" />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="authcode" :placeholder="$t('ui.image_captcha')" />
      <button type="submit">{{ $t('ui.bind_account') }}</button>
    </form>
    <p v-if="err" class="muted">{{ err }}</p>
  </section>
</template>
