<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: logoutSt, error } = await useAsyncData('logout-st', () =>
  api.post<{ pending?: boolean; status?: number }>('/v1/mcenter/account/logout/status', {}),
)
const rename = reactive({ old_password: '', new_username: '' })
const logoutPw = ref('')
const split = reactive({ old_password: '', new_username: '', new_password: '' })
const msg = ref('')
type SessionRow = {
  id: number
  device?: string
  ip?: string
  ip_loc?: string
  login_at_n?: string
  last_seen_at_n?: string
  is_current?: boolean
}
const { data: sessions, refresh: refreshSessions } = await useAsyncData('my-sessions', () =>
  api.post<SessionRow[]>('/v1/mcenter/sessions', {}).catch(() => [] as SessionRow[]),
)
const sessionList = computed(() => (Array.isArray(sessions.value) ? sessions.value : []))
async function doRename() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/account/username', { ...rename })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function applyLogout() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/account/logout/apply', { password: logoutPw.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function doSplit() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/account/split', { ...split })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function revokeSession(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sessions/revoke', { id })
    msg.value = t('common.success')
    await refreshSessions()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function revokeOthers() {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sessions/revoke-others', {})
    msg.value = t('common.success')
    await refreshSessions()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00338') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00338') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <h2>{{ $t('member_user_00220') }}</h2>
    <form class="form" @submit.prevent="doRename">
      <input v-model="rename.old_password" type="password" :placeholder="$t('wap_01097')" required />
      <input v-model="rename.new_username" required />
      <button type="submit">{{ $t('common.save') }}</button>
    </form>
    <h2>{{ $t('wap_user_00338') }}</h2>
    <p v-if="logoutSt?.pending" class="muted">{{ $t('common.yes') }} {{ logoutSt.status }}</p>
    <form class="form" @submit.prevent="applyLogout">
      <input v-model="logoutPw" type="password" :placeholder="$t('wap_01097')" required />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('wap_user_00339') }}</h2>
    <form class="form" @submit.prevent="doSplit">
      <input v-model="split.old_password" type="password" :placeholder="$t('wap_01097')" required />
      <input v-model="split.new_username" required />
      <input v-model="split.new_password" type="password" :placeholder="$t('wap_01099')" required />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('member_user_00058') }}</h2>
    <p v-if="!sessionList.length" class="muted">{{ $t('ui.no_items') }}</p>
    <article v-for="row in sessionList" :key="row.id" class="job-card">
      <h3>{{ row.device || row.ip }} <span v-if="row.is_current" class="muted">{{ $t('common.yes') }}</span></h3>
      <p class="muted">{{ row.ip }} {{ row.ip_loc }} · {{ row.login_at_n || row.last_seen_at_n }}</p>
      <button v-if="!row.is_current" type="button" @click="revokeSession(row.id)">{{ $t('common.delete') }}</button>
    </article>
    <p v-if="sessionList.length > 1">
      <button type="button" @click="revokeOthers">{{ $t('model_00093') }}</button>
    </p>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
