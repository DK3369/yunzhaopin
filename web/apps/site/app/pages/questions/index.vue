<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `questions-${page.value}`,
  () =>
    api.get<{ list: Array<{ id: number; title: string; catname?: string; answer_count?: number }>; total: number }>(
      '/v1/wap/questions',
      { page: page.value, page_size: 20 },
    ),
)
useSeoMeta({ title: t('wap_00160') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('wap_00160')" :error="error" :error-text="failMsg" :count="list.length">
    <SimpleCard
      v-for="row in list"
      :key="row.id"
      :to="`/questions/${row.id}`"
      :title="row.title"
      :meta="`${row.catname || ''} · ${row.answer_count || 0}`"
    />
    <template #pager>
      <Pager
        :page="page"
        :page-size="20"
        :total="data?.total || 0"
        @update:page="(p) => navigateTo({ query: { page: p } })"
      />
    </template>
  </NewsListShell>
</template>
