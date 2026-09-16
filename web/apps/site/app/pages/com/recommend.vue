<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = { uid: number; display_name: string; sex?: number; education?: number; lastupdate?: number; eid?: number; def_job?: number }

const api = useApi()
const { t } = useI18n()
const inviteUid = ref(0)
const { data, error } = await useAsyncData('com-rec-resumes', () =>
  api.post<Row[]>('/v1/mcenter/recommend/resumes', { limit: 40 }).catch(() => [] as Row[]),
)
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as Row[])
const msg = ref('')
async function addTalent(row: Row) {
  msg.value = ''
  const eid = Number(row.eid || row.def_job || 0)
  if (!eid) {
    msg.value = t('ui.failed')
    return
  }
  try {
    await api.post('/v1/mcenter/talent-pool', { eid, seeker_uid: row.uid })
    msg.value = t('ui.add_to_talent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function addTalentByUid(uid: number) {
  const row = list.value.find((x) => x.uid === uid)
  if (row) return addTalent(row)
}
useSeoMeta({ title: t('wap_user_00211') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00211')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <MemberHrResumeRows
      :rows="list.map((row) => ({
        key: row.uid,
        name: String(row.display_name || row.uid),
        to: `/resumes/${row.uid}${row.eid || row.def_job ? `?eid=${row.eid || row.def_job}` : ''}`,
      }))"
    >
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="cblue" @click="inviteUid = Number(row.key)">{{ $t('wap_com_00046') }}</a>
        <a href="javascript:;" class="cblue" @click="addTalentByUid(Number(row.key))">{{ $t('ui.add_to_talent') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="inviteUid = Number(row.key)">{{ $t('wap_com_00046') }}</div>
        <div class="hr_userlist_czicon" @click="addTalentByUid(Number(row.key))">{{ $t('ui.add_to_talent') }}</div>
      </template>
    </MemberHrResumeRows>
    <MemberComYqmsForm v-if="inviteUid" :seeker-uid="inviteUid" @done="inviteUid = 0" @cancel="inviteUid = 0" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
