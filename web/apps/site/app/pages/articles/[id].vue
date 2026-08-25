<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`article-${id}`, () =>
  api.get('/v1/wap/articles/detail', { id }),
)
const article = computed(() => (data.value || {}) as Record<string, unknown>)
useSeoMeta({
  title: () => String(article.value.title || '文章'),
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
    <h1>{{ article.title || '文章不存在' }}</h1>
    <div v-if="article.content || article.body" v-html="String(article.content || article.body)" />
    <p v-else class="muted">没有这篇文章，或暂时无法加载。</p>
  </article>
</template>
