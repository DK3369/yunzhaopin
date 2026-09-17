<script setup lang="ts">
import { seoJoin } from '~/utils/seo'
import { ensurePublicFound } from '~/utils/site'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(`ann-${id}`, () =>
  api.get('/v1/wap/announcements/detail', { id }),
)
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const prev = computed(() => (row.value.prev || null) as { id?: number; title?: string } | null)
const next = computed(() => (row.value.next || null) as { id?: number; title?: string } | null)
ensurePublicFound(Boolean(row.value.title || row.value.id), error.value)
useSeoMeta({
  title: () => String(row.value.title || t('ui.announcements')),
  description: () => seoJoin([row.value.description, row.value.content, row.value.title]),
})
</script>

<template>
  <article class="site-pc">
    <h1>{{ row.title || $t('ui.announcements') }}</h1>
    <div v-if="row.content || row.body" v-html="String(row.content || row.body)" />
    <p v-else class="muted">{{ $t('wap_00129') }}</p>
    <p class="muted">
      <NuxtLink v-if="prev?.id" :to="`/announcements/${prev.id}`">{{ $t('default_00326') }} {{ prev.title }}</NuxtLink>
      <NuxtLink v-if="next?.id" :to="`/announcements/${next.id}`">{{ $t('default_00327') }} {{ next.title }}</NuxtLink>
    </p>
  </article>
  <div class="site-h5 news_cont_box">
    <div class="news_cont_box_tit"><h1>{{ row.title || $t('ui.announcements') }}</h1></div>
    <div class="wap_news_cont">
      <div v-if="row.content || row.body" class="wap_txt" v-html="String(row.content || row.body)" />
      <p v-else class="muted">{{ $t('wap_00129') }}</p>
    </div>
    <p class="muted">
      <NuxtLink v-if="prev?.id" :to="`/announcements/${prev.id}`">{{ $t('default_00326') }} {{ prev.title }}</NuxtLink>
      <NuxtLink v-if="next?.id" :to="`/announcements/${next.id}`">{{ $t('default_00327') }} {{ next.title }}</NuxtLink>
    </p>
  </div>
</template>
