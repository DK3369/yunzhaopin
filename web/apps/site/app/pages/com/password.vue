<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ old_password: '', new_password: '', confirm: '' })
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
  if (form.new_password && form.confirm && form.new_password !== form.confirm) {
    msg.value = t('wap_01104')
    return
  }
  try {
    await api.post('/v1/mcenter/password', { old_password: form.old_password, new_password: form.new_password })
    msg.value = t('common.success')
    form.old_password = ''
    form.new_password = ''
    form.confirm = ''
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
  <MemberPanel :title="$t('member_com_00070')">
    <template #pcTabs>
      <div class="newmember_tit">
        <ul>
          <li><NuxtLink to="/com/binding">{{ $t('wap_user_00340') }}</NuxtLink></li>
          <li class="newmember_titcur"><a href="javascript:;">{{ $t('member_com_00070') }}</a></li>
          <li><NuxtLink to="/com/set">{{ $t('member_com_00538') }}</NuxtLink></li>
        </ul>
      </div>
    </template>
    <div class="admincont_box">
      <form class="site-pc" @submit.prevent="submit">
        <div class="admin_password">
          <span class="text_s_left">{{ $t('member_com_00381') }}：</span>
          <input v-model="form.old_password" type="password" class="com_info_text" />
          <span class="vs_right_span">{{ $t('member_com_00671') }}</span>
        </div>
        <div class="admin_password">
          <span class="text_s_left">{{ $t('wap_user_00305') }}：</span>
          <input v-model="form.new_password" type="password" class="com_info_text" />
        </div>
        <div class="admin_password">
          <span class="text_s_left">{{ $t('wap_com_00343') }}：</span>
          <input v-model="form.confirm" type="password" class="com_info_text" />
          <span class="vs_right_span">{{ $t('member_user_00580') }}</span>
        </div>
        <div class="admin_password">
          <span class="text_s_left">&nbsp;</span>
          <input type="submit" class="btn_01" :value="$t('member_com_00672')" />
        </div>
      </form>
    </div>
    <form class="site-h5 verification_form security" @submit.prevent="submit">
      <MemberField :label="$t('wap_01097')"><input v-model="form.old_password" type="password" /></MemberField>
      <MemberField :label="$t('wap_01099')"><input v-model="form.new_password" type="password" /></MemberField>
      <MemberField :label="$t('wap_com_00343')"><input v-model="form.confirm" type="password" /></MemberField>
      <button type="submit" class="btn_01">{{ $t('common.submit') }}</button>
    </form>
    <div v-if="sessionList.length" class="site-pc">
      <div v-for="row in sessionList" :key="row.id" class="sysynews_list">
        <div class="sysynews_span sysynews_name">
          {{ row.device || row.ip }}
          <span v-if="row.is_current" class="muted">{{ $t('common.yes') }}</span>
        </div>
        <div class="sysynews_span sysynews_time">{{ row.ip }} {{ row.ip_loc }} · {{ row.login_at_n || row.last_seen_at_n }}</div>
        <div class="sysynews_span sysynews_cz">
          <a v-if="!row.is_current" href="javascript:;" class="com_bth" @click="revokeSession(row.id)">{{ $t('common.delete') }}</a>
        </div>
      </div>
      <a href="javascript:;" class="com_bth" @click="revokeOthers">{{ $t('member_com_00083') }}</a>
    </div>
    <div class="site-h5 m_cardbox">
      <MemberSxNewsCard
        v-for="row in sessionList"
        :key="'h5-' + row.id"
        :title="row.device || row.ip || ''"
        :sub="row.ip_loc"
        :time="row.login_at_n || row.last_seen_at_n"
      />
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
