<script setup lang="ts">
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData('articles', () =>
  api.get('/v1/wap/articles', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('common.article') })
</script>

<template>
  <section>
    <h1>{{ $t('common.article') }}</h1>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
    <div class="yun_content news_list_box">
      <div class="index_news_box">
        <ul class="index_news_list_list">
          <li v-for="a in data?.list || []" :key="a.id">
            <NuxtLink :to="`/articles/${a.id}`">
              <i class="index_news_list_icon" />{{ a.title }}
            </NuxtLink>
            <em>{{ a.datetime_n || a.published_at_n }}</em>
          </li>
        </ul>
      </div>
    </div>
  </section>
</template>
