<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('look-jobs-mine', () =>
  api.post('/v1/mcenter/look-jobs/mine', { page: 1, page_size: 20 }),
)
const msg = ref('')
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/look-jobs/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00275') })
</script>

<template>
  <section>
    <nav class="m_tab">
      <NuxtLink to="/user/views">{{ $t('wap_user_00276') }}</NuxtLink>
      <NuxtLink to="/user/looks">{{ $t('wap_user_00275') }}</NuxtLink>
    </nav>
    <h1>{{ $t('wap_user_00275') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`">{{ row.job_name || $t('common.job') }}</NuxtLink>
          <span v-else>{{ row.job_name || row.id }}</span>
        </h3>
        <p class="muted">{{ row.com_name }} · {{ row.datetime_n }}</p>
        <p v-if="row.minsalary || row.maxsalary" class="muted">{{ row.minsalary }} - {{ row.maxsalary }}</p>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
