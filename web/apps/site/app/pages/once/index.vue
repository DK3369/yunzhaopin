<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `once-${page.value}-${keyword.value}`,
  () =>
    api.get<{ list: Array<{ id: number; title?: string; companyname: string; mans?: string; number?: string }>; total: number }>('/v1/wap/once-jobs/list', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
    }),
)
useSeoMeta({ title: t('wap_js_00130') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('wap_js_00130')" :error="error" :error-text="failMsg" :count="list.length">
    <form class="form" method="get" action="/once">
      <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <SimpleCard v-for="row in list" :key="row.id" :to="`/once/${row.id}`" :title="row.title || row.companyname" :meta="row.mans || row.number" />
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
