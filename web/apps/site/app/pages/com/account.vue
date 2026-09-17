<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: me } = await useAuthMe()
const { data: logoutSt, error } = await useAsyncData('com-logout-st', () =>
  api.post<{ pending?: boolean; status?: number }>('/v1/mcenter/account/logout/status', {}),
)
const rename = reactive({ old_password: '', new_username: '' })
const logoutPw = ref('')
const msg = ref('')
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
useSeoMeta({ title: t('wap_user_00338') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00338')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="com_release_box site-pc" @submit.prevent="doRename">
      <ul>
        <MemberReleaseRow :label="$t('wap_01097')" required>
          <input v-model="rename.old_password" type="password" required class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00220')" required>
          <input v-model="rename.new_username" required class="com_release_textnew_text" />
        </MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.save') }}</button>
    </form>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="doRename">
        <MemberField wap :label="$t('wap_01097')"><input v-model="rename.old_password" type="password" required /></MemberField>
        <MemberField wap :label="$t('member_user_00220')"><input v-model="rename.new_username" required /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.save') }}</button>
      </form>
    </div>
    <div v-if="logoutSt?.pending && !me?.is_sub" class="com_msg_no">
      <p>{{ $t('wap_00749') }}</p>
      <p>{{ $t('wap_00750') }}</p>
    </div>
    <template v-else-if="!me?.is_sub">
      <form class="com_release_box site-pc" @submit.prevent="applyLogout">
        <ul>
          <MemberReleaseRow :label="$t('wap_js_00139')" required>
            <input v-model="logoutPw" type="password" required class="com_release_textnew_text" />
          </MemberReleaseRow>
        </ul>
        <button type="submit" class="btn_01">{{ $t('member_user_00521') }}</button>
      </form>
      <div class="site-h5 issue_post_body">
        <form class="yun_createbox" @submit.prevent="applyLogout">
          <MemberField wap :label="$t('wap_js_00139')"><input v-model="logoutPw" type="password" required /></MemberField>
          <button type="submit" class="issue_post_body_btn">{{ $t('member_user_00521') }}</button>
        </form>
      </div>
    </template>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
