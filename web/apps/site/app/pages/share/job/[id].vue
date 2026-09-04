<script setup lang="ts">
import { seoJoin } from '~/utils/seo'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { settings } = useSiteChrome()
const shareOn = computed(() => String(settings.value.sy_h5_share || '1') !== '2')
const { data } = await useAsyncData(`share-job-${id}`, () => api.get('/v1/wap/jobs/detail', { id }))
const job = computed(
  () => ((data.value as { job?: Record<string, unknown> } | null)?.job || {}) as Record<string, unknown>,
)
useSeoMeta({
  title: () => String(job.value.name || t('common.share')),
  description: () => seoJoin([job.value.description, job.value.com_name, job.value.name]),
})
</script>

<template>
  <article v-if="shareOn">
    <h1>{{ job.name || $t('common.job') }}</h1>
    <p>{{ job.com_name }}</p>
    <p><NuxtLink :to="`/jobs/${id}`">{{ $t('common.more') }}</NuxtLink></p>
  </article>
  <p v-else class="muted">{{ $t('common_02409') }}</p>
</template>
