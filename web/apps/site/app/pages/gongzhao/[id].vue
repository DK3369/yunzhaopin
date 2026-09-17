<script setup lang="ts">
import { seoJoin } from '~/utils/seo'
import { ensurePublicFound } from '~/utils/site'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(`gz-${id}`, () => api.get('/v1/wap/gongzhao/detail', { id }))
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const prev = computed(() => (row.value.prev || null) as { id?: number; title?: string } | null)
const next = computed(() => (row.value.next || null) as { id?: number; title?: string } | null)
ensurePublicFound(Boolean(row.value.title || row.value.id), error.value)
useSeoMeta({
  title: () => String(row.value.title || t('ui.gongzhao')),
  description: () => seoJoin([row.value.body, row.value.title]),
})
useHead({ link: [{ rel: 'canonical', href: `/gongzhao/${id}` }] })
</script>

<template>
  <article class="site-pc">
    <h1>{{ row.title || $t('common_02409') }}</h1>
    <p v-if="row.start_at_n" class="muted">{{ row.start_at_n }} — {{ row.end_at_n }}</p>
    <div v-if="row.body" v-html="String(row.body)" />
    <p v-else-if="!row.title" class="muted">{{ $t('common_02409') }}</p>
    <p>
      <NuxtLink :to="`/poster/gongzhao/${id}`">{{ $t('ui.poster') }}</NuxtLink>
    </p>
    <p class="muted">
      <NuxtLink v-if="prev?.id" :to="`/gongzhao/${prev.id}`">{{ $t('default_00326') }} {{ prev.title }}</NuxtLink>
      <NuxtLink v-if="next?.id" :to="`/gongzhao/${next.id}`">{{ $t('default_00327') }} {{ next.title }}</NuxtLink>
    </p>
  </article>
  <div class="site-h5 news_cont_box">
    <div class="news_cont_box_tit"><h1>{{ row.title || $t('common_02409') }}</h1></div>
    <div class="news_cont_ms">{{ row.start_at_n }} — {{ row.end_at_n }}</div>
    <div class="wap_news_cont">
      <div v-if="row.body" class="wap_txt" v-html="String(row.body)" />
      <p v-else-if="!row.title" class="muted">{{ $t('common_02409') }}</p>
    </div>
    <p>
      <NuxtLink :to="`/poster/gongzhao/${id}`">{{ $t('ui.poster') }}</NuxtLink>
    </p>
    <p class="muted">
      <NuxtLink v-if="prev?.id" :to="`/gongzhao/${prev.id}`">{{ $t('default_00326') }} {{ prev.title }}</NuxtLink>
      <NuxtLink v-if="next?.id" :to="`/gongzhao/${next.id}`">{{ $t('default_00327') }} {{ next.title }}</NuxtLink>
    </p>
  </div>
</template>
