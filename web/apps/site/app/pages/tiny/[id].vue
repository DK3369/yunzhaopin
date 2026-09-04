<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`tiny-${id}`, () => api.get('/v1/wap/tiny-resumes/show', { id }))
const password = ref('')
const msg = ref('')
const owned = ref<Record<string, unknown> | null>(null)
const edit = reactive({
  username: '',
  sex: 1,
  exp: 1,
  job: '',
  mobile: '',
  production: '',
})

async function verify() {
  msg.value = ''
  try {
    const r = await api.post<Record<string, unknown>>('/v1/wap/tiny-resumes/verify', { id, password: password.value })
    owned.value = r
    edit.username = String(r.username || '')
    edit.sex = Number(r.sex || 1)
    edit.exp = Number(r.exp || 1)
    edit.job = String(r.job || '')
    edit.mobile = String(r.mobile || '')
    edit.production = String(r.production || '')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function refresh() {
  msg.value = ''
  try {
    await api.post('/v1/wap/tiny-resumes/refresh', { id, password: password.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/wap/tiny-resumes/update', { id, password: password.value, ...edit })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function remove() {
  msg.value = ''
  try {
    await api.post('/v1/wap/tiny-resumes/delete', { id, password: password.value })
    msg.value = t('common.success')
    await navigateTo('/tiny')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({ title: () => String(data.value?.username || t('wap_js_00066')) })
useHead({ link: [{ rel: 'canonical', href: `/tiny/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.username || $t('common_02409') }}</h1>
    <p v-if="data?.job" class="muted">{{ data.job }} · {{ data.exp }}</p>
    <p v-if="data?.mobile_masked" class="muted">{{ data.mobile_masked }}</p>
    <p v-if="data?.production">{{ data.production }}</p>
    <p v-else-if="!data?.username" class="muted">{{ $t('common_02409') }}</p>
    <form class="form" @submit.prevent="verify">
      <input v-model="password" type="password" :placeholder="$t('wap_user_00371')" required />
      <button type="submit">{{ $t('common.confirm') }}</button>
    </form>
    <form v-if="owned" class="form" @submit.prevent="save">
      <input v-model="edit.username" required />
      <select v-model.number="edit.sex">
        <option :value="1">{{ $t('common_02092') }}</option>
        <option :value="2">{{ $t('common_02069') }}</option>
      </select>
      <input v-model="edit.job" required />
      <input v-model="edit.mobile" required />
      <textarea v-model="edit.production" rows="5" required />
      <button type="submit">{{ $t('common.save') }}</button>
      <button type="button" @click="refresh">{{ $t('common.latest') }}</button>
      <button type="button" @click="remove">{{ $t('common.delete') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </article>
</template>
