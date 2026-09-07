<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const country = computed(() => countryQuery(route.query.country))
const provinceId = computed(() => Number(route.query.province_id || 0) || undefined)
const cityId = computed(() => Number(route.query.city_id || 0) || undefined)
const threeCityId = computed(() => Number(route.query.three_city_id || 0) || undefined)
const { t } = useI18n()
const api = useApi()
const { countryItems, provinceItems, cityItems, districtItems } = await useRegionCascade({
  country,
  provinceId: computed(() => provinceId.value || 0),
  cityId: computed(() => cityId.value || 0),
})
const { data, error } = await useAsyncData(
  () => `once-${page.value}-${keyword.value}-${country.value}-${provinceId.value || 0}-${cityId.value || 0}-${threeCityId.value || 0}`,
  () =>
    api.get<{ list: Array<{ id: number; title?: string; companyname: string; mans?: string; number?: string }>; total: number }>('/v1/wap/once-jobs/list', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
      country: country.value || undefined,
      province_id: provinceId.value,
      city_id: cityId.value,
      three_city_id: threeCityId.value,
    }),
)
useSeoMeta({ title: t('wap_js_00130') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
const { data: paylog, refresh: refreshPaylog } = await useAsyncData(
  'once-paylog',
  async () => {
    const f = import.meta.client ? String(localStorage.getItem('once_fast') || '') : ''
    if (!f) return { list: [] as Array<{ id: number; order_id: string; order_price: number; once_id?: number }> }
    return api.post<{ list: Array<{ id: number; order_id: string; order_price: number; once_id?: number }> }>(
      '/v1/wap/once-jobs/paylog',
      { fast: f, page: 1, page_size: 20 },
    ).catch(() => ({ list: [] }))
  },
)
async function cancelPay(id: number) {
  const f = String(localStorage.getItem('once_fast') || '')
  if (!f) return
  await api.post('/v1/wap/once-jobs/paylog', { fast: f, id })
  await refreshPaylog()
}
</script>

<template>
  <section>
    <form class="form" method="get" action="/once">
      <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
      <input v-if="country" type="hidden" name="country" :value="country" />
      <input v-if="provinceId" type="hidden" name="province_id" :value="provinceId" />
      <input v-if="cityId" type="hidden" name="city_id" :value="cityId" />
      <input v-if="threeCityId" type="hidden" name="three_city_id" :value="threeCityId" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <CountryFilterRow
      :label="$t('common.country')"
      :items="countryItems"
      :current="country"
      path="/once"
      :all-label="$t('common.all')"
    />
    <FilterRow
      v-if="country"
      :label="$t('member_com_00378')"
      param="province_id"
      :items="provinceItems"
      :current="provinceId"
      path="/once"
      :all-label="$t('common.all')"
      :extra-clear="{ city_id: undefined, three_city_id: undefined }"
    />
    <FilterRow
      v-if="provinceId && cityItems.length"
      :label="$t('common_02110')"
      param="city_id"
      :items="cityItems"
      :current="cityId"
      path="/once"
      :all-label="$t('common.all')"
      :extra-clear="{ three_city_id: undefined }"
    />
    <FilterRow
      v-if="cityId && districtItems.length"
      :label="$t('member_com_00378')"
      param="three_city_id"
      :items="districtItems"
      :current="threeCityId"
      path="/once"
      :all-label="$t('common.all')"
    />
    <p><NuxtLink to="/once/add">{{ $t('common.publish') }}</NuxtLink></p>
    <div v-if="(paylog?.list || []).length" class="stack">
      <h2>{{ $t('default_00031') }}</h2>
      <article v-for="row in paylog?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.order_id }} · {{ row.order_price }}</h3>
        <button type="button" @click="cancelPay(row.id)">{{ $t('common.cancel') }}</button>
      </article>
    </div>
    <NewsListShell :title="$t('wap_js_00130')" :error="error" :error-text="failMsg" :count="list.length">
      <SimpleCard v-for="row in list" :key="row.id" :to="`/once/${row.id}`" :title="row.title || row.companyname" :meta="row.mans || row.number" />
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
