<script setup lang="ts">
const api = useApi()
const form = reactive({
  username: '',
  password: '',
  moblie: '',
  email: '',
  captcha_cid: '',
  checkcode: '',
  usertype: 1,
})
const captcha = ref<{ cid: string; image: string } | null>(null)
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
}
onMounted(loadCaptcha)
const err = ref('')
async function submit() {
  err.value = ''
  try {
    await api.post('/v1/wap/register', { ...form })
    await navigateTo('/login')
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : '注册失败'
    loadCaptcha()
  }
}
useSeoMeta({ title: '注册' })
</script>

<template>
  <section>
    <h1>注册</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.username" placeholder="用户名" />
      <input v-model="form.password" type="password" placeholder="密码" />
      <input v-model="form.moblie" placeholder="手机" />
      <select v-model.number="form.usertype">
        <option :value="1">求职者</option>
        <option :value="2">企业</option>
      </select>
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="form.checkcode" placeholder="验证码" />
      <button type="submit">注册</button>
      <p v-if="err" class="muted">{{ err }}</p>
    </form>
  </section>
</template>
