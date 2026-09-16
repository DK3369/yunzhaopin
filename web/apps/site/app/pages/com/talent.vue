<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

type PoolRow = {
  id: number
  eid: number
  seeker_uid: number
  remark?: string | null
  ctime_n?: string
  uname?: string
}

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const keyword = ref('')
const picked = ref<number[]>([])
const { data: pool, error, refresh } = await useAsyncData(
  () => `talent-pool-${page.value}-${keyword.value}`,
  () =>
    api.post<{ list: PoolRow[]; total: number }>('/v1/mcenter/talent-pool/list', {
      page: page.value,
      page_size: pageSize,
      keyword: keyword.value || undefined,
    }),
)
const { data: publicResumes } = await useAsyncData('talent-search', () =>
  api.get('/v1/wap/resumes', { page: 1, page_size: 20 }),
)
const msg = ref('')
const list = computed<PoolRow[]>(() => pool.value?.list || [])
const total = computed(() => inferTotal(pool.value))
const inviteUid = ref(0)
const inviteJob = ref(0)
const poolRows = computed(() =>
  list.value.map((row) => ({
    key: row.id,
    name: String(row.uname || row.seeker_uid),
    time: row.ctime_n,
    to: `/resumes/${row.seeker_uid}?eid=${row.eid || ''}`,
    info: row.remark ? [row.remark] : [],
  })),
)

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}

async function add(row: { uid: number; eid?: number; def_job?: number }) {
  msg.value = ''
  const eid = Number(row.eid || row.def_job || 0)
  if (!eid) {
    msg.value = t('ui.failed')
    return
  }
  try {
    await api.post('/v1/mcenter/talent-pool', { eid, seeker_uid: row.uid })
    msg.value = t('ui.add_to_talent')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

function addByUid(uid: number) {
  const r = (publicResumes.value?.list || []).find((x: { uid: number }) => Number(x.uid) === uid)
  if (r) return add(r)
}

function pickInvite(row: PoolRow) {
  inviteUid.value = row.seeker_uid
  inviteJob.value = 0
}

const remarkFor = ref(0)
const remarkText = ref('')

function openRemark(row: PoolRow) {
  remarkFor.value = row.id
  remarkText.value = row.remark || ''
}

async function saveRemark() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/talent-pool/remark', {
      id: remarkFor.value,
      remark: remarkText.value,
    })
    remarkFor.value = 0
    remarkText.value = ''
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

async function remove(row: PoolRow) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/talent-pool/delete', { ids: [row.id] })
    if (remarkFor.value === row.id) remarkFor.value = 0
    if (inviteUid.value === row.seeker_uid) inviteUid.value = 0
    picked.value = picked.value.filter((x) => x !== row.id)
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
    await api.post('/v1/mcenter/talent-pool/delete', { ids: picked.value })
    picked.value = []
    remarkFor.value = 0
    inviteUid.value = 0
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

useSeoMeta({ title: t('member_com_00597') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00597')" :error="error && !isUnauthErr(error) ? error : undefined">
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <p class="site-pc">
      <input v-model="keyword" type="search" :placeholder="$t('admin_00149')" @keydown.enter.prevent="go(1)" />
      <button type="button" class="com_topbth" @click="go(1)">{{ $t('common.search') }}</button>
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
    <MemberResumeH1 :title="$t('ui.public_resumes')" />
    <MemberHrResumeRows
      :rows="(publicResumes?.list || []).map((r: Record<string, unknown>) => ({
        key: Number(r.uid),
        name: String(r.display_name || r.name || r.uid || ''),
        time: [r.education_n, r.exp_n].filter(Boolean).join(' · '),
        to: `/resumes/${r.uid}${r.eid || r.def_job ? `?eid=${r.eid || r.def_job}` : ''}`,
        info: [String(r.education_n || ''), String(r.exp_n || '')].filter(Boolean),
        photo: r.photo ? mediaUrl(String(r.photo)) : undefined,
      }))"
    >
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="cblue" @click="addByUid(Number(row.key))">{{ $t('ui.add_to_talent') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="addByUid(Number(row.key))">{{ $t('ui.add_to_talent') }}</div>
      </template>
    </MemberHrResumeRows>
    <MemberResumeH1 :title="$t('ui.favorited')" />
    <MemberHrResumeRows v-model:picked="picked" selectable :rows="poolRows">
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="cblue" @click="pickInvite(list.find((x) => x.id === Number(row.key))!)">{{ $t('wap_com_00046') }}</a>
        <a href="javascript:;" class="cblue" @click="openRemark(list.find((x) => x.id === Number(row.key))!)">{{ $t('wap_com_00069') }}</a>
        <a href="javascript:;" class="List_dete cblue" @click="remove(list.find((x) => x.id === Number(row.key))!)">{{ $t('common.delete') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="pickInvite(list.find((x) => x.id === Number(row.key))!)">{{ $t('wap_com_00046') }}</div>
        <div class="hr_userlist_czicon" @click="openRemark(list.find((x) => x.id === Number(row.key))!)">{{ $t('wap_com_00069') }}</div>
        <div class="hr_userlist_czicon" @click="remove(list.find((x) => x.id === Number(row.key))!)">{{ $t('common.delete') }}</div>
      </template>
    </MemberHrResumeRows>
    <div v-if="poolRows.length" class="com_Release_job_bot">
      <a href="javascript:;" class="c_btn_02" @click="removePicked">{{ $t('common.delete') }}</a>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <MemberComYqmsForm
      v-if="inviteUid"
      :seeker-uid="inviteUid"
      :job-id="inviteJob || undefined"
      @done="inviteUid = 0; msg = $t('wap_00291')"
      @cancel="inviteUid = 0"
    />
    <form v-if="remarkFor" class="com_release_box site-pc" @submit.prevent="saveRemark">
      <ul>
        <MemberReleaseRow :label="$t('wap_00807')" area><textarea v-model="remarkText" rows="3" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.save') }}</button>
      <button type="button" class="btn_01" @click="remarkFor = 0">{{ $t('common.cancel') }}</button>
    </form>
    <div v-if="remarkFor" class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="saveRemark">
        <MemberField wap area :label="$t('wap_00807')"><textarea v-model="remarkText" rows="3" /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.save') }}</button>
        <button type="button" class="issue_post_body_btn" @click="remarkFor = 0">{{ $t('common.cancel') }}</button>
      </form>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
