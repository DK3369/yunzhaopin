<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const form = reactive({
  username: '',
  password: '',
  moblie: '',
  captcha_cid: '',
  checkcode: '',
  usertype: 1,
  regway: 2,
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const err = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
}
onMounted(loadCaptcha)
async function submit() {
  err.value = ''
  try {
    await api.post('/v1/wap/register', { ...form })
    await $fetch('/api/auth/login', {
      method: 'POST',
      body: { username: form.username || form.moblie, password: form.password },
    })
    await api.post('/v1/mcenter/apply', { job_id: id })
    await navigateTo(`/jobs/${id}`)
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : t('common_00888')
    await loadCaptcha()
  }
}
useSeoMeta({ title: t('ui.quick_apply') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.quick_apply') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.username" :placeholder="$t('admin_user_00140')" />
      <input v-model="form.moblie" :placeholder="$t('wap_01619')" />
      <input v-model="form.password" type="password" :placeholder="$t('wap_user_00371')" />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="form.checkcode" :placeholder="$t('wap_00110')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="err" class="muted">{{ err }}</p>
  </section>
</template>
