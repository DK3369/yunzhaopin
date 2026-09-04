<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: bal, error } = await useAsyncData('integral-bal', () =>
  api.post('/v1/mcenter/integral/balance', {}),
)
const { data: hist } = await useAsyncData('integral-hist', () =>
  api.post('/v1/mcenter/integral/history', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('ui.my_integral') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.my_integral') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else>{{ $t('ui.balance') }} {{ bal?.balance ?? 0 }}</p>
    <h2>{{ $t('ui.flow') }}</h2>
    <p v-if="!(hist?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div class="stack">
      <article v-for="(row, i) in hist?.list || []" :key="i" class="job-card">
        <pre>{{ JSON.stringify(row) }}</pre>
      </article>
    </div>
  </section>
</template>
