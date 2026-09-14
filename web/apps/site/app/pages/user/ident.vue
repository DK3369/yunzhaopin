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
  <MemberPanel :title="$t('wap_user_00340')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form v-else class="form verification_form" @submit.prevent="save">
      <MemberResumeH1 :title="$t('wap_01030')" />
      <p class="muted">{{ statusLabel(data?.idcard_status, data?.idcard_pic) }}</p>
      <MemberField :label="$t('wap_01087')">
        <input v-model="idcard" :disabled="data?.idcard_status === 1" />
      </MemberField>
      <img v-if="idcardPic" :src="mediaUrl(idcardPic)" alt="" width="160" />
      <input v-if="data?.idcard_status !== 1" type="file" accept="image/jpeg,image/png,image/webp" @change="onPic" />
      <button v-if="data?.idcard_status !== 1" type="submit" class="verification_form_btn">{{ $t('wap_user_00176') }}</button>
    </form>
    <div class="account_settings_list">
      <div class="account_settings_list_left">
        <i class="account_settings_list_left_icon account_settings_list_left_icon_sj" />
        <div class="account_settings_tit">{{ $t('wap_user_00180') }}</div>
        {{ data?.telphone || $t('wap_user_00177') }} · {{ data?.moblie_status === 1 ? $t('wap_user_00127') : $t('wap_user_00182') }}
      </div>
      <NuxtLink to="/user/binding" class="account_settings_bth_hv">{{ $t('wap_00389') }}</NuxtLink>
    </div>
    <div class="account_settings_list">
      <div class="account_settings_list_left">
        <i class="account_settings_list_left_icon account_settings_list_left_icon_yx" />
        <div class="account_settings_tit">{{ $t('wap_user_00179') }}</div>
        {{ data?.email || $t('wap_user_00177') }} · {{ data?.email_status === 1 ? $t('wap_user_00127') : $t('wap_user_00181') }}
      </div>
      <NuxtLink to="/user/binding" class="account_settings_bth_hv">{{ $t('wap_00389') }}</NuxtLink>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
