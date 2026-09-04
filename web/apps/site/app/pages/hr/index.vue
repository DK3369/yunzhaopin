<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const cid = computed(() => Number(route.query.cid || 0) || undefined)
const keyword = computed(() => String(route.query.keyword || ''))
const order = computed(() => String(route.query.order || ''))
const { t } = useI18n()
const api = useApi()
const { data: classes } = await useAsyncData('hr-classes', () =>
  api.get<Array<{ id: number; name: string }>>('/v1/wap/hr-docs/classes').catch(() => []),
)
const { data, error } = await useAsyncData(
  () => `hr-${page.value}-${cid.value || 0}-${keyword.value}-${order.value}`,
  () =>
    api.get<{ list: Array<{ id: number; name: string; created_at_n?: string; hits?: number }>; total: number }>(
      '/v1/wap/hr-docs',
      {
        page: page.value,
        page_size: 20,
        cid: cid.value,
        keyword: keyword.value || undefined,
        order: order.value || undefined,
      },
    ),
)
useSeoMeta({ title: t('default_00138') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('default_00138')" :error="error" :error-text="failMsg" :count="list.length">
    <form class="form" method="get" action="/hr">
      <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <p>
      <NuxtLink to="/hr">{{ $t('common.all') }}</NuxtLink>
      <NuxtLink v-for="c in classes || []" :key="c.id" :to="{ query: { cid: c.id } }">{{ c.name }}</NuxtLink>
      <NuxtLink :to="{ query: { order: 'hits' } }">{{ $t('common.hot') }}</NuxtLink>
    </p>
    <SimpleCard v-for="row in list" :key="row.id" :to="`/hr/${row.id}`" :title="row.name" :meta="row.created_at_n" />
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
