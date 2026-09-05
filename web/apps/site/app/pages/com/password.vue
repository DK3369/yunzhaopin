<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ old_password: '', new_password: '' })
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
const { data: sessions, refresh: refreshSessions } = await useAsyncData('com-sessions', () =>
  api.post<SessionRow[]>('/v1/mcenter/sessions', {}).catch(() => [] as SessionRow[]),
)
const sessionList = computed(() => (Array.isArray(sessions.value) ? sessions.value : []))
function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}
async function submit() {
  try {
    await api.post('/v1/mcenter/password', { ...form })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function revokeSession(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sessions/revoke', { id })
    msg.value = t('common.success')
    await refreshSessions()
  } catch (e: unknown) {
    msg.value = fail(e)
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
    msg.value = fail(e)
  }
}
useSeoMeta({ title: t('member_user_00226') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00226')">
    <form class="form" @submit.prevent="submit">
      <input v-model="form.old_password" type="password" :placeholder="$t('wap_01097')" />
      <input v-model="form.new_password" type="password" :placeholder="$t('wap_01099')" />
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
  </MemberPanel>
</template>
