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
  <section>
    <h1>{{ $t('ui.hr') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <form class="form" @submit.prevent="createCode">
        <input v-model="note" :placeholder="$t('ui.desc')" />
        <button type="submit">{{ $t('ui.add') }}</button>
      </form>
      <article v-for="c in codes || []" :key="c.id" class="job-card">
        <p>{{ c.code }} · {{ c.note }}</p>
        <p class="muted">{{ c.expires_at_n }} · {{ c.remaining }}</p>
        <button type="button" @click="revoke(c.id)">{{ $t('common.delete') }}</button>
      </article>
      <h2>{{ $t('ui.hr') }}</h2>
      <p v-if="!(hrs || []).length" class="muted">{{ $t('ui.no_data') }}</p>
      <article v-for="row in hrs || []" :key="row.hr_uid" class="job-card">
        <p>{{ row.hr_uid }} · {{ row.role }}</p>
        <p class="muted">{{ row.joined_at_n }}</p>
        <button type="button" @click="removeHr(row.hr_uid)">{{ $t('common.delete') }}</button>
      </article>
      <h2>{{ $t('common.submit') }}</h2>
      <form class="form" @submit.prevent="join">
        <input v-model="joinCode" required />
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <article v-for="co in companies || []" :key="co.company_uid" class="job-card">
        <p>{{ co.company_uid }} · {{ co.role }}</p>
      </article>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </section>
</template>
