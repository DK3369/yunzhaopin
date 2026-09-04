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
async function add(uid: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/talent-pool', { eid: uid, seeker_uid: uid })
    msg.value = t('ui.add_to_talent')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.talent_pool') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.talent_pool') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('ui.please_login_com') : $t('ui.load_failed') }}</p>
    <h2>{{ $t('ui.public_resumes') }}</h2>
    <p v-if="!(publicResumes?.list || []).length" class="muted">{{ $t('ui.no_public_resume') }}</p>
    <div class="stack">
      <article v-for="r in publicResumes?.list || []" :key="r.uid" class="job-card">
        <h3>{{ r.display_name || r.name }}</h3>
        <p class="muted">{{ r.education_n }} · {{ r.exp_n }}</p>
        <NuxtLink :to="`/resumes/${r.uid}`">{{ $t('ui.view') }}</NuxtLink>
        <button type="button" @click="add(r.uid)">{{ $t('ui.add_to_talent') }}</button>
      </article>
    </div>
    <h2>{{ $t('ui.favorited') }}</h2>
    <p v-if="!(pool?.list || []).length" class="muted">{{ $t('ui.talent_empty') }}</p>
    <div class="stack">
      <article v-for="row in pool?.list || []" :key="row.id" class="job-card">
        <h3>{{ $t('ui.seeker') }} {{ row.seeker_uid }}</h3>
        <p v-if="row.remark" class="muted">{{ row.remark }}</p>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
