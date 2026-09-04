<script setup lang="ts">
import { seoJoin } from '~/utils/seo'
import type { CompanyLike, JobLike } from '~/utils/site'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const tab = computed(() => String(useRoute().query.tab || 'intro'))
const { data } = await useAsyncData(`fair-${id}`, () => api.get('/v1/wap/zph/detail', { id }))
const { data: companies } = await useAsyncData(`fair-com-${id}`, () =>
  api.get<{ list?: CompanyLike[] }>('/v1/wap/zph/companies', { id, page: 1, page_size: 20 }).catch(() => ({ list: [] })),
)
const { data: jobs } = await useAsyncData(`fair-job-${id}`, async () => {
  const raw = await api.get<JobLike[] | { list?: JobLike[] }>('/v1/wap/zph/jobs', { id, page: 1, page_size: 20 }).catch(() => [])
  if (Array.isArray(raw)) return raw
  return raw?.list || []
})
useSeoMeta({
  title: () => String(data.value?.title || t('ui.fairs')),
  description: () => seoJoin([data.value?.address, data.value?.body, data.value?.title]),
})
useHead({ link: [{ rel: 'canonical', href: `/fairs/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || $t('zph_00001') }}</h1>
    <p v-if="data?.address" class="muted">{{ data.address }} · {{ data.start_at_n }}</p>
    <p>
      <NuxtLink :to="{ query: { tab: 'intro' } }">{{ $t('common.home') }}</NuxtLink>
      <NuxtLink :to="{ query: { tab: 'companies' } }">{{ $t('default_00114') }}</NuxtLink>
      <NuxtLink :to="{ query: { tab: 'jobs' } }">{{ $t('default_00246') }}</NuxtLink>
    </p>
    <div v-if="tab === 'companies'">
      <p v-if="!(companies?.list || []).length" class="muted">{{ $t('wap_00590') }}</p>
      <CompanyCard v-for="c in companies?.list || []" :key="c.uid" :company="c" />
    </div>
    <div v-else-if="tab === 'jobs'">
      <p v-if="!(jobs || []).length" class="muted">{{ $t('default_00033') }}</p>
      <JobCard v-for="j in jobs || []" :key="j.id" :job="j" />
    </div>
    <template v-else>
      <div v-if="data?.body" v-html="data.body" />
      <p v-else-if="!data?.title" class="muted">{{ $t('wap_00603') }}</p>
    </template>
  </article>
</template>
