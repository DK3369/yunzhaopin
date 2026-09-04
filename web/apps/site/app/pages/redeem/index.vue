<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const nid = computed(() => Number(route.query.nid || 0) || undefined)
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `redeem-${page.value}-${nid.value || 0}`,
  () =>
    api.get<{ list: Array<{ id: number; name: string; integral?: number }>; total: number }>('/v1/wap/redeem/rewards', {
      page: page.value,
      page_size: 20,
      nid: nid.value,
    }),
)
const { data: classes } = await useAsyncData('redeem-classes', () =>
  api.get<Array<{ id: number; name: string; parent_id?: number }>>('/v1/wap/redeem/classes').catch(() => []),
)
const classItems = computed(() =>
  (classes.value || [])
    .filter((c) => !c.parent_id)
    .map((c) => ({ id: c.id, name: c.name })),
)
useSeoMeta({ title: t('common_06524') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <section>
    <p><NuxtLink to="/redeem/orders">{{ $t('common.more') }}</NuxtLink></p>
    <FilterRow
      v-if="classItems.length"
      :label="$t('common_06524')"
      param="nid"
      :items="classItems"
      :current="nid"
      path="/redeem"
      :all-label="$t('common.all')"
    />
    <NewsListShell :title="$t('common_06524')" :error="error" :error-text="failMsg" :count="list.length">
      <SimpleCard v-for="row in list" :key="row.id" :to="`/redeem/${row.id}`" :title="row.name" :meta="String(row.integral || '')" />
      <template #pager>
        <Pager
          :page="page"
          :page-size="20"
          :total="data?.total || 0"
          @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
        />
      </template>
    </NewsListShell>
  </section>
</template>
