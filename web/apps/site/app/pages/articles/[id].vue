<script setup lang="ts">
import { seoJoin, unixToIso } from '~/utils/seo'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`article-${id}`, () =>
  api.get('/v1/wap/articles/detail', { id }),
)
const article = computed(() => (data.value || {}) as Record<string, unknown>)
const prev = computed(() => (article.value.prev || null) as { id?: number; title?: string } | null)
const next = computed(() => (article.value.next || null) as { id?: number; title?: string } | null)
const related = computed(
  () => (Array.isArray(article.value.related) ? article.value.related : []) as Array<{ id: number; title: string }>,
)
useSeoMeta({
  title: () => String(article.value.title || t('common.article')),
  description: () => seoJoin([article.value.summary, article.value.content, article.value.body, article.value.title]),
  keywords: () => String(article.value.keyword || article.value.title || ''),
})
useHead({
  link: [{ rel: 'canonical', href: `/articles/${id}` }],
  script: article.value.title
    ? [
        {
          type: 'application/ld+json',
          innerHTML: JSON.stringify({
            '@context': 'https://schema.org',
            '@type': 'NewsArticle',
            headline: article.value.title,
            description: seoJoin([article.value.summary, article.value.content, article.value.body]),
            datePublished: unixToIso(article.value.datetime || article.value.published_at || article.value.ctime),
          }),
        },
      ]
    : [],
})
</script>

<template>
  <article>
    <h1>{{ article.title || $t('ui.article_missing') }}</h1>
    <div v-if="article.content || article.body" v-html="String(article.content || article.body)" />
    <p v-else class="muted">{{ $t('common_02409') }}</p>
    <p class="muted">
      <NuxtLink v-if="prev?.id" :to="`/articles/${prev.id}`">{{ $t('default_00326') }} {{ prev.title }}</NuxtLink>
      <NuxtLink v-if="next?.id" :to="`/articles/${next.id}`">{{ $t('default_00327') }} {{ next.title }}</NuxtLink>
    </p>
    <div v-if="related.length">
      <h2>{{ $t('wap_01474') }}</h2>
      <SimpleCard v-for="row in related" :key="row.id" :to="`/articles/${row.id}`" :title="row.title" />
    </div>
  </article>
</template>
