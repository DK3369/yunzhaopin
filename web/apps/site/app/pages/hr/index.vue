<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const cid = computed(() => Number(route.query.cid || 0) || undefined)
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `hr-${page.value}-${cid.value || 0}`,
  () =>
    api.get<{ list: Array<{ id: number; name: string; created_at_n?: string }>; total: number }>('/v1/wap/hr-docs', {
      page: page.value,
      page_size: 20,
      cid: cid.value,
    }),
)
useSeoMeta({ title: t('default_00138') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('default_00138')" :error="error" :error-text="failMsg" :count="list.length">
    <SimpleCard v-for="row in list" :key="row.id" :to="`/hr/${row.id}`" :title="row.name" :meta="row.created_at_n" />
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
