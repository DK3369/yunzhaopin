<script setup lang="ts">
import { seoJoin } from '~/utils/seo'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`ann-${id}`, () =>
  api.get('/v1/wap/announcements/detail', { id }),
)
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const prev = computed(() => (row.value.prev || null) as { id?: number; title?: string } | null)
const next = computed(() => (row.value.next || null) as { id?: number; title?: string } | null)
useSeoMeta({
  title: () => String(row.value.title || t('ui.announcements')),
  description: () => seoJoin([row.value.description, row.value.content, row.value.title]),
})
</script>

<template>
  <article>
    <h1>{{ row.title || $t('ui.announcements') }}</h1>
    <div v-if="row.content || row.body" v-html="String(row.content || row.body)" />
    <p v-else class="muted">{{ $t('wap_00129') }}</p>
    <p class="muted">
      <NuxtLink v-if="prev?.id" :to="`/announcements/${prev.id}`">{{ $t('default_00326') }} {{ prev.title }}</NuxtLink>
      <NuxtLink v-if="next?.id" :to="`/announcements/${next.id}`">{{ $t('default_00327') }} {{ next.title }}</NuxtLink>
    </p>
  </article>
</template>
