<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('resume-downloads-inbox', () =>
  api.post('/v1/mcenter/resume-downloads/inbox', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('admin_user_00263') })
</script>

<template>
  <section>
    <h1>{{ $t('admin_user_00263') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`">{{ row.uname || row.com_id }}</NuxtLink>
          <span v-else>{{ row.uname || row.com_id || row.id }}</span>
        </h3>
        <p class="muted">{{ row.datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
