<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('my-apps', () =>
  api.post('/v1/mcenter/my-applications', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])
const msg = ref('')
async function withdraw(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-applications/withdraw', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.applications') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.applications') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!list.length" class="muted">{{ $t('ui.no_applies') }}</p>
    <div v-else class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>{{ $t('common.job') }} #{{ row.job_id }}</h3>
        <p class="muted">{{ row.datetime_n }}</p>
        <NuxtLink :to="`/jobs/${row.job_id}`">{{ $t('wap_com_00427') }}</NuxtLink>
        <button v-if="!row.quxiao" type="button" @click="withdraw(row.id)">{{ $t('common.cancel') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
