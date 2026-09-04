<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('look-jobs', () =>
  api.post('/v1/mcenter/my-views', { kind: 1, page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_user_00275') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00275') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink v-if="row.target_id" :to="`/jobs/${row.target_id}`">{{ row.kind_n || $t('common.job') }} {{ row.target_id }}</NuxtLink>
          <span v-else>#{{ row.id }}</span>
        </h3>
        <p class="muted">{{ row.datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
