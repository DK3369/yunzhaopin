<script setup lang="ts">
const route = useRoute()
const api = useApi()
const { t } = useI18n()
const form = reactive({
  uid: Number(route.query.uid || 0),
  code: '',
  username: '',
  password: '',
})
const msg = ref('')
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/wap/claim', { ...form })
    msg.value = t('ui.claim_ok')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.claim') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.claim') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model.number="form.uid" type="number" placeholder="uid" required />
      <input v-model="form.code" placeholder="code" required />
      <input v-model="form.username" :placeholder="$t('ui.username')" required />
      <input v-model="form.password" type="password" :placeholder="$t('ui.password')" required />
      <button type="submit">{{ $t('ui.claim') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
