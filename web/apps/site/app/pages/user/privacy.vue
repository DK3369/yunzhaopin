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
async function changeStatus(v: number) {
  status.value = v
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/status', { status: v })
    msg.value = t('ui.saved')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function saveName() {
  msg.value = ''
  try {
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
  <MemberPanel :title="$t('wap_user_00215')">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <div v-else class="site-h5">
      <div class="privacy_title">
        <p>{{ $t('wap_01144') }}</p>
      </div>
      <div class="privacy_body">
        <ul class="privacy_body_card">
          <li @click="changeStatus(1)">
            <a class="mui-navigate-right">
              <div class="privacy_body_card_title">{{ $t('wap_01105') }}</div>
              <span class="privacy_list_p">{{ $t('wap_01106') }}</span>
            </a>
            <div v-if="status === 1" class="mui-table-pitch-on">
              <img src="/legacy/h5/images/table-view_yes.png" alt="" width="100%" height="100%" />
            </div>
          </li>
          <li @click="changeStatus(3)">
            <a class="mui-navigate-right">
              <div class="privacy_body_card_title">{{ $t('wap_01107') }}</div>
              <span class="privacy_list_p">{{ $t('wap_01108') }}</span>
            </a>
            <div v-if="status === 3" class="mui-table-pitch-on">
              <img src="/legacy/h5/images/table-view_yes.png" alt="" width="100%" height="100%" />
            </div>
          </li>
          <li @click="changeStatus(2)">
            <a class="mui-navigate-right">
              <div class="privacy_body_card_title">{{ $t('wap_01109') }}</div>
              <span class="privacy_list_p">{{ $t('wap_01110') }}</span>
            </a>
            <div v-if="status === 2" class="mui-table-pitch-on">
              <img src="/legacy/h5/images/table-view_yes.png" alt="" width="100%" height="100%" />
            </div>
          </li>
          <li class="privacy_list_af" @click="navigateTo('/user/blacklist')">
            <div>
              <div class="privacy_body_card_title">{{ $t('wap_01111') }}</div>
              <span class="privacy_list_p">{{ $t('wap_01112') }}</span>
            </div>
            <div class="mui-table-pitch-on">
              <img src="/legacy/h5/images/issue_add.png" alt="" width="100%" height="100%" />
            </div>
          </li>
        </ul>
      </div>
    </div>
    <div v-if="!error" class="site-pc account_settings">
      <div class="account_settings_list" @click="changeStatus(1)">
        <div class="account_settings_list_left">
          <div class="account_settings_tit">{{ $t('wap_01105') }}</div>
          {{ $t('wap_01106') }}
        </div>
        <div :class="status === 1 ? 'account_settings_bth' : 'account_settings_bth_hv'">{{ $t('wap_js_00005') }}</div>
      </div>
      <div class="account_settings_list" @click="changeStatus(3)">
        <div class="account_settings_list_left">
          <div class="account_settings_tit">{{ $t('wap_01107') }}</div>
          {{ $t('wap_01108') }}
        </div>
        <div :class="status === 3 ? 'account_settings_bth' : 'account_settings_bth_hv'">{{ $t('ui.company_only_visible') }}</div>
      </div>
      <div class="account_settings_list" @click="changeStatus(2)">
        <div class="account_settings_list_left">
          <div class="account_settings_tit">{{ $t('wap_01109') }}</div>
          {{ $t('wap_01110') }}
        </div>
        <div :class="status === 2 ? 'account_settings_bth' : 'account_settings_bth_hv'">{{ $t('ui.hidden') }}</div>
      </div>
      <form class="form" @submit.prevent="saveName">
        <MemberField :label="$t('wap_00529')">
          <select v-model.number="nametype">
            <option :value="1">{{ $t('wap_00529') }}</option>
            <option :value="2">{{ $t('common_02430') }}</option>
          </select>
        </MemberField>
        <button type="submit" class="verification_form_btn">{{ $t('common.save') }}</button>
      </form>
      <div class="account_settings_list">
        <div class="account_settings_list_left">
          <div class="account_settings_tit">{{ $t('member_user_00044') }}</div>
        </div>
        <NuxtLink to="/user/blacklist" class="account_settings_bth_hv">{{ $t('common.more') }}</NuxtLink>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
