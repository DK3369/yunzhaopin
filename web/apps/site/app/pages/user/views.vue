<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('look-resumes', () =>
  api.post('/v1/mcenter/look-resumes/list', { page: 1, page_size: 20 }),
)
const msg = ref('')
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/look-resumes/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00276') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00276') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_who_viewed') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`">{{ row.com_name || row.com_id }}</NuxtLink>
          <span v-else>{{ row.com_name || row.id }}</span>
        </h3>
        <p class="muted">{{ row.com_job }} <template v-if="row.com_job_num">· {{ row.com_job_num }}</template></p>
        <p class="muted">{{ row.datetime_n }}</p>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
