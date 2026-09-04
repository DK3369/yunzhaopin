<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-fans', () =>
  api.post('/v1/mcenter/fans', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_com_00407') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_com_00407') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.uid" class="job-card">
        <h3>
          <NuxtLink :to="`/resumes/${row.uid}`">{{ row.username || row.uid }}</NuxtLink>
        </h3>
        <p class="muted">{{ row.last_datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
