<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('user-ident', () =>
  api
    .post<{
      idcard?: string
      idcard_pic?: string
      idcard_status?: number
      telphone?: string
      email?: string
      moblie_status?: number
      email_status?: number
    }>('/v1/mcenter/resume/list', {})
    .catch(() => null),
)
const idcard = ref('')
const idcardPic = ref('')
watch(
  data,
  (row) => {
    idcard.value = String(row?.idcard || '')
    idcardPic.value = String(row?.idcard_pic || '')
  },
  { immediate: true },
)
const msg = ref('')
function statusLabel(st?: number, hasPic?: string) {
  if (st === 1) return t('wap_user_00128')
  if (st === 2) return t('wap_user_00167')
  if (hasPic) return t('wap_user_00178')
  return t('wap_user_00175')
}
async function onPic(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  msg.value = ''
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/cert', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    idcardPic.value = r.key || r.url
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume', { idcard: idcard.value, idcard_pic: idcardPic.value })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00340') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00340') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form v-else class="form" @submit.prevent="save">
      <h2>{{ $t('wap_01030') }}</h2>
      <p class="muted">{{ statusLabel(data?.idcard_status, data?.idcard_pic) }}</p>
      <input v-model="idcard" :placeholder="$t('wap_01087')" :disabled="data?.idcard_status === 1" />
      <img v-if="idcardPic" :src="mediaUrl(idcardPic)" alt="" width="160" />
      <input v-if="data?.idcard_status !== 1" type="file" accept="image/jpeg,image/png,image/webp" @change="onPic" />
      <button v-if="data?.idcard_status !== 1" type="submit">{{ $t('wap_user_00176') }}</button>
    </form>
    <article class="job-card">
      <h3>{{ $t('wap_user_00180') }}</h3>
      <p>{{ data?.telphone || $t('wap_user_00177') }} · {{ data?.moblie_status === 1 ? $t('wap_user_00127') : $t('wap_user_00182') }}</p>
      <NuxtLink to="/user/binding">{{ $t('wap_00389') }}</NuxtLink>
    </article>
    <article class="job-card">
      <h3>{{ $t('wap_user_00179') }}</h3>
      <p>{{ data?.email || $t('wap_user_00177') }} · {{ data?.email_status === 1 ? $t('wap_user_00127') : $t('wap_user_00181') }}</p>
      <NuxtLink to="/user/binding">{{ $t('wap_00389') }}</NuxtLink>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
