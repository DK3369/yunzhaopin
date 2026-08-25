<script setup lang="ts">
const api = useApi()
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
    msg.value = '验证码已发送'
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '发送失败'
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
    msg.value = '密码已重置，请登录'
    await navigateTo('/login')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '重置失败'
  }
}
useSeoMeta({ title: '找回密码' })
</script>

<template>
  <section>
    <h1>找回密码</h1>
    <form class="form" @submit.prevent="resetPw">
      <input v-model="form.moblie" placeholder="手机号" autocomplete="tel" />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="form.authcode" placeholder="图形验证码" />
      <button type="button" @click="sendSms">发送短信验证码</button>
      <input v-model="form.moblie_code" placeholder="短信验证码" />
      <input v-model="form.password" type="password" placeholder="新密码" autocomplete="new-password" />
      <button type="submit">重置密码</button>
      <p v-if="msg" class="muted">{{ msg }}</p>
      <NuxtLink to="/login">返回登录</NuxtLink>
    </form>
  </section>
</template>
