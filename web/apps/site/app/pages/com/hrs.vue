<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type HrRow = { hr_uid: number; role?: string; status: number; joined_at_n?: string }
type CodeRow = { id: number; code: string; note?: string; remaining?: number; expires_at_n?: string; status: number }
type CoRow = { company_uid: number; role?: string; joined_at?: number }

const api = useApi()
const { t } = useI18n()
const { data: hrs, error, refresh: refreshHrs } = await useAsyncData('com-hrs', () =>
  api.post<HrRow[]>('/v1/mcenter/company/hrs', {}).catch(() => [] as HrRow[]),
)
const { data: codes, refresh: refreshCodes } = await useAsyncData('com-hr-codes', () =>
  api.post<CodeRow[]>('/v1/mcenter/company/invite-codes/list', {}).catch(() => [] as CodeRow[]),
)
const { data: companies, refresh: refreshCos } = await useAsyncData('com-my-companies', () =>
  api.post<CoRow[]>('/v1/mcenter/company/my-companies', {}).catch(() => [] as CoRow[]),
)
const note = ref('')
const joinCode = ref('')
const msg = ref('')

async function createCode() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/invite-codes', { note: note.value })
    note.value = ''
    msg.value = t('common.success')
    await refreshCodes()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function revoke(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  try {
    await api.post('/v1/mcenter/company/invite-codes/revoke', { id })
    await refreshCodes()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function removeHr(uid: number) {
  if (!window.confirm(t('member_com_00083'))) return
  try {
    await api.post('/v1/mcenter/company/hrs/remove', { uid })
    await refreshHrs()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function join() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/join', { code: joinCode.value })
    joinCode.value = ''
    msg.value = t('common.success')
    await refreshCos()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('ui.hr') })
</script>

<template>
  <MemberPanel :title="$t('ui.hr')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <form class="com_release_box" @submit.prevent="createCode">
        <ul>
          <MemberReleaseRow :label="$t('ui.desc')"><input v-model="note" class="com_release_textnew_text" /></MemberReleaseRow>
        </ul>
        <button type="submit" class="verification_form_btn">{{ $t('ui.add') }}</button>
      </form>
      <table class="com_table site-pc">
        <tr>
          <th>{{ $t('ui.hr') }}</th>
          <th>{{ $t('member_user_00106') }}</th>
          <th>{{ $t('member_user_00048') }}</th>
        </tr>
        <tr v-for="c in codes || []" :key="c.id">
          <td>{{ c.code }} · {{ c.note }}</td>
          <td>{{ c.expires_at_n }}</td>
          <td><a href="javascript:;" class="List_dete cblue" @click="revoke(c.id)">{{ $t('common.delete') }}</a></td>
        </tr>
      </table>
      <MemberResumeH1 :title="$t('ui.hr')" />
      <div v-for="row in hrs || []" :key="row.hr_uid" class="sysynews_list site-pc">
        <div class="sysynews_span sysynews_name">{{ row.hr_uid }} · {{ row.role }}</div>
        <div class="sysynews_span sysynews_time">{{ row.joined_at_n }}</div>
        <div class="sysynews_span sysynews_cz">
          <a href="javascript:;" class="cblue" @click="removeHr(row.hr_uid)">{{ $t('common.delete') }}</a>
        </div>
      </div>
      <div class="site-h5 m_cardbox">
        <MemberSxNewsCard
          v-for="row in hrs || []"
          :key="'h5-' + row.hr_uid"
          :title="`${row.hr_uid} · ${row.role || ''}`"
          :time="row.joined_at_n"
        />
      </div>
      <form class="com_release_box" @submit.prevent="join">
        <ul>
          <MemberReleaseRow :label="$t('ui.hr')" required><input v-model="joinCode" required class="com_release_textnew_text" /></MemberReleaseRow>
        </ul>
        <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
      </form>
      <div v-for="co in companies || []" :key="co.company_uid" class="sysynews_list site-pc">
        <div class="sysynews_span sysynews_name">{{ co.company_uid }} · {{ co.role }}</div>
      </div>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
