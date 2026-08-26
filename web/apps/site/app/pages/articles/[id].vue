<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`article-${id}`, () =>
  api.get('/v1/wap/articles/detail', { id }),
)
const article = computed(() => (data.value || {}) as Record<string, unknown>)
useSeoMeta({
  title: () => String(article.value.title || t('common.article')),
  description: () =>
    stripHtml(article.value.description || article.value.content || article.value.body || article.value.title),
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
            description: stripHtml(article.value.description || article.value.content || article.value.body),
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
    <p v-else class="muted">{{ $t('home.no_job_data') }}</p>
  </article>
</template>
