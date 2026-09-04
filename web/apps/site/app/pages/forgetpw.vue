<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const channel = ref<'sms' | 'email'>('sms')
const form = reactive({
  moblie: '',
  email: '',
  captcha_cid: '',
  authcode: '',
  moblie_code: '',
  email_code: '',
  password: '',
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const msg = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
}
onMounted(loadCaptcha)
async function sendSms() {
  msg.value = ''
  try {
    await api.post('/v1/wap/forgetpw/send-sms', {
      moblie: form.moblie,
      captcha_cid: form.captcha_cid,
      authcode: form.authcode,
    })
    msg.value = t('ui.sms_sent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.send_failed')
    loadCaptcha()
  }
}
async function sendEmail() {
  msg.value = ''
  try {
    await api.post('/v1/wap/forgetpw/send-email', {
      email: form.email,
      captcha_cid: form.captcha_cid,
      authcode: form.authcode,
    })
    msg.value = t('ui.sms_sent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.send_failed')
    loadCaptcha()
  }
}
async function resetPw() {
  msg.value = ''
  try {
    if (channel.value === 'email') {
      await api.post('/v1/wap/forgetpw/reset-by-email', {
        email: form.email,
        email_code: form.email_code,
        password: form.password,
      })
    } else {
      await api.post('/v1/wap/forgetpw/reset', {
        moblie: form.moblie,
        moblie_code: form.moblie_code,
        password: form.password,
      })
    }
    msg.value = t('ui.password_reset_ok')
    await navigateTo('/login')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.reset_failed')
  }
}
useSeoMeta({ title: t('wap_js_00123') })
</script>

<template>
  <section class="site-inner">
    <div style="text-align: right; margin-bottom: 12px"><LangSwitch /></div>
    <h1>{{ $t('wap_js_00123') }}</h1>
    <p>
      <button type="button" @click="channel = 'sms'">{{ $t('wap_01619') }}</button>
      <button type="button" @click="channel = 'email'">{{ $t('ui.email_addr') }}</button>
    </p>
    <form class="form" @submit.prevent="resetPw">
      <input v-if="channel === 'sms'" v-model="form.moblie" :placeholder="$t('wap_01619')" autocomplete="tel" />
      <input v-else v-model="form.email" :placeholder="$t('ui.email_addr')" autocomplete="email" />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="form.authcode" :placeholder="$t('ui.image_captcha')" />
      <button v-if="channel === 'sms'" type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
      <button v-else type="button" @click="sendEmail">{{ $t('admin_user_00166') }}</button>
      <input v-if="channel === 'sms'" v-model="form.moblie_code" :placeholder="$t('wap_01371')" />
      <input v-else v-model="form.email_code" :placeholder="$t('wap_01371')" />
      <input v-model="form.password" type="password" :placeholder="$t('wap_user_00305')" autocomplete="new-password" />
      <button type="submit">{{ $t('admin_user_00137') }}</button>
      <p v-if="msg" class="muted">{{ msg }}</p>
      <NuxtLink to="/login">{{ $t('common.login') }}</NuxtLink>
    </form>
  </section>
</template>
