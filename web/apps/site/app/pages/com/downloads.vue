<script setup lang="ts">
import { mediaUrl } from '~/utils/site'

type DownRow = {
  id: number
  uid: number
  eid?: number
  uname?: string
  datetime_n?: string
  photo?: string
  sex_n?: string
  edu_n?: string
  exp_n?: string
  salary?: string
}

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const keyword = ref('')
const { data, error, refresh } = await useAsyncData(
  () => `downloads-${page.value}-${keyword.value}`,
  () =>
    api.post<{ list: DownRow[]; total: number }>('/v1/mcenter/resume-downloads/outbox', {
      page: page.value,
      page_size: pageSize,
      keyword: keyword.value || undefined,
    }),
)
const list = computed(() => data.value?.list || [])
const rows = computed(() =>
  list.value.map((row) => ({
    key: Number(row.id || row.uid),
    name: String(row.uname || row.uid || ''),
    time: String(row.datetime_n || ''),
    to: `/resumes/${row.uid}?eid=${row.eid || ''}`,
    downloaded: true,
    photo: row.photo ? mediaUrl(String(row.photo)) : undefined,
    info: [row.sex_n, row.edu_n, row.exp_n].map((x) => String(x || '')).filter(Boolean),
    salary: row.salary ? String(row.salary) : undefined,
  })),
)
const total = computed(() => inferTotal(data.value))
const msg = ref('')
const inviteUid = ref(0)
const picked = ref<number[]>([])
function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}

async function exportCsv() {
  msg.value = ''
  try {
    const r = await api.post<{ csv: string; filename?: string }>('/v1/mcenter/resume-downloads/export', {})
    const blob = new Blob([r.csv || ''], { type: 'text/csv;charset=utf-8' })
    const a = document.createElement('a')
    a.href = URL.createObjectURL(blob)
    a.download = r.filename || 'resume-downloads.csv'
    a.click()
    URL.revokeObjectURL(a.href)
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

async function remove(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-downloads/delete', { ids: [id] })
    picked.value = picked.value.filter((x) => x !== id)
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function removePicked() {
  if (!picked.value.length) {
    msg.value = t('common_01164')
    return
  }
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-downloads/delete', { ids: picked.value })
    picked.value = []
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

function pickInvite(id: number) {
  const row = list.value.find((r) => r.id === id)
  if (!row) return
  inviteUid.value = row.uid
}

const remarkFor = ref<DownRow | null>(null)
const remarkText = ref('')
const remarkStatus = ref(2)
const remarkStates = [1, 2, 3, 4, 5, 7]
function browseLabel(s?: number) {
  const map: Record<number, string> = {
    1: t('wap_user_00260'),
    2: t('wap_user_00258'),
    3: t('wap_user_00266'),
    4: t('wap_user_00354'),
    5: t('member_com_00108'),
    7: t('wap_user_00356'),
  }
  return map[Number(s)] ?? String(s ?? '')
}
async function openRemark(id: number) {
  const row = list.value.find((r) => r.id === id)
  if (!row) return
  remarkFor.value = row
  remarkText.value = ''
  remarkStatus.value = 2
  const hit = await api
    .post<{ note?: string; status?: number } | null>('/v1/mcenter/remarks/get-one', {
      target_uid: row.uid,
      kind: 1,
      eid: row.eid || 0,
    })
    .catch(() => null)
  remarkText.value = hit?.note || ''
  if (hit?.status) remarkStatus.value = Number(hit.status)
}
async function saveRemark() {
  const row = remarkFor.value
  if (!row) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/remarks', {
      target_uid: row.uid,
      target_kind: 1,
      eid: row.eid || 0,
      status: remarkStatus.value,
      note: remarkText.value,
    })
    remarkFor.value = null
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

useSeoMeta({ title: t('wap_com_00235') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00235')" :error="error" :empty="!error && !(data?.list || []).length">
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>
    <p class="muted">{{ $t('wap_com_00235') }} {{ total }}</p>
    <p class="site-pc">
      <input v-model="keyword" type="search" :placeholder="$t('admin_00149')" @keydown.enter.prevent="go(1)" />
      <button type="button" class="com_topbth" @click="go(1)">{{ $t('common.search') }}</button>
      <button type="button" class="com_topbth" @click="exportCsv">{{ $t('admin_01322') }}</button>
    </p>
    <div class="site-h5 com-h5-filters">
      <input
        v-model="keyword"
        type="search"
        class="com-h5-filters__kw"
        :placeholder="$t('admin_00149')"
        @keydown.enter.prevent="go(1)"
      />
      <button type="button" class="issue_post_body_btn" @click="go(1)">{{ $t('common.search') }}</button>
    </div>
    <MemberHrResumeRows v-model:picked="picked" selectable :rows="rows">
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="cblue" @click="pickInvite(Number(row.key))">{{ $t('wap_com_00046') }}</a>
        <a href="javascript:;" class="cblue" @click="openRemark(Number(row.key))">{{ $t('member_user_00242') }}</a>
        <a href="javascript:;" class="List_dete cblue" @click="remove(Number(row.key))">{{ $t('common.delete') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="pickInvite(Number(row.key))">{{ $t('wap_com_00046') }}</div>
        <div class="hr_userlist_czicon" @click="openRemark(Number(row.key))">{{ $t('member_user_00242') }}</div>
        <div class="hr_userlist_czicon" @click="remove(Number(row.key))">{{ $t('common.delete') }}</div>
      </template>
    </MemberHrResumeRows>
    <div v-if="rows.length" class="com_Release_job_bot">
      <a href="javascript:;" class="c_btn_02" @click="removePicked">{{ $t('common.delete') }}</a>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <MemberComYqmsForm
      v-if="inviteUid"
      :seeker-uid="inviteUid"
      @done="inviteUid = 0; msg = $t('wap_00291')"
      @cancel="inviteUid = 0"
    />
    <form v-if="remarkFor" class="com_release_box site-pc" @submit.prevent="saveRemark">
      <ul>
        <MemberReleaseRow :label="$t('member_user_00530')">
          <select v-model.number="remarkStatus">
            <option v-for="s in remarkStates" :key="s" :value="s">{{ browseLabel(s) }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00242')" area><textarea v-model="remarkText" rows="3" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.save') }}</button>
      <button type="button" class="btn_01" @click="remarkFor = null">{{ $t('common.cancel') }}</button>
    </form>
    <div v-if="remarkFor" class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="saveRemark">
        <MemberField wap :label="$t('member_user_00530')">
          <select v-model.number="remarkStatus">
            <option v-for="s in remarkStates" :key="'h5rs-' + s" :value="s">{{ browseLabel(s) }}</option>
          </select>
        </MemberField>
        <MemberField wap area :label="$t('member_user_00242')"><textarea v-model="remarkText" rows="3" /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.save') }}</button>
        <button type="button" class="issue_post_body_btn" @click="remarkFor = null">{{ $t('common.cancel') }}</button>
      </form>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
