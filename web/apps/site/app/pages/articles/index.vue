<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const { t } = useI18n()
const api = useApi()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const category = computed(() => String(route.query.category || route.query.nid || ''))
const { data, error } = await useAsyncData(
  () => `articles-${page.value}-${keyword.value}-${category.value}`,
  () =>
    api.get<{ list: Array<{ id: number; title: string; datetime_n?: string; published_at_n?: string }>; total: number }>(
      '/v1/wap/articles',
      {
        page: page.value,
        page_size: 20,
        keyword: keyword.value || undefined,
        category: category.value || undefined,
      },
    ),
)
useSeoMeta({ title: keyword.value ? `${keyword.value} - ${t('common.article')}` : t('common.article') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('common.article')" :error="error" :error-text="failMsg" :count="list.length">
    <form class="form" method="get" action="/articles">
      <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
      <input v-if="category" type="hidden" name="category" :value="category" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <SimpleCard
      v-for="a in list"
      :key="a.id"
      :to="`/articles/${a.id}`"
      :title="a.title"
      :meta="a.datetime_n || a.published_at_n"
    />
    <template #pager>
      <Pager
        :page="page"
        :page-size="20"
        :total="data?.total || 0"
        @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
      />
    </template>
  </NewsListShell>
</template>
