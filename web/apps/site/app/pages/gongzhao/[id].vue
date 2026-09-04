<script setup lang="ts">
import { seoJoin } from '~/utils/seo'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`gz-${id}`, () => api.get('/v1/wap/gongzhao/detail', { id }))
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const prev = computed(() => (row.value.prev || null) as { id?: number; title?: string } | null)
const next = computed(() => (row.value.next || null) as { id?: number; title?: string } | null)
useSeoMeta({
  title: () => String(row.value.title || t('ui.gongzhao')),
  description: () => seoJoin([row.value.body, row.value.title]),
})
useHead({ link: [{ rel: 'canonical', href: `/gongzhao/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ row.title || $t('common_02409') }}</h1>
    <p v-if="row.start_at_n" class="muted">{{ row.start_at_n }} — {{ row.end_at_n }}</p>
    <div v-if="row.body" v-html="String(row.body)" />
    <p v-else-if="!row.title" class="muted">{{ $t('common_02409') }}</p>
    <p class="muted">
      <NuxtLink v-if="prev?.id" :to="`/gongzhao/${prev.id}`">{{ $t('default_00326') }} {{ prev.title }}</NuxtLink>
      <NuxtLink v-if="next?.id" :to="`/gongzhao/${next.id}`">{{ $t('default_00327') }} {{ next.title }}</NuxtLink>
    </p>
  </article>
</template>
