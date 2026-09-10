<script setup lang="ts">
import { seoJoin } from '~/utils/seo'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { settings } = useSiteChrome()
const shareOn = computed(() => String(settings.value.sy_h5_share || '1') !== '2')
const { data } = await useAsyncData(`share-company-${id}`, () =>
  api.post<{ url?: string; id?: number }>('/v1/wap/share/companies', { uid: id }).catch(() => null),
)
const { data: company } = await useAsyncData(`share-company-detail-${id}`, () =>
  api.get('/v1/wap/companies/detail', { uid: id }).catch(() => null),
)
const name = computed(() => String((company.value as { name?: string } | null)?.name || t('common.company')))
const href = computed(() => {
  const origin = String(useRuntimeConfig().public.siteUrl || '').replace(/\/$/, '')
  return `${origin}/companies/${id}`
})
useSeoMeta({
  title: () => name.value,
  description: () => seoJoin([name.value, data.value?.url]),
})
</script>

<template>
  <article v-if="shareOn">
    <h1>{{ name }}</h1>
    <p v-if="data?.url" class="muted">{{ data.url }}</p>
    <ShareSceneQr kind="company" :id="id" :href="href" />
    <p><NuxtLink :to="`/companies/${id}`">{{ $t('common.more') }}</NuxtLink></p>
  </article>
  <p v-else class="muted">{{ $t('common_02409') }}</p>
</template>
