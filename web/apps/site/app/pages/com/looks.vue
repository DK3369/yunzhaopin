<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-look-jobs', () =>
  api.post('/v1/mcenter/look-jobs/list', { page: 1, page_size: 20 }),
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
useSeoMeta({ title: t('member_com_00007') })
</script>

<template>
  <section>
    <h1>{{ $t('member_com_00007') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.job_name || row.job_id }}</h3>
        <p class="muted">
          <NuxtLink v-if="row.uid" :to="`/resumes/${row.eid || row.uid}`">{{ row.uname || row.uid }}</NuxtLink>
          · {{ row.datetime_n }}
        </p>
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`">{{ $t('wap_com_00427') }}</NuxtLink>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
