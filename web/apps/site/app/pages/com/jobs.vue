<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'

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
}

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { data, error, refresh } = await useAsyncData('com-jobs', () =>
  api.post('/v1/mcenter/jobs/list', { page: 1, page_size: 20 }),
)
const { data: counts, refresh: refreshCounts } = await useAsyncData('com-job-counts', () =>
  api
    .post<{
      total: number
      online: number
      breakjob_num?: number
      top_num?: number
      rec_num?: number
      urgent_num?: number
    }>('/v1/mcenter/jobs/counts', {})
    .catch(() => null),
)
const list = computed(() => (data.value?.list || []) as JobRow[])
const msg = ref('')
const days = ref(1)
const picked = ref<number[]>([])
const reserveOn = computed(() => String(settings.value.com_job_reserve || '') === '1')
const reserveEnd = ref('')
const reserveInterval = ref(30)
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

async function refreshJob(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/jobs/refresh', { id })
    msg.value = t('common.success')
    await refresh()
    await refreshCounts()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
async function setStatus(id: number, status: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/jobs/status', { id, status })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
async function promote(jobId: number, kind: 'top' | 'rec' | 'urgent') {
  msg.value = ''
  const n = Math.max(1, Math.min(365, Number(days.value) || 1))
  try {
    await api.post('/v1/mcenter/jobs/promote', { job_id: jobId, kind, days: n })
    msg.value = t('common.success')
    await refresh()
    await refreshCounts()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
async function closePromote(jobId: number, kind: 'top' | 'rec' | 'urgent') {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/jobs/promote/close', { job_id: jobId, kind })
    msg.value = t('common.success')
    await refresh()
    await refreshCounts()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}
async function batch(kind: 'refresh' | 'close' | 'delete') {
  msg.value = ''
  if (!picked.value.length) {
    msg.value = t('common_01164')
    return
  }
  try {
    await api.post(`/v1/mcenter/jobs/batch/${kind}`, { ids: picked.value })
    msg.value = t('common.success')
    picked.value = []
    await refresh()
    await refreshCounts()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
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
    })
    msg.value = t('common_01047')
    await refresh()
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
</script>

<template>
  <MemberPanel :title="$t('wap_com_00106')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p><NuxtLink to="/com/jobs/new">{{ $t('wap_00322') }}</NuxtLink></p>
    <p v-if="counts" class="muted">
      {{ $t('wap_com_00029') }} {{ counts.breakjob_num ?? 0 }} ·
      {{ $t('wap_com_00238') }} {{ counts.top_num ?? 0 }}{{ $t('common_02067') }} ·
      {{ $t('wap_com_00237') }} {{ counts.rec_num ?? 0 }}{{ $t('common_02067') }} ·
      {{ $t('member_com_00613') }} {{ counts.urgent_num ?? 0 }}{{ $t('common_02067') }}
    </p>
    <p>
      <label>{{ $t('member_com_00282') }}
        <input v-model.number="days" type="number" min="1" max="365" style="width: 4em">
        {{ $t('common_02067') }}
      </label>
    </p>
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <p>
      <label><input v-model="allPicked" type="checkbox" /> {{ $t('common.all') }}</label>
      <button type="button" @click="batch('refresh')">{{ $t('wap_com_00029') }}</button>
      <button type="button" @click="batch('close')">{{ $t('wap_com_00245') }}</button>
      <button type="button" @click="batch('delete')">{{ $t('common.delete') }}</button>
    </p>
    <p v-if="reserveOn">
      {{ $t('member_com_00267') }}
      <input v-model="reserveEnd" type="date" />
      <input v-model.number="reserveInterval" type="number" min="1" style="width: 5em" />
      <button type="button" @click="reservePicked(1)">{{ $t('member_com_00261') }}</button>
      <button type="button" @click="reservePicked(2)">{{ $t('member_com_00278') }}</button>
    </p>
    <article v-for="job in list" :key="job.id" class="look_resume_list">
      <h3>
        <input type="checkbox" :checked="picked.includes(job.id)" @change="togglePick(job.id)" />
        {{ job.name }}
      </h3>
      <p class="muted">{{ $t('member_user_00181') }} {{ jobPhase(job) }}</p>
      <p class="muted">
        <span v-if="job.istop">{{ $t('wap_com_00238') }} {{ expireOf(job, 'top') }}</span>
        <span v-if="job.is_rec"> {{ $t('wap_com_00237') }} {{ expireOf(job, 'rec') }}</span>
        <span v-if="job.is_urgent"> {{ $t('member_com_00613') }} {{ expireOf(job, 'urgent') }}</span>
      </p>
      <p>
        <NuxtLink :to="`/com/jobs/new?id=${job.id}`">{{ $t('common.edit') }}</NuxtLink>
        <button type="button" @click="refreshJob(job.id)">{{ $t('wap_com_00029') }}</button>
        <button type="button" @click="setStatus(job.id, 0)">{{ $t('wap_com_00244') }}</button>
        <button type="button" @click="setStatus(job.id, 1)">{{ $t('wap_com_00245') }}</button>
        <template v-if="reserveOn">
          <button type="button" @click="reserveOne(job.id, 1)">{{ $t('member_com_00267') }}</button>
          <button type="button" @click="reserveOne(job.id, 2)">{{ $t('member_com_00278') }}</button>
        </template>
      </p>
      <p>
        <button v-if="!job.istop" type="button" @click="promote(job.id, 'top')">{{ $t('wap_com_00238') }}</button>
        <button v-else type="button" @click="closePromote(job.id, 'top')">{{ $t('wap_com_00231') }}</button>
        <button v-if="!job.is_rec" type="button" @click="promote(job.id, 'rec')">{{ $t('wap_com_00237') }}</button>
        <button v-else type="button" @click="closePromote(job.id, 'rec')">{{ $t('common.close') }} {{ $t('wap_com_00237') }}</button>
        <button v-if="!job.is_urgent" type="button" @click="promote(job.id, 'urgent')">{{ $t('member_com_00613') }}</button>
        <button v-else type="button" @click="closePromote(job.id, 'urgent')">{{ $t('common.close') }} {{ $t('member_com_00613') }}</button>
      </p>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
