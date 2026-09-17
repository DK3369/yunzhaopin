<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = { uid: number; username: string; status: number; login_date_n?: string }

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-sub-accounts', () =>
  api.post<Row[]>('/v1/mcenter/company/sub-accounts/list', {}),
)
const form = reactive({ username: '', password: '' })
const msg = ref('')

async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/sub-accounts/create', { ...form })
    form.username = ''
    form.password = ''
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function setPassword(uid: number) {
  const password = window.prompt(t('member_user_00226'))
  if (!password) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/sub-accounts/update', { uid, password })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function setStatus(uid: number, status: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/sub-accounts/update', { uid, status })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(uid: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/sub-accounts/delete', { uid })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('common_01597') })
</script>

<template>
  <MemberPanel :title="$t('common_01597')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <form class="com_release_box site-pc" @submit.prevent="create">
        <ul>
          <MemberReleaseRow :label="$t('admin_user_00140')" required>
            <input v-model="form.username" required class="com_release_textnew_text" />
          </MemberReleaseRow>
          <MemberReleaseRow :label="$t('wap_user_00371')" required>
            <input v-model="form.password" type="password" required class="com_release_textnew_text" />
          </MemberReleaseRow>
        </ul>
        <button type="submit" class="btn_01">{{ $t('ui.add') }}</button>
      </form>
      <div class="site-h5 issue_post_body">
        <form class="yun_createbox" @submit.prevent="create">
          <MemberField wap :label="$t('admin_user_00140')"><input v-model="form.username" required /></MemberField>
          <MemberField wap :label="$t('wap_user_00371')"><input v-model="form.password" type="password" required /></MemberField>
          <button type="submit" class="issue_post_body_btn">{{ $t('ui.add') }}</button>
        </form>
      </div>
      <table v-if="(data || []).length" class="com_table site-pc">
        <tr>
          <th>{{ $t('admin_user_00140') }}</th>
          <th>{{ $t('member_user_00181') }}</th>
          <th>{{ $t('admin_yunying_00131') }}</th>
          <th>{{ $t('member_user_00048') }}</th>
        </tr>
        <tr v-for="row in data || []" :key="row.uid">
          <td>{{ row.username }}</td>
          <td>{{ row.status === 2 ? $t('admin_user_00138') : $t('admin_user_company_00205') }}</td>
          <td>{{ row.login_date_n }}</td>
          <td>
            <a href="javascript:;" class="com_bth cblue" @click="setPassword(row.uid)">{{ $t('member_user_00226') }}</a>
            <a
              v-if="row.status === 2"
              href="javascript:;"
              class="com_bth cblue"
              @click="setStatus(row.uid, 1)"
            >{{ $t('admin_user_company_00205') }}</a>
            <a v-else href="javascript:;" class="com_bth cblue" @click="setStatus(row.uid, 2)">{{ $t('admin_user_00150') }}</a>
            <a href="javascript:;" class="List_dete cblue" @click="remove(row.uid)">{{ $t('common.delete') }}</a>
          </td>
        </tr>
      </table>
      <div class="site-h5">
        <div v-for="row in data || []" :key="'h5-' + row.uid" class="com_cardlist">
          <div class="com_cardlist_tit">{{ row.username }}</div>
          <div class="com_cardlist_p">
            <span class="com_cardlist_p_name">{{ $t('admin_yunying_00131') }}</span>
            {{ row.login_date_n }}
          </div>
          <div class="com_card_cz">
            <span class="com_bth" @click="setPassword(row.uid)">{{ $t('member_user_00226') }}</span>
            <span v-if="row.status === 2" class="com_bth" @click="setStatus(row.uid, 1)">{{ $t('admin_user_company_00205') }}</span>
            <span v-else class="com_bth" @click="setStatus(row.uid, 2)">{{ $t('admin_user_00150') }}</span>
            <span class="com_card_delete" @click="remove(row.uid)" />
          </div>
        </div>
      </div>
      <p v-if="!(data || []).length" class="muted">{{ $t('ui.no_data') }}</p>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
