<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('eval-logs-mine', () =>
  api.post('/v1/mcenter/eval-logs', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_00194') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00194') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink v-if="row.paper_id" :to="`/eval/${row.paper_id}`">{{ $t('wap_00194') }} #{{ row.paper_id }}</NuxtLink>
          <span v-else>{{ row.id }}</span>
        </h3>
        <p>{{ row.score }}</p>
        <p class="muted">{{ row.created_at_n }}</p>
      </article>
    </div>
  </section>
</template>
