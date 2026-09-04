<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('privacy-resume', () =>
  api.post<{ status?: number; nametype?: number }>('/v1/mcenter/resume/list', {}),
)
const status = ref(1)
const nametype = ref(1)
watch(
  data,
  (row) => {
    if (!row) return
    status.value = Number(row.status || 1)
    nametype.value = Number(row.nametype || 1)
  },
  { immediate: true },
)
const msg = ref('')
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/status', { status: status.value })
    await api.post('/v1/mcenter/resume', { nametype: nametype.value })
    msg.value = t('ui.saved')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00215') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00215') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form v-else class="form" @submit.prevent="save">
      <p class="muted">{{ $t('ui.privacy_hint') }}</p>
      <select v-model.number="status">
        <option :value="1">{{ $t('wap_js_00005') }}</option>
        <option :value="2">{{ $t('ui.hidden') }}</option>
        <option :value="3">{{ $t('ui.company_only_visible') }}</option>
      </select>
      <select v-model.number="nametype">
        <option :value="1">{{ $t('wap_00529') }}</option>
        <option :value="2">{{ $t('common_02430') }}</option>
      </select>
      <button type="submit">{{ $t('common.save') }}</button>
    </form>
    <p><NuxtLink to="/user/blacklist">{{ $t('member_user_00044') }}</NuxtLink></p>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
