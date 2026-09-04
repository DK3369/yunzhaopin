<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `once-${page.value}`,
  () =>
    api.get<{ list: Array<{ id: number; companyname: string; number?: string }>; total: number }>('/v1/wap/once-jobs/list', {
      page: page.value,
      page_size: 20,
    }),
)
useSeoMeta({ title: t('wap_js_00130') })
const failMsg = computed(() => listFailMsg(error.value, t('common_00376'), t('common_00376')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('wap_js_00130')" :error="error" :error-text="failMsg" :count="list.length">
    <SimpleCard v-for="row in list" :key="row.id" :to="`/once/${row.id}`" :title="row.companyname" :meta="row.number" />
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
