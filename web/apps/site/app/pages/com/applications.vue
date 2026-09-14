<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

type Row = {
  id: number
  uid: number
  job_id: number
  eid?: number
  datetime_n?: string
  is_browse?: number
  invited?: boolean
  job_name?: string
  uname?: string
  apply_url?: string
  photo?: string
  sex_n?: string
  exp_n?: string
  edu_n?: string
  age?: number | string
  salary?: string
}
type Counts = {
  total: number
  pending: number
  viewed: number
  to_notify: number
  unsuitable: number
  unreachable: number
  hired: number
}

const api = useApi()
const { t } = useI18n()

const PAGE_SIZE = 20
const page = ref(1)
const state = ref<number | null>(null)
const filters = reactive({ job_id: '', keyword: '', edu: '', exp: '', sex: '', uptime: '', resume_state: '' })

/** Only send filled-in filters: Rust treats an absent key as "no filter". */
function activeFilters() {
  const out: Record<string, string | number> = {}
  for (const [k, v] of Object.entries(filters)) {
    if (String(v).trim() !== '') out[k] = v
  }
  return out
}

const listKey = computed(() =>
  JSON.stringify({ p: page.value, s: state.value, ...activeFilters() }),
)
const { data, error, refresh } = await useAsyncData(
  () => `com-apps-${listKey.value}`,
  () =>
    api.post<{ list: Row[]; total: number }>('/v1/mcenter/applications', {
      page: page.value,
      page_size: PAGE_SIZE,
      ...activeFilters(),
      ...(state.value === null ? {} : { state: state.value }),
    }),
)
const { data: counts, refresh: refreshCounts } = await useAsyncData(
  () => `com-apps-counts-${JSON.stringify(activeFilters())}`,
  () => api.post<Counts>('/v1/mcenter/applications/state-counts', activeFilters()),
)

const { data: myJobs } = await useAsyncData('com-apps-jobs', () =>
  api
    .post<{ list: Array<{ id: number; name: string }> }>('/v1/mcenter/jobs/list', {
      page: 1,
      page_size: 200,
    })
    .catch(() => ({ list: [] })),
)
const { data: dicts } = await usePublicDicts()
const eduDict = computed(() => dicts.value?.educations_user ?? [])
const expDict = computed(() => dicts.value?.experiences_user ?? [])

const list = computed(() => data.value?.list || [])
const total = computed(() => data.value?.total || 0)
const msg = ref('')

async function reload() {
  await Promise.all([refresh(), refreshCounts()])
}
function applyFilters() {
  page.value = 1
  selected.value = []
  return reload()
}
function resetFilters() {
  Object.assign(filters, { job_id: '', keyword: '', edu: '', exp: '', sex: '', uptime: '', resume_state: '' })
  state.value = null
  return applyFilters()
}
function pickState(v: number | null) {
  state.value = v
  page.value = 1
  selected.value = []
  return refresh()
}
watch(page, () => {
  selected.value = []
  refresh()
})

// ==================== Bulk selection ====================

const selected = ref<number[]>([])
const allChecked = computed({
  get: () => list.value.length > 0 && selected.value.length === list.value.length,
  set: (on: boolean) => {
    selected.value = on ? list.value.map((r) => r.id) : []
  },
})
async function batchRead() {
  if (!selected.value.length) return
  await run(() => api.post('/v1/mcenter/applications/batch-read', { ids: selected.value }))
  selected.value = []
}

// ==================== Row actions ====================

async function run(fn: () => Promise<unknown>) {
  msg.value = ''
  try {
    await fn()
    await reload()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function openResume(row: Row) {
  await api.post('/v1/mcenter/applications/browse', { id: row.id }).catch(() => null)
  await navigateTo(`/resumes/${row.eid || row.uid}`)
}
function setState(id: number, next: number) {
  return run(() => api.post('/v1/mcenter/applications/state', { id, state: next }))
}
function removeRow(id: number) {
  if (!window.confirm(t('member_com_00083'))) return Promise.resolve()
  return run(() => api.post('/v1/mcenter/applications/delete', { id }))
}
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
function rowInfo(row: Row) {
  return [row.sex_n, row.exp_n, row.edu_n, row.age ? String(row.age) : ''].filter(Boolean) as string[]
}

// ==================== Remark ====================

const remarkFor = ref<Row | null>(null)
const remarkText = ref('')
async function openRemark(row: Row) {
  remarkFor.value = row
  remarkText.value = ''
  const hit = await api
    .post<{ note?: string } | null>('/v1/mcenter/remarks/get-one', {
      target_uid: row.uid,
      kind: 3,
    })
    .catch(() => null)
  remarkText.value = hit?.note || ''
}
async function saveRemark() {
  const row = remarkFor.value
  if (!row) return
  await run(() =>
    api.post('/v1/mcenter/remarks', {
      target_uid: row.uid,
      target_kind: 3,
      note: remarkText.value,
    }),
  )
  remarkFor.value = null
}

// ==================== Interview invite ====================

const invite = reactive({
  seeker_uid: 0,
  job_id: 0,
  content: '',
  address: '',
  intertime: '',
  linkman: '',
  linktel: '',
  save_yqmb: false,
})
function pick(row: Row) {
  invite.seeker_uid = row.uid
  invite.job_id = row.job_id
}
async function sendInvite(confirm = false) {
  msg.value = ''
  if (!invite.seeker_uid || !invite.job_id) {
    msg.value = t('common_01153')
    return
  }
  if (!invite.intertime.trim()) {
    msg.value = t('member_com_00681')
    return
  }
  try {
    const res = await api.post<{ status: number; jifen?: number; price?: number }>(
      '/v1/mcenter/company/yqms/create',
      { ...invite, confirm },
    )
    if (res.status === 2) {
      const text = res.jifen
        ? `${t('common_00697')}${res.jifen}${t('common_01935')}?`
        : res.price
          ? `${t('common_00696')}${res.price}${t('common_00757')}?`
          : t('common_00696')
      if (window.confirm(text)) await sendInvite(true)
      return
    }
    msg.value = t('wap_00291')
    await reload()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}

const tabs = computed(() => [
  { v: null, label: t('common.all'), n: counts.value?.total },
  { v: 1, label: t('wap_user_00260'), n: counts.value?.pending },
  { v: 2, label: t('wap_user_00258'), n: counts.value?.viewed },
  { v: 3, label: t('wap_user_00266'), n: counts.value?.to_notify },
  { v: 4, label: t('wap_user_00354'), n: counts.value?.unsuitable },
  { v: 5, label: t('member_com_00108'), n: counts.value?.unreachable },
  { v: 7, label: t('wap_user_00356'), n: counts.value?.hired },
])
const uptimeOpts = computed(() => [
  { v: 1, label: t('common_01940') },
  { v: 3, label: t('admin_user_00179') },
  { v: 7, label: t('wap_00339') },
  { v: 30, label: t('member_com_00368') },
])
const resumeStateOpts = computed(() => [
  { v: 1, label: t('wap_user_00165') },
  { v: 0, label: t('wap_user_00166') },
  { v: 3, label: t('wap_user_00167') },
])
useSeoMeta({ title: t('member_com_00454') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00454')" :error="error && !isUnauthErr(error) ? error : undefined">
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>

    <MemberComScreen
      :tabs="tabs.map((tab) => ({
        value: tab.v,
        label: tab.label,
        on: state === tab.v,
        count: tab.n,
        select: () => pickState(tab.v),
      }))"
      :keyword="filters.keyword"
      searchable
      @update:keyword="filters.keyword = $event"
      @search="applyFilters"
    >
      <select v-model="filters.job_id">
        <option value="">{{ $t('wap_user_00154') }}</option>
        <option v-for="j in myJobs?.list || []" :key="j.id" :value="j.id">{{ j.name }}</option>
      </select>
    </MemberComScreen>

    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <p v-if="msg" class="muted">{{ msg }}</p>
      <table v-if="list.length" class="com_table site-pc">
        <tr>
          <th><label><input v-model="allChecked" type="checkbox" /> {{ $t('wap_js_00074') }}</label></th>
          <th>{{ $t('wap_00456') }}</th>
          <th>{{ $t('wap_com_00288') }}</th>
          <th>{{ $t('member_user_00048') }}</th>
        </tr>
        <tr v-for="row in list" :key="row.id">
          <td>
            <span class="newcom_user_infoheckb">
              <input v-model="selected" type="checkbox" class="newcom_user_infoheck" :value="row.id" />
            </span>
          </td>
          <td>
            <div class="newcom_user_info">
              <div v-if="row.photo" class="newcom_user_pic">
                <img :src="mediaUrl(row.photo)" alt="" />
              </div>
              <div>
                <a href="javascript:;" class="newcom_user_name" @click.prevent="openResume(row)">{{ row.uname || row.uid }}</a>
                <span class="newcom_user_zt">{{ browseLabel(row.is_browse) }}</span>
                <span v-if="row.invited" class="hr_yyy">{{ $t('wap_user_00216') }}</span>
                <div v-if="rowInfo(row).length" class="newcom_user_infop">{{ rowInfo(row).join(' · ') }}</div>
                <div v-if="row.salary">{{ $t('wap_00925') }}：{{ row.salary }}</div>
              </div>
            </div>
          </td>
          <td>
            <div>{{ $t('wap_00787') }}<a href="javascript:;" class="newcom_user_td">{{ row.job_name || row.job_id }}</a></div>
            <div class="com_received_tdtime">{{ row.datetime_n }}</div>
          </td>
          <td>
            <a href="javascript:;" class="cblue" @click="pick(row)">{{ $t('wap_com_00046') }}</a>
            <a href="javascript:;" class="cblue" @click="openRemark(row)">{{ $t('member_user_00242') }}</a>
            <select :value="row.is_browse" @change="setState(row.id, Number(($event.target as HTMLSelectElement).value))">
              <option v-for="s in [1, 2, 3, 4, 5, 7]" :key="s" :value="s">{{ browseLabel(s) }}</option>
            </select>
            <a href="javascript:;" class="List_dete cblue" @click="removeRow(row.id)">{{ $t('wap_js_00077') }}</a>
          </td>
        </tr>
      </table>
      <p v-if="list.length" class="site-pc">
        <button type="button" class="com_topbth" :disabled="!selected.length" @click="batchRead">{{ $t('member_com_00492') }}</button>
      </p>
      <div class="site-h5 resume_management_body_card">
        <div class="management_body_card_content">
          <MemberHrUserCard
            v-for="row in list"
            :key="'h5-' + row.id"
            :name="row.uname || String(row.uid)"
            :photo="row.photo ? mediaUrl(row.photo) : undefined"
            :job="row.job_name"
            :time="row.datetime_n"
            :state-text="browseLabel(row.is_browse)"
            :is-browse="row.is_browse"
            :invited="row.invited"
            :info="rowInfo(row)"
            @open="openResume(row)"
          >
            <div class="hr_userlist_czicon" @click="pick(row)">{{ $t('wap_com_00046') }}</div>
            <div class="hr_userlist_czicon" @click="removeRow(row.id)">{{ $t('common.delete') }}</div>
          </MemberHrUserCard>
        </div>
      </div>
      <form v-if="remarkFor" class="com_release_box" @submit.prevent="saveRemark">
        <ul>
          <MemberReleaseRow :label="$t('member_user_00242')" area><textarea v-model="remarkText" rows="3" /></MemberReleaseRow>
        </ul>
        <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
        <button type="button" class="verification_form_btn" @click="remarkFor = null">{{ $t('common.cancel') }}</button>
      </form>
      <MemberPager :page="page" :page-size="PAGE_SIZE" :total="total" @update:page="(p) => (page = p)" />
    </template>

    <h2>{{ $t('wap_com_00046') }}</h2>
    <form class="com_release_box" @submit.prevent="sendInvite()">
      <ul>
        <MemberReleaseRow v-if="invite.seeker_uid" :label="$t('common.resume')">
          <span>{{ invite.seeker_uid }} · {{ invite.job_id }}</span>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_00040')" required><input v-model="invite.intertime" type="datetime-local" required /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00243')" required><input v-model="invite.address" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common_02051')"><input v-model="invite.linkman" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common.phone')" required><input v-model="invite.linktel" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00102')" area><textarea v-model="invite.content" rows="3" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_com_00512')">
          <input v-model="invite.save_yqmb" type="checkbox" />
        </MemberReleaseRow>
      </ul>
      <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
    </form>
  </MemberPanel>
</template>

<style scoped>
.filters { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 1rem; }
.tabs { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 1rem; }
.tabs .on { font-weight: 700; }
.bulk { display: flex; align-items: center; gap: 1rem; }
.acts { display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; }
.pickbox { float: right; }
</style>
