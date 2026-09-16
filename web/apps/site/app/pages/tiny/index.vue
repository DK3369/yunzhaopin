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
const { countryItems, countryDictItems, provinceItems, cityItems, districtItems } = await useRegionCascade({
  country,
  provinceId: computed(() => provinceId.value || 0),
  cityId: computed(() => cityId.value || 0),
})
const { data, error } = await useAsyncData(
  () => `tiny-${page.value}-${keyword.value}-${country.value}-${provinceId.value || 0}-${cityId.value || 0}-${threeCityId.value || 0}`,
  () =>
    api.get<{
      list: Array<{ id: number; username: string; job?: string; sex?: number; exp?: number; lastupdate?: number }>
      total: number
    }>('/v1/wap/tiny-resumes/list', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
      country: country.value || undefined,
      province_id: provinceId.value,
      city_id: cityId.value,
      three_city_id: threeCityId.value,
    }),
)
useSeoMeta({ title: t('wap_js_00066') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <div class="site-pc">
    <NewsListShell :title="$t('wap_js_00066')" :error="error" :error-text="failMsg" :count="list.length">
      <form class="form" method="get" action="/tiny">
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
        path="/tiny"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="country"
        :label="$t('member_com_00378')"
        param="province_id"
        :items="provinceItems"
        :current="provinceId"
        path="/tiny"
        :all-label="$t('common.all')"
        :extra-clear="{ city_id: undefined, three_city_id: undefined }"
      />
      <FilterRow
        v-if="provinceId && cityItems.length"
        :label="$t('common_02110')"
        param="city_id"
        :items="cityItems"
        :current="cityId"
        path="/tiny"
        :all-label="$t('common.all')"
        :extra-clear="{ three_city_id: undefined }"
      />
      <FilterRow
        v-if="cityId && districtItems.length"
        :label="$t('member_com_00378')"
        param="three_city_id"
        :items="districtItems"
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
  </div>
  <div class="site-h5">
    <div class="job_header_nav resumeAdeFlex">
      <H5FilterBar
        :all-label="$t('common.all')"
        :tabs="[
          {
            key: 'country',
            label: $t('common.country'),
            items: countryDictItems,
            extraClear: { three_city_id: undefined },
            childKey: 'province_id',
            childItems: provinceItems,
            grandKey: 'city_id',
            grandItems: cityItems,
          },
        ]"
      />
    </div>
    <form action="/tiny" method="get" style="padding: 0.2rem 0.32rem">
      <input class="searchnew" name="keyword" :value="keyword" :placeholder="$t('common.search')" />
    </form>
    <p style="padding: 0 0.32rem"><NuxtLink to="/tiny/add">{{ $t('common.publish') }}</NuxtLink></p>
    <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
    <div v-else-if="!list.length" class="wap_member_no">{{ $t('wap_00524') }}</div>
    <template v-else>
      <NuxtLink v-for="row in list" :key="'h5-' + row.id" :to="`/tiny/${row.id}`" class="tiny_list">
        <div class="tiny_list_content">
          <h3>{{ row.job || row.username }}</h3>
          <div class="tiny_list_content_p">
            <span class="tiny_list_content_username">{{ row.username }}</span>
          </div>
        </div>
      </NuxtLink>
    </template>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </div>
</template>
