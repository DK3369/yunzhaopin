<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type HrRow = { hr_uid: number; username?: string; role?: string; status: number; joined_at_n?: string }
type CodeRow = {
  id: number
  code: string
  note?: string
  max_uses?: number
  used_count?: number
  remaining?: number
  expires_at?: number
  expires_at_n?: string
  status: number
}
type CoRow = { company_uid: number; company_name?: string; role?: string; joined_at?: number }

const api = useApi()
const { t } = useI18n()
const route = useRoute()
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
const maxUses = ref(0)
const expiresLocal = ref('')
const joinCode = ref(String(route.query.code || ''))
const msg = ref('')

function expiresUnix() {
  const s = expiresLocal.value.trim()
  if (!s) return 0
  const t = Date.parse(s)
  return Number.isFinite(t) ? Math.floor(t / 1000) : 0
}

function remainText(c: CodeRow) {
  if (c.max_uses === 0 || c.remaining === -1) return t('common.not_limited')
  if (c.remaining != null) return String(c.remaining)
  const max = Number(c.max_uses || 0)
  const used = Number(c.used_count || 0)
  return String(Math.max(0, max - used))
}

async function copyCode(code: string) {
  msg.value = ''
  try {
    if (import.meta.client && navigator.clipboard) {
      await navigator.clipboard.writeText(code)
      msg.value = t('common.success')
    }
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

async function createCode() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/invite-codes', {
      note: note.value,
      max_uses: maxUses.value,
      expires_at: expiresUnix(),
    })
    note.value = ''
    maxUses.value = 0
    expiresLocal.value = ''
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

watch(
  () => String(route.query.code || ''),
  (c) => {
    if (c) joinCode.value = c
  },
)

useSeoMeta({ title: t('ui.hr') })
</script>

<template>
  <MemberPanel :title="$t('ui.hr')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <form class="com_release_box site-pc" @submit.prevent="createCode">
        <ul>
          <MemberReleaseRow :label="$t('ui.desc')"><input v-model="note" class="com_release_textnew_text" /></MemberReleaseRow>
          <MemberReleaseRow :label="$t('ui.qty')">
            <input v-model.number="maxUses" type="number" min="0" class="com_release_textnew_text" />
            <span class="muted">{{ $t('common.not_limited') }} 0</span>
          </MemberReleaseRow>
          <MemberReleaseRow :label="$t('ui.expire')">
            <input v-model="expiresLocal" type="datetime-local" class="com_release_textnew_text" />
          </MemberReleaseRow>
        </ul>
        <button type="submit" class="btn_01">{{ $t('ui.add') }}</button>
      </form>
      <div class="site-h5 issue_post_body">
        <form class="yun_createbox" @submit.prevent="createCode">
          <MemberField wap :label="$t('ui.desc')"><input v-model="note" /></MemberField>
          <MemberField wap :label="$t('ui.qty')"><input v-model.number="maxUses" type="number" min="0" /></MemberField>
          <MemberField wap :label="$t('ui.expire')"><input v-model="expiresLocal" type="datetime-local" /></MemberField>
          <button type="submit" class="issue_post_body_btn">{{ $t('ui.add') }}</button>
        </form>
      </div>
      <table class="com_table site-pc">
        <tr>
          <th>{{ $t('ui.invite') }}</th>
          <th>{{ $t('ui.qty') }}</th>
          <th>{{ $t('ui.expire') }}</th>
          <th>{{ $t('member_user_00048') }}</th>
        </tr>
        <tr v-for="c in codes || []" :key="c.id">
          <td>{{ c.code }} · {{ c.note }}</td>
          <td>{{ remainText(c) }}</td>
          <td>{{ c.expires_at ? c.expires_at_n : $t('common.not_limited') }}</td>
          <td>
            <a href="javascript:;" class="cblue" @click="copyCode(c.code)">{{ $t('wap_com_00232') }}</a>
            ·
            <a href="javascript:;" class="List_dete cblue" @click="revoke(c.id)">{{ $t('common.delete') }}</a>
          </td>
        </tr>
      </table>
      <div class="site-h5">
        <div v-for="c in codes || []" :key="'h5c-' + c.id" class="com_cardlist">
          <div class="com_cardlist_tit">{{ c.code }} · {{ c.note || '' }}</div>
          <div class="com_cardlist_p">
            <span class="com_cardlist_p_name">{{ $t('ui.qty') }}</span>
            {{ remainText(c) }}
          </div>
          <div class="com_cardlist_p">
            <span class="com_cardlist_p_name">{{ $t('ui.expire') }}</span>
            {{ c.expires_at ? c.expires_at_n : $t('common.not_limited') }}
          </div>
          <div class="com_card_cz">
            <a href="javascript:;" class="cblue" @click="copyCode(c.code)">{{ $t('wap_com_00232') }}</a>
            <span class="com_card_delete" @click="revoke(c.id)" />
          </div>
        </div>
      </div>
      <MemberResumeH1 :title="$t('ui.hr')" />
      <table v-if="(hrs || []).length" class="com_table site-pc">
        <tr>
          <th>{{ $t('ui.username') }}</th>
          <th>{{ $t('ui.time') }}</th>
          <th>{{ $t('member_user_00048') }}</th>
        </tr>
        <tr v-for="row in hrs || []" :key="row.hr_uid">
          <td>{{ row.username || row.hr_uid }} · {{ row.role }}</td>
          <td>{{ row.joined_at_n }}</td>
          <td><a href="javascript:;" class="com_bth cblue" @click="removeHr(row.hr_uid)">{{ $t('common.delete') }}</a></td>
        </tr>
      </table>
      <div class="site-h5">
        <div v-for="row in hrs || []" :key="'h5-' + row.hr_uid" class="com_cardlist">
          <div class="com_cardlist_tit">{{ row.username || row.hr_uid }} · {{ row.role || '' }}</div>
          <div class="com_cardlist_p">
            <span class="com_cardlist_p_name">{{ $t('ui.time') }}</span>
            {{ row.joined_at_n }}
          </div>
          <div class="com_card_cz">
            <span class="com_card_delete" @click="removeHr(row.hr_uid)" />
          </div>
        </div>
      </div>
      <form class="com_release_box site-pc" @submit.prevent="join">
        <ul>
          <MemberReleaseRow :label="$t('ui.invite')" required>
            <input v-model="joinCode" required class="com_release_textnew_text" />
          </MemberReleaseRow>
        </ul>
        <button type="submit" class="btn_01">{{ $t('common.submit') }}</button>
      </form>
      <div class="site-h5 issue_post_body">
        <form class="yun_createbox" @submit.prevent="join">
          <MemberField wap :label="$t('ui.invite')"><input v-model="joinCode" required /></MemberField>
          <button type="submit" class="issue_post_body_btn">{{ $t('common.submit') }}</button>
        </form>
      </div>
      <table v-if="(companies || []).length" class="com_table site-pc">
        <tr>
          <th>{{ $t('ui.company_name') }}</th>
          <th>{{ $t('member_user_00181') }}</th>
        </tr>
        <tr v-for="co in companies || []" :key="co.company_uid">
          <td>{{ co.company_name || co.company_uid }}</td>
          <td>{{ co.role }}</td>
        </tr>
      </table>
      <div class="site-h5">
        <div v-for="co in companies || []" :key="'co-' + co.company_uid" class="com_cardlist">
          <div class="com_cardlist_tit">{{ co.company_name || co.company_uid }}</div>
          <div class="com_cardlist_p">
            <span class="com_cardlist_p_name">{{ $t('member_user_00181') }}</span>
            {{ co.role }}
          </div>
        </div>
      </div>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
