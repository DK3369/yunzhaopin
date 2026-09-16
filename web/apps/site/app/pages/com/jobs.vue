<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'
import { ApiError } from '~/utils/envelope'

type JobRow = {
  id: number
  name?: string
  state?: number
  status?: number
  istop?: boolean
  is_rec?: boolean
  is_urgent?: boolean
  xsdate?: number
  rec_time?: number
  urgent_time?: number
  minsalary?: number
  maxsalary?: number
  jobhits?: number
  jobnum?: number
  jobexpoure?: number
  lastupdate_n?: string
  statusbody?: string
}

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const route = useRoute()
const wRaw = Number(route.query.w)
const w = ref(Number.isFinite(wRaw) ? wRaw : 1)
const keywordInput = ref(typeof route.query.keyword === 'string' ? route.query.keyword : '')
const keyword = ref(keywordInput.value.trim())
const { data: pack, error, refresh } = await useAsyncData(
  () => `com-jobs-${page.value}-${w.value}-${keyword.value}`,
  () =>
    api.post<{
      jobs?: { list?: JobRow[]; total?: number }
      counts?: {
        w0?: number
        w1?: number
        w3?: number
        w4?: number
        w5?: number
        total: number
        online: number
        pending?: number
        closed?: number
        breakjob_num?: number
        top_num?: number
        rec_num?: number
        urgent_num?: number
      }
    }>('/v1/mcenter/jobs/overview', {
      page: page.value,
      page_size: pageSize,
      w: w.value,
      keyword: keyword.value || undefined,
    }),
)
const data = computed(() => pack.value?.jobs)
const counts = computed(() => pack.value?.counts || null)
const list = computed(() => (data.value?.list || []) as JobRow[])
const msg = ref('')
const days = ref(1)
const picked = ref<number[]>([])
const reserveOn = computed(() => String(settings.value.com_job_reserve || '') === '1')
const reserveEnd = ref('')
const reserveInterval = ref(30)
const reserveStart = ref('')
const reserveStop = ref('')
const quoteHint = ref('')
const buyHint = ref('')
const allPicked = computed({
  get: () => list.value.length > 0 && picked.value.length === list.value.length,
  set: (v: boolean) => {
    picked.value = v ? list.value.map((j) => j.id) : []
  },
})
function togglePick(id: number) {
  if (picked.value.includes(id)) picked.value = picked.value.filter((x) => x !== id)
  else picked.value = [...picked.value, id]
}

function jobPhase(job: { state?: number; status?: number }) {
  if (Number(job.status) === 1) return t('wap_com_00242')
  if (Number(job.state) === 0) return t('wap_user_00006')
  if (Number(job.state) === 3) return t('wap_user_00167')
  if (Number(job.state) === 1) return t('wap_com_00243')
  return t('member_user_00181')
}

function expireOf(job: JobRow, kind: 'top' | 'rec' | 'urgent') {
  const ts = kind === 'top' ? job.xsdate : kind === 'rec' ? job.rec_time : job.urgent_time
  return formatUnixDate(ts)
}

function isQuotaErr(e: unknown) {
  if (!(e instanceof ApiError)) return false
  return (
    e.key === 'job_refresh_quota' ||
    e.key === 'model_00056' ||
    e.key === 'member_com_00696' ||
    e.key === 'wap_01287' ||
    e.key === 'api_wxapp_00002' ||
    e.key === 'common_00207' ||
    e.key === 'common_00206' ||
    e.key === 'common_00180'
  )
}
function failAct(e: unknown) {
  if (isQuotaErr(e)) {
    buyHint.value = e instanceof ApiError ? e.message : t('wap_com_00048')
    return t('wap_com_00048')
  }
  return e instanceof Error ? e.message : t('ui.load_failed')
}

async function refreshJob(id: number, confirm = false) {
  msg.value = ''
  buyHint.value = ''
  try {
    const r = await api.post<{ status?: number; integral?: number; jifen?: number; price?: number }>(
      '/v1/mcenter/jobs/refresh',
      { id, confirm },
    )
    if (Number(r.status) === 2) {
      const pts = r.integral || r.jifen
      const text = pts
        ? `${t('common_00697')}${pts}${t('common_01935')}?`
        : r.price
          ? `${t('common_00696')}${r.price}${t('common_00757')}?`
          : t('common_00696')
      if (window.confirm(text)) await refreshJob(id, true)
      return
    }
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = failAct(e)
  }
}
async function setStatus(id: number, status: number) {
  msg.value = ''
  buyHint.value = ''
  try {
    await api.post('/v1/mcenter/jobs/status', { id, status })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = failAct(e)
  }
}
function promoOn(job: JobRow, kind: 'top' | 'rec' | 'urgent') {
  if (kind === 'top') return Boolean(job.istop)
  if (kind === 'rec') return Boolean(job.is_rec)
  return Boolean(job.is_urgent)
}
async function promoteOrClose(job: JobRow, kind: 'top' | 'rec' | 'urgent') {
  if (promoOn(job, kind)) return closePromote(job.id, kind)
  return promote(job.id, kind)
}
async function promote(jobId: number, kind: 'top' | 'rec' | 'urgent') {
  msg.value = ''
  quoteHint.value = ''
  buyHint.value = ''
  const n = Math.max(1, Math.min(365, Number(days.value) || 1))
  try {
    const q = await api.post<{ remain?: number; active?: boolean; expire_at?: number }>(
      '/v1/mcenter/jobs/promote/quote',
      { job_id: jobId, kind },
    )
    quoteHint.value = `${q.remain ?? 0}`
    await api.post('/v1/mcenter/jobs/promote', { job_id: jobId, kind, days: n })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = failAct(e)
  }
}
async function closePromote(jobId: number, kind: 'top' | 'rec' | 'urgent') {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/jobs/promote/close', { job_id: jobId, kind })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
const { data: vip } = await useAsyncData(
  'com-vip-current',
  () =>
    api
      .post<{ job_num?: number; rating_type?: number; expires_at?: number; active?: boolean }>(
        '/v1/mcenter/vip/current',
        {},
      )
      .catch(() => null),
  reuseAsyncCache(),
)
const hintDismissed = ref(false)
onMounted(() => {
  hintDismissed.value = sessionStorage.getItem('i_know_job') === '1'
})
function dismissHint() {
  sessionStorage.setItem('i_know_job', '1')
  hintDismissed.value = true
}
const webtel = computed(() => String(settings.value.sy_comwebtel || settings.value.sy_freewebtel || ''))
const pendingN = computed(() => Number(counts.value?.w0 || 0))
const showPendingHint = computed(() => !hintDismissed.value && pendingN.value > 0)
const showQuotaHint = computed(
  () => !hintDismissed.value && pendingN.value <= 0 && Number(vip.value?.rating_type) === 1,
)
const vipJobQuota = computed(() => {
  const etime = Number(vip.value?.expires_at ?? 0)
  const todayStart = Math.floor(new Date().setHours(0, 0, 0, 0) / 1000)
  const ok = etime === 0 || etime > todayStart
  if (!ok) return { prefix: 'default_00381' as const, num: '0' }
  if (Number(vip.value?.rating_type) === 1) {
    return { prefix: 'default_00379' as const, num: String(vip.value?.job_num ?? 0) }
  }
  return { prefix: 'default_00380' as const, num: t('common_01936') }
})

async function batchOpen() {
  msg.value = ''
  buyHint.value = ''
  if (!picked.value.length) {
    msg.value = t('common_01164')
    return
  }
  try {
    for (const id of picked.value) {
      await api.post('/v1/mcenter/jobs/status', { id, status: 0 })
    }
    msg.value = t('common.success')
    picked.value = []
    await refresh()
  } catch (e: unknown) {
    msg.value = failAct(e)
  }
}
async function batch(kind: 'refresh' | 'close' | 'delete') {
  msg.value = ''
  buyHint.value = ''
  if (!picked.value.length) {
    msg.value = t('common_01164')
    return
  }
  try {
    await api.post(`/v1/mcenter/jobs/batch/${kind}`, { ids: picked.value })
    msg.value = t('common.success')
    picked.value = []
    await refresh()
  } catch (e: unknown) {
    msg.value = failAct(e)
  }
}
async function reserveOne(id: number, status: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/jobs/reserve', {
      job_id: id,
      end_time: reserveEnd.value,
      interval: reserveInterval.value,
      status,
      s_time: reserveStart.value,
      e_time: reserveStop.value,
    })
    msg.value = t('common_01047')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
async function copyShare(id: number, kind: 'text' | 'link') {
  msg.value = ''
  const fallback = import.meta.client ? `${window.location.origin}/jobs/${id}` : `/jobs/${id}`
  try {
    const r = await api.get<{ plain_text?: string; share_url?: string }>('/v1/wap/jobs/share-text', { id })
    const text = kind === 'link' ? String(r.share_url || fallback) : String(r.plain_text || '')
    if (import.meta.client && navigator.clipboard && text) {
      await navigator.clipboard.writeText(text)
      msg.value = t('common.success')
      return
    }
    msg.value = text || t('ui.load_failed')
  } catch (e: unknown) {
    if (kind === 'link' && import.meta.client && navigator.clipboard) {
      try {
        await navigator.clipboard.writeText(fallback)
        msg.value = t('common.success')
        return
      } catch {
        /* fall through */
      }
    }
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
async function fillReserve(id: number) {
  msg.value = ''
  try {
    const r = await api.post<{
      status?: number
      interval?: number
      s_time?: string
      e_time?: string
      end_time?: number
    }>('/v1/mcenter/jobs/reserve/get', { job_id: id })
    reserveEnd.value = formatUnixDate(r.end_time)
    reserveInterval.value = Number(r.interval || 30)
    reserveStart.value = r.s_time || ''
    reserveStop.value = r.e_time || ''
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
async function reservePicked(status: number) {
  msg.value = ''
  if (!picked.value.length) {
    msg.value = t('common_01164')
    return
  }
  try {
    for (const id of picked.value) {
      await api.post('/v1/mcenter/jobs/reserve', {
        job_id: id,
        end_time: reserveEnd.value,
        interval: reserveInterval.value,
        status,
        s_time: reserveStart.value,
        e_time: reserveStop.value,
      })
    }
    msg.value = t('common_01047')
    picked.value = []
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
useSeoMeta({ title: t('wap_com_00106') })
const jobTotal = computed(() => inferTotal(data.value, list.value))
const h5Menu = ref(0)
const h5MenuKind = ref<'promote' | 'more' | ''>('')
function toggleH5Menu(id: number, kind: 'promote' | 'more') {
  if (h5Menu.value === id && h5MenuKind.value === kind) {
    h5Menu.value = 0
    h5MenuKind.value = ''
    return
  }
  h5Menu.value = id
  h5MenuKind.value = kind
}
function setW(n: number) {
  w.value = n
  page.value = 1
  picked.value = []
  if (String(route.query.w) !== String(n)) {
    navigateTo({ path: '/com/jobs', query: { ...route.query, w: String(n) } }, { replace: true })
  }
  refresh()
}
function searchJobs() {
  keyword.value = keywordInput.value.trim()
  page.value = 1
  picked.value = []
  refresh()
}
watch(page, () => {
  picked.value = []
  refresh()
})
const jobTabs = computed(() => [
  { value: 1, label: t('wap_com_00243'), on: w.value === 1, count: counts.value?.w1 ?? counts.value?.online, select: () => setW(1) },
  { value: 0, label: t('wap_user_00006'), on: w.value === 0, count: counts.value?.w0 ?? counts.value?.pending, select: () => setW(0) },
  { value: 3, label: t('wap_user_00167'), on: w.value === 3, count: counts.value?.w3, select: () => setW(3) },
  { value: 4, label: t('wap_com_00245'), on: w.value === 4, count: counts.value?.w4 ?? counts.value?.closed, select: () => setW(4) },
  { value: 5, label: t('common.all'), on: w.value === 5, count: counts.value?.w5 ?? counts.value?.total, select: () => setW(5) },
])
const allJobCount = computed(() => Number(counts.value?.w5 ?? counts.value?.total ?? 0))
const emptyAll = computed(() => allJobCount.value <= 0 && !keyword.value)
const emptyText = computed(() => {
  if (emptyAll.value) return t('member_com_00216')
  if (w.value === 1) return t('wap_com_00211')
  if (w.value === 0 || w.value === 3) return t('wap_com_00210')
  if (w.value === 4) return t('wap_com_00209')
  return t('member_com_00216')
})
const emptySub = computed(() => (emptyAll.value ? t('member_com_00215') : ''))
</script>

<template>
  <MemberPanel
    :title="$t('wap_com_00106')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !list.length"
    :empty-text="emptyText"
    :empty-sub="emptySub"
    empty-to="/com/jobs/new"
    :empty-action="$t('wap_00322')"
  >
    <MemberComScreen
      :tabs="jobTabs"
      add-to="/com/jobs/new"
      :add-label="$t('wap_00322')"
      :keyword="keywordInput"
      searchable
      :search-placeholder="$t('member_com_00218')"
      @update:keyword="keywordInput = $event"
      @search="searchJobs"
    >
      <template #addExtra>
        <div v-if="showPendingHint" class="com_topbth_zh">
          <div class="com_topbth_zh_pd">
            {{ $t('member_com_00040') }}
            <div>
              {{ $t('member_com_00224') }}
              <font color="#FF0000">{{ pendingN }}</font>
              {{ $t('default_00378') }}{{ webtel }}
            </div>
          </div>
          <div class="com_topbth_zh_bot"><a href="javascript:;" @click.prevent="dismissHint">{{ $t('member_com_00214') }}</a></div>
        </div>
        <div v-else-if="showQuotaHint" class="com_topbth_zh">
          <div class="com_topbth_zh_pd">
            {{ $t('default_00383') }}{{ vip?.job_num ?? 0 }}{{ $t('wap_user_00151') }}
          </div>
          <div class="com_topbth_zh_bot"><a href="javascript:;" @click.prevent="dismissHint">{{ $t('member_com_00214') }}</a></div>
        </div>
      </template>
    </MemberComScreen>
    <div class="site-h5 com-h5-filters">
      <input
        v-model="keywordInput"
        type="search"
        class="com-h5-filters__kw"
        :placeholder="$t('member_com_00218')"
        @keydown.enter.prevent="searchJobs"
      />
    </div>
    <div class="admincont_box site-pc">
      <p v-if="counts" class="muted">
        {{ $t('wap_com_00029') }} {{ counts.breakjob_num ?? 0 }} ·
        {{ $t('wap_com_00238') }} {{ counts.top_num ?? 0 }}{{ $t('common_02067') }} ·
        {{ $t('wap_com_00237') }} {{ counts.rec_num ?? 0 }}{{ $t('common_02067') }} ·
        {{ $t('member_com_00613') }} {{ counts.urgent_num ?? 0 }}{{ $t('common_02067') }}
      </p>
      <p>
        {{ $t('member_com_00282') }}
        <input v-model.number="days" type="number" min="1" max="365" class="com_release_textnew_text" style="width: 5em" />
        {{ $t('common_02067') }}
      </p>
      <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
      <table v-if="list.length" class="com_table">
        <tr>
          <th width="25"><label><input v-model="allPicked" type="checkbox" class="com_job_list_check" /></label></th>
          <th>{{ $t('wap_com_00288') }}</th>
          <th>{{ $t('wap_00794') }}</th>
          <th>{{ $t('member_com_00268') }}</th>
          <th>{{ $t('wap_00847') }}</th>
          <th>{{ $t('wap_00326') }}</th>
          <th>{{ $t('wap_com_00246') }}</th>
          <th>{{ $t('wap_com_00236') }}</th>
          <th>{{ $t('member_user_00048') }}</th>
        </tr>
        <tr v-for="job in list" :key="job.id">
          <td align="center">
            <input type="checkbox" class="com_job_list_check" :checked="picked.includes(job.id)" @change="togglePick(job.id)" />
          </td>
          <td>
            <div class="job_looklist_namebox">
              <NuxtLink :to="`/jobs/${job.id}`" class="job_looklist_name">{{ job.name }}</NuxtLink>
            </div>
            <div class="muted">{{ jobPhase(job) }}</div>
            <div v-if="Number(job.state) === 3 && job.statusbody" class="y_verify_wtg_yuany">
              {{ $t('admin_system_00134') }}{{ job.statusbody }}
            </div>
          </td>
          <td align="center">
            {{ job.jobnum ?? 0 }}
            <NuxtLink v-if="job.jobnum" :to="`/com/applications?job_id=${job.id}`" class="yun_m_job_r_l">{{ $t('wap_com_00427') }}</NuxtLink>
          </td>
          <td align="center">{{ job.jobhits ?? 0 }}</td>
          <td align="center">{{ job.jobexpoure ?? 0 }}{{ $t('wap_01543') }}</td>
          <td align="center">{{ job.lastupdate_n || '—' }}</td>
          <td align="center">
            <a href="javascript:;" class="job_looklist_fx" @click="copyShare(job.id, 'text')">{{ $t('wap_com_00246') }}</a>
            <NuxtLink :to="`/poster/job/${job.id}`" class="job_looklist_hb">{{ $t('member_com_00270') }}</NuxtLink>
          </td>
          <td align="center">
            <div class="job_looklist_tgbox">
              <a href="javascript:;" class="job_looklist_tg" :class="{ job_looklist_tg_kq: job.is_rec }" @click="promoteOrClose(job, 'rec')">{{ job.is_rec ? $t('wap_com_00224') : $t('wap_01465') }}</a>
              <a href="javascript:;" class="job_looklist_tg" :class="{ job_looklist_tg_kq: job.is_urgent }" @click="promoteOrClose(job, 'urgent')">{{ job.is_urgent ? $t('wap_com_00224') : $t('wap_00222') }}</a>
              <a href="javascript:;" class="job_looklist_tg" :class="{ job_looklist_tg_kq: job.istop }" @click="promoteOrClose(job, 'top')">{{ job.istop ? $t('wap_com_00224') : $t('wap_user_00335') }}</a>
              <a v-if="reserveOn" href="javascript:;" class="job_looklist_tg" @click="fillReserve(job.id)">{{ $t('member_com_00267') }}</a>
            </div>
          </td>
          <td align="center">
            <a href="javascript:;" class="com_bth" @click="refreshJob(job.id)">{{ $t('wap_com_00029') }}</a>
            <NuxtLink :to="`/jobs/${job.id}`" class="com_bth">{{ $t('wap_00071') }}</NuxtLink>
            <a v-if="Number(job.status) === 1" href="javascript:;" class="com_bth" @click="setStatus(job.id, 0)">{{ $t('wap_com_00244') }}</a>
            <a v-else href="javascript:;" class="com_bth" @click="setStatus(job.id, 1)">{{ $t('wap_com_00245') }}</a>
            <NuxtLink :to="`/com/jobs/new?id=${job.id}`" class="com_bth">{{ $t('wap_js_00073') }}</NuxtLink>
            <a href="javascript:;" class="com_bth" @click="picked = [job.id]; batch('delete')">{{ $t('wap_js_00077') }}</a>
          </td>
        </tr>
      </table>
      <div v-if="list.length" class="com_Release_job_bot">
        <label class="com_Release_job_qx"><input v-model="allPicked" type="checkbox" class="com_job_list_check" /> {{ $t('common.all') }}</label>
        <a href="javascript:;" class="c_btn_02" @click="batch('refresh')">{{ $t('wap_com_00029') }}</a>
        <a v-if="w === 4" href="javascript:;" class="c_btn_02 c_btn_02_w110" @click="batchOpen">{{ $t('member_com_00219') }}</a>
        <a href="javascript:;" class="c_btn_02" @click="batch('close')">{{ $t('wap_com_00245') }}</a>
        <a href="javascript:;" class="c_btn_02 c_btn_02_w110" @click="batch('delete')">{{ $t('common.delete') }}</a>
        <span v-if="reserveOn">
          <input v-model="reserveEnd" type="date" />
          <input v-model="reserveStart" type="time" />
          <input v-model="reserveStop" type="time" />
          <input v-model.number="reserveInterval" type="number" min="1" style="width: 5em" />
          <a href="javascript:;" class="c_btn_02" @click="reservePicked(1)">{{ $t('member_com_00261') }}</a>
        </span>
      </div>
    </div>
    <div class="site-h5 more_position_body">
      <div v-for="job in list" :key="'h5-' + job.id" class="position_body_card">
        <div class="position_body_card_top">
          <NuxtLink :to="`/jobs/${job.id}`" class="body_card_top_name">{{ job.name }}</NuxtLink>
          <span v-if="job.is_rec" class="job-h5-badge">{{ $t('wap_01465') }}</span>
          <span v-if="job.is_urgent" class="job-h5-badge">{{ $t('wap_00222') }}</span>
          <span v-if="job.istop" class="job-h5-badge">{{ $t('wap_user_00335') }}</span>
        </div>
        <div v-if="Number(job.state) === 3 && job.statusbody" class="y_verify_wtg_yuany">
          {{ $t('admin_system_00134') }}{{ job.statusbody }}
        </div>
        <div class="position_body_card_center">
          <div class="body_card_center_left">
            <div class="more_position_quantity">
              <div class="position_quantity_exposure">
                <div class="quantity_exposure_q">{{ $t('wap_00847') }}</div>
                <div class="quantity_exposure_a">{{ job.jobexpoure ?? 0 }}</div>
              </div>
              <div class="position_quantity_exposure">
                <div class="quantity_exposure_q">{{ $t('wap_00848') }}</div>
                <div class="quantity_exposure_a">{{ job.jobhits ?? 0 }}</div>
              </div>
            </div>
            <div class="more_position_new_time">
              <div class="quantity_exposure_q">{{ $t('wap_00849') }}</div>
              <div class="quantity_exposure_a">{{ job.lastupdate_n || jobPhase(job) }}</div>
            </div>
          </div>
          <NuxtLink :to="`/com/applications?job_id=${job.id}`" class="body_card_center_right">
            <div class="more_position_deliver_number">{{ job.jobnum ?? 0 }}</div>
            <div class="more_position_deliver">{{ $t('wap_com_00235') }}</div>
          </NuxtLink>
        </div>
        <div class="position_body_card_bom">
          <ul>
            <li @click="toggleH5Menu(job.id, 'promote')">
              <div class="body_card_bom_icon"><img src="/legacy/h5/images/job_promotion.png" alt="" /></div>
              <div class="body_card_bom_name">{{ $t('wap_com_00236') }}</div>
              <div v-if="h5Menu === job.id && h5MenuKind === 'promote'" class="job_czmore" @click.stop>
                <span class="job_czmore_a">
                  {{ $t('member_com_00282') }}
                  <input v-model.number="days" type="number" min="1" max="365" style="width: 4em" />
                  {{ $t('common_02067') }}
                </span>
                <span class="job_czmore_a" :class="{ job_looklist_tg_kq: job.istop }" @click="promoteOrClose(job, 'top')">{{ job.istop ? $t('wap_com_00224') : $t('wap_user_00335') }}</span>
                <span class="job_czmore_a" :class="{ job_looklist_tg_kq: job.is_rec }" @click="promoteOrClose(job, 'rec')">{{ job.is_rec ? $t('wap_com_00224') : $t('wap_01465') }}</span>
                <span class="job_czmore_a" :class="{ job_looklist_tg_kq: job.is_urgent }" @click="promoteOrClose(job, 'urgent')">{{ job.is_urgent ? $t('wap_com_00224') : $t('wap_00222') }}</span>
              </div>
            </li>
            <li @click="refreshJob(job.id)">
              <div class="body_card_bom_icon"><img src="/legacy/h5/images/jobhunter_refresh.png" alt="" /></div>
              <div class="body_card_bom_name">{{ $t('wap_com_00029') }}</div>
            </li>
            <li>
              <NuxtLink :to="`/com/jobs/new?id=${job.id}`">
                <div class="body_card_bom_icon"><img src="/legacy/h5/images/jobhunter_preview.png" alt="" /></div>
                <div class="body_card_bom_name">{{ $t('common.edit') }}</div>
              </NuxtLink>
            </li>
            <li @click="toggleH5Menu(job.id, 'more')">
              <div class="body_card_bom_icon"><img src="/legacy/h5/images/addition.png" alt="" /></div>
              <div class="body_card_bom_name">{{ $t('wap_js_00089') }}</div>
              <div v-if="h5Menu === job.id && h5MenuKind === 'more'" class="job_czmore" @click.stop>
                <span v-if="Number(job.status) === 1" class="job_czmore_a job_czmore_gb" @click="setStatus(job.id, 0)">{{ $t('wap_com_00244') }}</span>
                <span v-else class="job_czmore_a job_czmore_gb" @click="setStatus(job.id, 1)">{{ $t('wap_com_00245') }}</span>
                <span class="job_czmore_a job_czmore_sc" @click="picked = [job.id]; batch('delete')">{{ $t('wap_js_00077') }}</span>
                <span class="job_czmore_a" @click="copyShare(job.id, 'text')">{{ $t('wap_com_00246') }}</span>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="jobTotal" @update:page="go" />
    <p v-if="quoteHint" class="muted">{{ quoteHint }}</p>
    <p v-if="buyHint" class="muted">
      {{ buyHint }}
      <NuxtLink to="/com/member-right">{{ $t('wap_com_00097') }}</NuxtLink>
      ·
      <NuxtLink to="/com/added">{{ $t('wap_com_00048') }}</NuxtLink>
      ·
      <NuxtLink to="/com/pay">{{ $t('common_01946') }}</NuxtLink>
    </p>
    <p v-if="msg">{{ msg }}</p>
    <div class="com_tip_bottom">
      <div class="yun_tip_tit">{{ $t('wap_user_00205') }}</div>
      <div class="yun_prompt_cont">
        <p>{{ $t(vipJobQuota.prefix) }}{{ vipJobQuota.num }}{{ $t('default_00385').replace('<br>', '') }}</p>
        <p>
          2、{{ $t('member_com_00455') }}
          <NuxtLink to="/com/member-right" class="yun_m_job_r_l">{{ $t('member_com_00266') }}</NuxtLink>
        </p>
        <p>3、{{ $t('member_com_00456') }}</p>
        <p>4、{{ $t('member_com_00460') }}</p>
        <p>5、{{ $t('member_com_00461') }}</p>
        <p>6、{{ $t('member_com_00462') }}</p>
      </div>
    </div>
  </MemberPanel>
</template>
