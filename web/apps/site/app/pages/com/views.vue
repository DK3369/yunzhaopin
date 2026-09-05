<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-look-resumes', () =>
  api.post('/v1/mcenter/look-resumes/mine', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('member_com_00006') })
</script>

<template>
  <section>
    <h1>{{ $t('member_com_00006') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_views') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink :to="`/resumes/${row.resume_id || row.uid}`">{{ row.resume_name || row.uid }}</NuxtLink>
        </h3>
        <p class="muted">{{ row.datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
