<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: pool, error, refresh } = await useAsyncData('talent-pool', () =>
  api.post('/v1/mcenter/talent-pool/list', { page: 1, page_size: 20 }),
)
const { data: publicResumes } = await useAsyncData('talent-search', () =>
  api.get('/v1/wap/resumes', { page: 1, page_size: 20 }),
)
const msg = ref('')
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
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_com_00597') })
</script>

<template>
  <section>
    <h1>{{ $t('member_com_00597') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
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
    <p v-if="!(pool?.list || []).length" class="muted">{{ $t('ui.talent_empty') }}</p>
    <div class="stack">
      <article v-for="row in pool?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink :to="`/resumes/${row.eid || row.seeker_uid}`">{{ row.uname || row.seeker_uid }}</NuxtLink>
        </h3>
        <p v-if="row.remark" class="muted">{{ row.remark }}</p>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
