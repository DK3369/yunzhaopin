<script setup lang="ts">
const route = useRoute()
const api = useApi()
const { t } = useI18n()
const form = reactive({
  uid: Number(route.query.uid || 0),
  code: String(route.query.code || ''),
  username: '',
  password: '',
  cpassword: '',
})
const msg = ref('')
function validate() {
  if (!form.username.trim()) {
    msg.value = t('wap_01454')
    return false
  }
  if (form.username.length < 2 || form.username.length > 16) {
    msg.value = t('wap_00168')
    return false
  }
  if (form.password.length < 6 || form.password.length > 20) {
    msg.value = `${t('wap_01586')}${t('common_02053')}${t('wap_01587')}`
    return false
  }
  if (form.password !== form.cpassword) {
    msg.value = t('wap_js_00138')
    return false
  }
  return true
}
async function submit() {
  msg.value = ''
  if (!validate()) return
  try {
    await api.post('/v1/wap/claim', {
      uid: form.uid,
      code: form.code,
      username: form.username,
      password: form.password,
    })
    msg.value = t('wap_01780')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('wap_01781')
  }
}
useSeoMeta({ title: t('resume_00011') })
</script>

<template>
  <section class="password_box">
    <h1>{{ $t('resume_00011') }}</h1>
    <form class="form account" @submit.prevent="submit">
      <input type="hidden" :value="form.uid" />
      <input type="hidden" :value="form.code" />
      <div class="J_validate_group">
        <p class="selecttip">{{ $t('wap_00172') }}</p>
        <input v-model="form.username" class="input_295_34" :placeholder="$t('wap_01451')" required />
        <p class="selecttip">{{ $t('wap_user_00305') }}：</p>
        <input v-model="form.password" class="input_295_34" type="password" :placeholder="$t('wap_01452')" required />
        <p class="selecttip">{{ $t('wap_01450') }}</p>
        <input v-model="form.cpassword" class="input_295_34" type="password" :placeholder="$t('wap_01453')" required />
        <div class="reset_btnbox">
          <button type="submit" class="reset_xia">{{ $t('common.submit') }}</button>
        </div>
      </div>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
