<script setup lang="ts">
const { t } = useI18n()
const api = useApi()
const { applyToQuery } = useSubSite()
const form = reactive({
  title: '',
  salary: '',
  address: '',
  require: '',
  companyname: '',
  linkman: '',
  linktel: '',
  password: '',
  mans: '',
  oncepricegear: 0,
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const authcode = ref('')
const smsCode = ref('')
const msg = ref('')
const gears = ref<Array<{ id: number; days: number; price: number }>>([])
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
}
onMounted(async () => {
  await loadCaptcha()
  try {
    gears.value = await api.get('/v1/wap/once-jobs/gears')
    if (gears.value.length && !form.oncepricegear) form.oncepricegear = gears.value[0].id
  } catch {
    gears.value = []
  }
})
async function sendSms() {
  msg.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: form.linktel,
      scene: 'once',
      captcha_cid: captcha.value?.cid,
      authcode: authcode.value,
    })
    msg.value = t('ui.sms_sent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.send_failed')
    loadCaptcha()
  }
}
async function submit() {
  msg.value = ''
  try {
    const r = await api.post<{ id: number }>('/v1/wap/once-jobs', applyToQuery({
      ...form,
      captcha_cid: captcha.value?.cid || '',
      authcode: authcode.value,
      moblie_code: smsCode.value,
    }))
    msg.value = t('common.success')
    await navigateTo(`/once/${r.id}`)
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
    loadCaptcha()
  }
}
useSeoMeta({ title: t('wap_01356') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ $t('wap_01356') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.title" :placeholder="$t('wap_01357')" required />
      <input v-model="form.salary" :placeholder="$t('wap_01359')" />
      <input v-model="form.address" :placeholder="$t('wap_01363')" required />
      <textarea v-model="form.require" rows="5" required />
      <input v-model="form.companyname" :placeholder="$t('wap_01367')" required />
      <input v-model="form.linkman" :placeholder="$t('wap_01368')" required />
      <input v-model="form.linktel" :placeholder="$t('wap_01369')" required />
      <input v-model="form.mans" :placeholder="$t('wap_01360')" />
      <select v-model.number="form.oncepricegear">
        <option v-for="g in gears" :key="g.id" :value="g.id">{{ g.days }}{{ $t('wap_01375') }} · {{ g.price }}</option>
      </select>
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="authcode" :placeholder="$t('ui.image_captcha')" />
      <button type="button" @click="sendSms">{{ $t('wap_01373') }}</button>
      <input v-model="smsCode" :placeholder="$t('wap_01372')" />
      <input v-model="form.password" type="password" :placeholder="$t('wap_01380')" required />
      <button type="submit">{{ $t('wap_00354') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
