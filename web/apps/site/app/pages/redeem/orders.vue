<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const { t } = useI18n()
const api = useApi()
const { me } = useSiteChrome()
const page = computed(() => Number(useRoute().query.page || 1))
if (!me.value) {
  await navigateTo({ path: '/login', query: { next: '/redeem/orders' } })
}
const { data, error } = await useAsyncData(
  () => `redeem-orders-${page.value}`,
  () =>
    api.post<{ list: Array<{ id: number; name: string; status_n?: string; created_at_n?: string }>; total: number }>(
      '/v1/mcenter/redeem/orders',
      { page: page.value, page_size: 20 },
    ),
)
useSeoMeta({ title: t('common_06524') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('common_06524')" :error="error" :error-text="failMsg" :count="list.length">
    <SimpleCard v-for="row in list" :key="row.id" :title="row.name" :meta="`${row.status_n || ''} · ${row.created_at_n || ''}`" />
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
