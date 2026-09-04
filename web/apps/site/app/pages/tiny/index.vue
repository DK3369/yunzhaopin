<script setup lang="ts">
import { listFailMsg } from '~/utils/site'
import type { DictItem } from '~/utils/query'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const provinceId = computed(() => Number(route.query.province_id || 0) || undefined)
const cityId = computed(() => Number(route.query.city_id || 0) || undefined)
const threeCityId = computed(() => Number(route.query.three_city_id || 0) || undefined)
const { t, locale } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `tiny-${page.value}-${keyword.value}-${provinceId.value || 0}-${cityId.value || 0}-${threeCityId.value || 0}`,
  () =>
    api.get<{ list: Array<{ id: number; username: string; job?: string }>; total: number }>('/v1/wap/tiny-resumes/list', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
      province_id: provinceId.value,
      city_id: cityId.value,
      three_city_id: threeCityId.value,
    }),
)
const { data: provinces } = await useAsyncData(
  () => `dict-city-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/cities').catch(() => [] as DictItem[]),
)
const { data: cities } = await useAsyncData(
  () => `dict-city-child-${locale.value}-${provinceId.value || 0}`,
  () =>
    provinceId.value
      ? api.get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: provinceId.value }).catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
const { data: districts } = await useAsyncData(
  () => `dict-city-dist-${locale.value}-${cityId.value || 0}`,
  () =>
    cityId.value
      ? api.get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: cityId.value }).catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
useSeoMeta({ title: t('wap_js_00066') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('wap_js_00066')" :error="error" :error-text="failMsg" :count="list.length">
    <form class="form" method="get" action="/tiny">
      <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
      <input v-if="provinceId" type="hidden" name="province_id" :value="provinceId" />
      <input v-if="cityId" type="hidden" name="city_id" :value="cityId" />
      <input v-if="threeCityId" type="hidden" name="three_city_id" :value="threeCityId" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <FilterRow
      :label="$t('member_com_00378')"
      param="province_id"
      :items="provinces || []"
      :current="provinceId"
      path="/tiny"
      :all-label="$t('common.all')"
    />
    <FilterRow
      v-if="provinceId && (cities || []).length"
      :label="$t('common_02110')"
      param="city_id"
      :items="cities || []"
      :current="cityId"
      path="/tiny"
      :all-label="$t('common.all')"
    />
    <FilterRow
      v-if="cityId && (districts || []).length"
      :label="$t('member_com_00378')"
      param="three_city_id"
      :items="districts || []"
      :current="threeCityId"
      path="/tiny"
      :all-label="$t('common.all')"
    />
    <p><NuxtLink to="/tiny/add">{{ $t('common.publish') }}</NuxtLink></p>
    <SimpleCard v-for="row in list" :key="row.id" :to="`/tiny/${row.id}`" :title="row.username" :meta="row.job" />
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
