<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `tiny-${page.value}-${keyword.value}`,
  () =>
    api.get<{ list: Array<{ id: number; username: string; job?: string }>; total: number }>('/v1/wap/tiny-resumes/list', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
    }),
)
useSeoMeta({ title: t('wap_js_00066') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('wap_js_00066')" :error="error" :error-text="failMsg" :count="list.length">
    <form class="form" method="get" action="/tiny">
      <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <p><NuxtLink to="/tiny/add">{{ $t('common.publish') }}</NuxtLink></p>
    <SimpleCard v-for="row in list" :key="row.id" :to="`/tiny/${row.id}`" :title="row.username" :meta="row.job" />
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
