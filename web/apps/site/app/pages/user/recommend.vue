<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('recommend-jobs', () =>
  api.post('/v1/mcenter/recommend/jobs', { limit: 20 }).catch(() => []),
)
const list = computed(() => (Array.isArray(data.value) ? data.value : data.value?.list || data.value || []))
useSeoMeta({ title: t('wap_user_00211') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00211') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!list.length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>
          <NuxtLink :to="`/jobs/${row.id}`">{{ row.name }}</NuxtLink>
        </h3>
        <p class="muted">
          <NuxtLink v-if="row.uid" :to="`/companies/${row.uid}`">{{ row.com_name }}</NuxtLink>
          · {{ row.min_salary }} - {{ row.max_salary }}
        </p>
      </article>
    </div>
  </section>
</template>
