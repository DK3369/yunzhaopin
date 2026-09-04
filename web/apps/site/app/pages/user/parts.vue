<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: applies, error } = await useAsyncData('my-part-applies', () =>
  api.post('/v1/mcenter/my-part-applications/list', { page: 1, page_size: 20 }),
)
const { data: collects } = await useAsyncData('my-part-collects', () =>
  api.post('/v1/mcenter/my-part-collects/list', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('ui.part_fav') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.part_fav') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <h2>{{ $t('ui.apply') }}</h2>
    <p v-if="!(applies?.list || []).length" class="muted">{{ $t('ui.no_apply') }}</p>
    <div class="stack">
      <article v-for="row in applies?.list || []" :key="row.id" class="job-card">
        <h3>job_id {{ row.job_id }}</h3>
        <p class="muted">status {{ row.status }} · {{ row.ctime_n }}</p>
      </article>
    </div>
    <h2>{{ $t('ui.my_fav') }}</h2>
    <p v-if="!(collects?.list || []).length" class="muted">{{ $t('ui.no_fav') }}</p>
    <div class="stack">
      <article v-for="row in collects?.list || []" :key="row.id" class="job-card">
        <h3>job_id {{ row.job_id }}</h3>
        <p class="muted">{{ row.ctime_n }}</p>
      </article>
    </div>
  </section>
</template>
