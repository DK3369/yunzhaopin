<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-parts', () =>
  api.post('/v1/mcenter/com-parts/list', { page: 1, page_size: 20 }),
)
const { data: applies, refresh: refreshApplies } = await useAsyncData('com-part-applies', () =>
  api.post('/v1/mcenter/com-part-applications', { page: 1, page_size: 20 }),
)
const msg = ref('')
async function setApply(id: number, status: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/com-part-applications/status', { id, status })
    msg.value = t('common.success')
    await refreshApplies()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function removePart(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/com-parts', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.com_parts') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.com_parts') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('ui.please_login_com') : $t('ui.load_failed') }}</p>
    <h2>{{ $t('ui.published') }}</h2>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_parts') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.name || row.id }}</h3>
        <button type="button" @click="removePart(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <h2>{{ $t('ui.recv_applies') }}</h2>
    <p v-if="!(applies?.list || []).length" class="muted">{{ $t('ui.no_apply') }}</p>
    <div class="stack">
      <article v-for="row in applies?.list || []" :key="row.id" class="job-card">
        <h3>uid {{ row.uid }} · job_id {{ row.job_id }}</h3>
        <p class="muted">status {{ row.status }} {{ row.status_n }}</p>
        <button type="button" @click="setApply(row.id, 2)">{{ $t('common.yes') }}</button>
        <button type="button" @click="setApply(row.id, 3)">{{ $t('common.phone') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
