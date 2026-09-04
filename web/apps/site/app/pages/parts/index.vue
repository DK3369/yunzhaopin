<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const { t } = useI18n()
const api = useApi()
const { applyToQuery } = useSubSite()
const { data, error } = await useAsyncData(
  () => `parts-${page.value}-${keyword.value}`,
  () =>
    api.get<{ list: Array<{ id: number; name: string; com_name?: string; city_name?: string }>; total: number }>(
      '/v1/wap/parts',
      applyToQuery({
        page: page.value,
        page_size: 20,
        keyword: keyword.value || undefined,
        part_type: Number(route.query.part_type || 0) || undefined,
      }),
    ),
)
useSeoMeta({ title: t('wap_com_00311') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
useListLoginGate(error)
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('wap_com_00311')" :error="error" :error-text="failMsg" :count="list.length">
    <form class="form" method="get" action="/parts">
      <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <SimpleCard
      v-for="row in list"
      :key="row.id"
      :to="`/parts/${row.id}`"
      :title="row.name"
      :meta="`${row.com_name || ''} · ${row.city_name || ''}`"
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

