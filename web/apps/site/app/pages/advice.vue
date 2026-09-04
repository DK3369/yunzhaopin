<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { me, settings } = useSiteChrome()
const form = reactive({
  infotype: 'advice',
  content: '',
  moblie: '',
  username: '',
  captcha_cid: '',
  captcha_input: '',
  moblie_code: '',
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const msg = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
  form.captcha_input = ''
}
onMounted(async () => {
  if (me.value) {
    form.username = me.value.username || ''
  }
  await loadCaptcha()
})
async function sendSms() {
  msg.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: form.moblie,
      scene: 'advice',
      captcha_cid: form.captcha_cid,
      authcode: form.captcha_input,
    })
    msg.value = t('ui.sms_sent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.send_failed')
    await loadCaptcha()
  }
}
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/wap/advice', { ...form })
    msg.value = t('common.success')
    form.content = ''
    await loadCaptcha()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common.no')
    await loadCaptcha()
  }
}
useSeoMeta({ title: t('wap_user_00203') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00203') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.infotype" />
      <textarea v-model="form.content" rows="5" required />
      <input v-model="form.moblie" :placeholder="$t('wap_01619')" />
      <template v-if="String(settings.sy_advice_mobilecode || '') === '1'">
        <button type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
        <input v-model="form.moblie_code" :placeholder="$t('wap_01371')" />
      </template>
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="form.captcha_input" :placeholder="$t('wap_00110')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
