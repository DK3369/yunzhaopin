<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

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
const { data: pool, error, refresh } = await useAsyncData('talent-pool', () =>
  api.post<{ list: PoolRow[]; total: number }>('/v1/mcenter/talent-pool/list', {
    page: 1,
    page_size: 20,
  }),
)
const { data: publicResumes } = await useAsyncData('talent-search', () =>
  api.get('/v1/wap/resumes', { page: 1, page_size: 20 }),
)
const msg = ref('')
const list = computed<PoolRow[]>(() => pool.value?.list || [])

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
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

useSeoMeta({ title: t('member_com_00597') })
</script>

<template>
  <section>
    <h1>{{ $t('member_com_00597') }}</h1>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <h2>{{ $t('ui.public_resumes') }}</h2>
    <p v-if="!(publicResumes?.list || []).length" class="muted">{{ $t('ui.no_public_resume') }}</p>
    <div class="stack">
      <article v-for="r in publicResumes?.list || []" :key="r.uid" class="job-card">
        <h3>{{ r.display_name || r.name }}</h3>
        <p class="muted">{{ r.education_n }} · {{ r.exp_n }}</p>
        <NuxtLink :to="`/resumes/${r.uid}`">{{ $t('wap_com_00427') }}</NuxtLink>
        <button type="button" @click="add(r)">{{ $t('ui.add_to_talent') }}</button>
      </article>
    </div>
    <h2>{{ $t('ui.favorited') }}</h2>
    <p v-if="!list.length" class="muted">{{ $t('ui.talent_empty') }}</p>
    <div class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>
          <NuxtLink :to="`/resumes/${row.eid || row.seeker_uid}`">
            {{ row.uname || row.seeker_uid }}
          </NuxtLink>
        </h3>
        <p v-if="row.remark" class="muted">{{ $t('ui.remark') }}: {{ row.remark }}</p>
        <p v-if="row.ctime_n" class="muted">{{ row.ctime_n }}</p>
        <div class="row">
          <button type="button" @click="openRemark(row)">{{ $t('wap_com_00069') }}</button>
          <button type="button" @click="remove(row)">{{ $t('common.delete') }}</button>
        </div>
        <form v-if="remarkFor === row.id" class="form" @submit.prevent="saveRemark">
          <textarea v-model="remarkText" rows="3" :placeholder="$t('wap_00807')" />
          <div class="row">
            <button type="submit">{{ $t('common.save') }}</button>
            <button type="button" @click="remarkFor = 0">{{ $t('common.cancel') }}</button>
          </div>
        </form>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>

<style scoped>
.row {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-wrap: wrap;
}
</style>
