<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const kw = ref(keyword.value)
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `fairs-${page.value}-${keyword.value}`,
  () => api.get<{ list: Array<{ id: number; title: string; city_name?: string; start_at_n?: string }>; total: number }>('/v1/wap/zph', {
    page: page.value,
    page_size: 20,
    keyword: keyword.value || undefined,
  }),
)
useSeoMeta({ title: t('wap_00558') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('wap_00558')" :error="error" :error-text="failMsg" :count="list.length">
    <form class="yun_bth_box" @submit.prevent="navigateTo({ query: { keyword: kw || undefined, page: 1 } })">
      <input v-model="kw" :placeholder="$t('common.search')" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <SimpleCard
      v-for="row in list"
      :key="row.id"
      :to="`/fairs/${row.id}`"
      :title="row.title"
      :meta="`${row.city_name || ''} · ${row.start_at_n || ''}`"
    />
    <template #pager>
      <Pager
        :page="page"
        :page-size="20"
        :total="data?.total || 0"
        @update:page="(p) => navigateTo({ query: { page: p, keyword: keyword || undefined } })"
      />
    </template>
  </NewsListShell>
</template>
