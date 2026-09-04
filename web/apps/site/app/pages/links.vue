<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data } = await useAsyncData('links', () => api.get('/v1/wap/friend-links'))
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as { id: number; name: string; url: string }[])
const form = reactive({ name: '', url: '', captcha_cid: '', captcha_input: '' })
const captcha = ref<{ cid: string; image: string } | null>(null)
const msg = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
  form.captcha_input = ''
}
onMounted(loadCaptcha)
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/wap/friend-links/apply', { ...form })
    msg.value = t('common.success')
    form.name = ''
    form.url = ''
    await loadCaptcha()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
    await loadCaptcha()
  }
}
useSeoMeta({ title: t('ui.links') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.links') }}</h1>
    <p v-if="!list.length" class="muted">{{ $t('ui.no_links') }}</p>
    <ul v-else class="stack">
      <li v-for="row in list" :key="row.id">
        <a :href="row.url" rel="nofollow noopener" target="_blank">{{ row.name }}</a>
      </li>
    </ul>
    <h2>{{ $t('common.submit') }}</h2>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.name" required />
      <input v-model="form.url" required />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="form.captcha_input" :placeholder="$t('wap_00110')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
