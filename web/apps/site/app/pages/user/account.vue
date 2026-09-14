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
const applyUt = ref(2)
const applyBody = ref('')
const { data: utSt, refresh: refreshUt } = await useAsyncData('usertype-st', () =>
  api.post<{ pending?: boolean; apply_usertype?: number }>('/v1/mcenter/account/usertype/status', {}).catch(() => null),
)
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
async function applyUsertype() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/account/usertype/apply', {
      apply_usertype: applyUt.value,
      apply_body: applyBody.value,
    })
    msg.value = t('common.success')
    await refreshUt()
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
  <MemberPanel :title="$t('wap_user_00338')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form class="form verification_form" @submit.prevent="doRename">
      <MemberField :label="$t('wap_01097')">
        <input v-model="rename.old_password" type="password" required />
      </MemberField>
      <MemberField :label="$t('member_user_00220')">
        <input v-model="rename.new_username" required />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('common.save') }}</button>
    </form>
    <MemberResumeH1 :title="$t('wap_user_00338')" />
    <div v-if="logoutSt?.pending" class="msg_no">
      <p>{{ $t('wap_00749') }}</p>
      <p>{{ $t('wap_00750') }}</p>
    </div>
    <template v-else>
      <div class="wxts_box">
        <div class="wxts">{{ $t('wap_user_00205') }}</div>
        {{ $t('admin_user_00191') }}
      </div>
      <form class="form verification_form" @submit.prevent="applyLogout">
        <div class="Binding_pop_box_list">
          <input
            v-model="logoutPw"
            type="password"
            class="Binding_pop_box_list_text Binding_pop_box_list_textw200"
            :placeholder="$t('wap_js_00139')"
            required
          />
        </div>
        <button type="submit" class="msg_no_sq uesr_submit">{{ $t('member_user_00521') }}</button>
      </form>
    </template>
    <h2>{{ $t('wap_user_00339') }}</h2>
    <form class="form verification_form" @submit.prevent="doSplit">
      <MemberField :label="$t('wap_01097')"><input v-model="split.old_password" type="password" required /></MemberField>
      <MemberField :label="$t('member_user_00220')"><input v-model="split.new_username" required /></MemberField>
      <MemberField :label="$t('wap_01099')"><input v-model="split.new_password" type="password" required /></MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('admin_user_00162') }}</h2>
    <form class="form verification_form" @submit.prevent="applyUsertype">
      <MemberField :label="$t('admin_user_00162')">
        <select v-model.number="applyUt">
          <option :value="1">{{ $t('common.resume') }}</option>
          <option :value="2">{{ $t('common.company') }}</option>
        </select>
      </MemberField>
      <MemberField :label="$t('wap_user_00203')"><input v-model="applyBody" /></MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="utSt?.pending" class="muted">{{ $t('common.yes') }}</p>
    <h2>{{ $t('member_user_00058') }}</h2>
    <div v-if="!sessionList.length" class="msg_no"><p>{{ $t('ui.no_items') }}</p></div>
    <div v-for="row in sessionList" :key="row.id" class="attention_enterprises_list site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">
        {{ row.device || row.ip }}
        <span v-if="row.is_current">{{ $t('common.yes') }}</span>
      </div>
      <div class="attention_enterprises_span attention_enterprises_time">{{ row.ip }} {{ row.ip_loc }} · {{ row.login_at_n || row.last_seen_at_n }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">
        <a v-if="!row.is_current" href="javascript:;" class="cblue" @click="revokeSession(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in sessionList"
          :key="'h5-' + row.id"
          variant="issue"
          :title="row.device || row.ip || ''"
          :sub="row.ip_loc"
          :time="row.login_at_n || row.last_seen_at_n"
        />
      </div>
    </div>
    <p v-if="sessionList.length > 1">
      <button type="button" class="verification_form_btn" @click="revokeOthers">{{ $t('model_00093') }}</button>
    </p>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
