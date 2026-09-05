<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

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
}
type DictItem = { id: number; name: string }
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
const dictParams = { source: 'user' }
const { data: eduDict } = await useAsyncData('dict-edu-user', () =>
  api.get<DictItem[]>('/v1/wap/dict/educations', dictParams).catch(() => [] as DictItem[]),
)
const { data: expDict } = await useAsyncData('dict-exp-user', () =>
  api.get<DictItem[]>('/v1/wap/dict/experiences', dictParams).catch(() => [] as DictItem[]),
)

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
  <section>
    <h1>{{ $t('member_com_00454') }}</h1>

    <form class="filters" @submit.prevent="applyFilters">
      <select v-model="filters.job_id">
        <option value="">{{ $t('wap_user_00154') }}·{{ $t('common_01936') }}</option>
        <option v-for="j in myJobs?.list || []" :key="j.id" :value="j.id">{{ j.name }}</option>
      </select>
      <select v-model="filters.edu">
        <option value="">{{ $t('wap_com_00301') }}·{{ $t('common_01936') }}</option>
        <option v-for="d in eduDict || []" :key="d.id" :value="d.id">{{ d.name }}</option>
      </select>
      <select v-model="filters.exp">
        <option value="">{{ $t('wap_user_00240') }}·{{ $t('common_01936') }}</option>
        <option v-for="d in expDict || []" :key="d.id" :value="d.id">{{ d.name }}</option>
      </select>
      <select v-model="filters.sex">
        <option value="">{{ $t('wap_com_00303') }}·{{ $t('common_01936') }}</option>
        <option value="1">{{ $t('common_02092') }}</option>
        <option value="2">{{ $t('common_02069') }}</option>
      </select>
      <select v-model="filters.uptime">
        <option value="">{{ $t('wap_00326') }}·{{ $t('common_01936') }}</option>
        <option v-for="o in uptimeOpts" :key="o.v" :value="o.v">{{ o.label }}</option>
      </select>
      <select v-model="filters.resume_state">
        <option value="">{{ $t('member_com_00110') }}·{{ $t('common_01936') }}</option>
        <option v-for="o in resumeStateOpts" :key="o.v" :value="o.v">{{ o.label }}</option>
      </select>
      <input v-model="filters.keyword" :placeholder="$t('admin_00149')" />
      <button type="submit">{{ $t('wap_00238') }}</button>
      <button type="button" @click="resetFilters">{{ $t('wap_00327') }}</button>
    </form>

    <nav class="tabs">
      <button
        v-for="tab in tabs"
        :key="String(tab.v)"
        type="button"
        :class="{ on: state === tab.v }"
        @click="pickState(tab.v)"
      >
        {{ tab.label }}<span v-if="tab.n != null" class="muted"> ({{ tab.n }})</span>
      </button>
    </nav>

    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <p v-if="list.length" class="bulk">
        <label><input v-model="allChecked" type="checkbox" /> {{ $t('wap_js_00074') }}</label>
        <button type="button" :disabled="!selected.length" @click="batchRead">
          {{ $t('member_com_00492') }}
        </button>
      </p>
      <p v-if="msg" class="muted">{{ msg }}</p>
      <p v-if="!list.length" class="muted">{{ $t('ui.no_applies') }}</p>

      <div class="stack">
        <article v-for="row in list" :key="row.id" class="job-card">
          <label class="pickbox"><input v-model="selected" type="checkbox" :value="row.id" /></label>
          <h3>
            <a href="#" @click.prevent="openResume(row)">{{ row.uname || row.uid }}</a>
            · {{ row.job_name || row.job_id }}
          </h3>
          <p class="muted">{{ row.datetime_n }} · {{ browseLabel(row.is_browse) }}</p>
          <p class="acts">
            <button type="button" @click="pick(row)">{{ $t('wap_com_00046') }}</button>
            <button type="button" @click="openRemark(row)">{{ $t('member_user_00242') }}</button>
            <select :value="row.is_browse" @change="setState(row.id, Number(($event.target as HTMLSelectElement).value))">
              <option v-for="s in [1, 2, 3, 4, 5, 7]" :key="s" :value="s">{{ browseLabel(s) }}</option>
            </select>
            <button type="button" @click="removeRow(row.id)">{{ $t('wap_js_00077') }}</button>
          </p>
          <form v-if="remarkFor?.id === row.id" class="form" @submit.prevent="saveRemark">
            <textarea v-model="remarkText" rows="3" :placeholder="$t('member_user_00242')" />
            <button type="submit">{{ $t('common.submit') }}</button>
            <button type="button" @click="remarkFor = null">{{ $t('common.cancel') }}</button>
          </form>
        </article>
      </div>

      <Pager :page="page" :page-size="PAGE_SIZE" :total="total" @update:page="(p) => (page = p)" />
    </template>

    <h2>{{ $t('wap_com_00046') }}</h2>
    <form class="form" @submit.prevent="sendInvite()">
      <p v-if="invite.seeker_uid" class="muted">{{ invite.seeker_uid }} · {{ invite.job_id }}</p>
      <input v-model="invite.intertime" type="datetime-local" required />
      <input v-model="invite.address" :placeholder="$t('wap_00040')" required />
      <input v-model="invite.linkman" :placeholder="$t('common_02051')" />
      <input v-model="invite.linktel" :placeholder="$t('common.phone')" required />
      <textarea v-model="invite.content" rows="3" />
      <label><input v-model="invite.save_yqmb" type="checkbox" /> {{ $t('member_com_00512') }}</label>
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
  </section>
</template>

<style scoped>
.filters { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 1rem; }
.tabs { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 1rem; }
.tabs .on { font-weight: 700; }
.bulk { display: flex; align-items: center; gap: 1rem; }
.acts { display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; }
.pickbox { float: right; }
</style>
