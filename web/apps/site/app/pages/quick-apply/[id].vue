<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const form = reactive({
  uname: '',
  telphone: '',
  password: '',
  sex: 1,
  birthday: '',
  edu: 0,
  exp: 0,
  captcha_cid: '',
  checkcode: '',
  moblie_code: '',
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const err = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
}
onMounted(loadCaptcha)
async function sendSms() {
  err.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: form.telphone,
      scene: 'register',
      captcha_cid: form.captcha_cid,
      authcode: form.checkcode,
    })
    err.value = t('ui.sms_sent')
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : t('common_00888')
    await loadCaptcha()
  }
}
async function submit() {
  err.value = ''
  try {
    await $fetch('/api/auth/quick-apply', {
      method: 'POST',
      body: { ...form, job_id: id },
    })
    await navigateTo(`/jobs/${id}`)
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || (e instanceof Error ? e.message : t('common_00888'))
    await loadCaptcha()
  }
}
useSeoMeta({ title: t('ui.quick_apply') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.quick_apply') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.uname" :placeholder="$t('admin_user_00140')" required />
      <select v-model.number="form.sex">
        <option :value="1">{{ $t('common_02092') }}</option>
        <option :value="2">{{ $t('common_02069') }}</option>
      </select>
      <input v-model="form.birthday" :placeholder="$t('wap_00454')" />
      <input v-model="form.telphone" :placeholder="$t('wap_01619')" required />
      <input v-model="form.password" type="password" :placeholder="$t('wap_user_00371')" required />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="form.checkcode" :placeholder="$t('wap_00110')" />
      <button type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
      <input v-model="form.moblie_code" :placeholder="$t('wap_01371')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="err" class="muted">{{ err }}</p>
  </section>
</template>
