<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({
  moblie: '',
  captcha_cid: '',
  authcode: '',
  moblie_code: '',
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
async function resetPw() {
  msg.value = ''
  try {
    await api.post('/v1/wap/forgetpw/reset', {
      moblie: form.moblie,
      moblie_code: form.moblie_code,
      password: form.password,
    })
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
    <h1>{{ $t('wap_js_00123') }}</h1>
    <form class="form" @submit.prevent="resetPw">
      <input v-model="form.moblie" :placeholder="$t('wap_01619')" autocomplete="tel" />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="form.authcode" :placeholder="$t('ui.image_captcha')" />
      <button type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
      <input v-model="form.moblie_code" :placeholder="$t('wap_01371')" />
      <input v-model="form.password" type="password" :placeholder="$t('wap_user_00305')" autocomplete="new-password" />
      <button type="submit">{{ $t('admin_user_00137') }}</button>
      <p v-if="msg" class="muted">{{ msg }}</p>
      <NuxtLink to="/login">{{ $t('common.login') }}</NuxtLink>
    </form>
  </section>
</template>
