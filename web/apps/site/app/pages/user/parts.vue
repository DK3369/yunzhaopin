<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: applies, error, refresh: refreshApplies } = await useAsyncData('my-part-applies', () =>
  api.post('/v1/mcenter/my-part-applications/list', { page: 1, page_size: 20 }),
)
const { data: collects, refresh: refreshCollects } = await useAsyncData('my-part-collects', () =>
  api.post('/v1/mcenter/my-part-collects/list', { page: 1, page_size: 20 }),
)
const msg = ref('')
function statusLabel(status?: number) {
  if (status === 1) return t('wap_user_00260')
  if (status === 2) return t('wap_com_00427')
  if (status === 3) return t('wap_com_00046')
  return String(status ?? '')
}
async function delApply(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-part-applications', { ids: [id] })
    await refreshApplies()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function delCollect(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-part-collects', { ids: [id] })
    await refreshCollects()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00303') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00303') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <h2>{{ $t('ui.apply') }}</h2>
    <p v-if="!(applies?.list || []).length" class="muted">{{ $t('ui.no_apply') }}</p>
    <div class="stack">
      <article v-for="row in applies?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink v-if="row.job_id" :to="`/parts/${row.job_id}`">{{ row.job_name || row.job_id }}</NuxtLink>
          <span v-else>{{ row.job_name || row.job_id }}</span>
        </h3>
        <p class="muted">{{ row.com_name }} · {{ statusLabel(row.status) }} · {{ row.ctime_n }}</p>
        <button type="button" @click="delApply(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <h2>{{ $t('member_user_00103') }}</h2>
    <p v-if="!(collects?.list || []).length" class="muted">{{ $t('ui.no_fav') }}</p>
    <div class="stack">
      <article v-for="row in collects?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink v-if="row.job_id" :to="`/parts/${row.job_id}`">{{ row.job_name || row.job_id }}</NuxtLink>
          <span v-else>{{ row.job_name || row.job_id }}</span>
        </h3>
        <p class="muted">{{ row.com_name }} · {{ row.ctime_n }}</p>
        <button type="button" @click="delCollect(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
