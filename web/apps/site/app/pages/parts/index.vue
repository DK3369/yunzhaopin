<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const country = computed(() => countryQuery(route.query.country))
const provinceId = computed(() => Number(route.query.province_id || 0) || undefined)
const cityId = computed(() => Number(route.query.city_id || 0) || undefined)
const threeCityId = computed(() => Number(route.query.three_city_id || 0) || undefined)
const partType = computed(() => Number(route.query.part_type || 0) || undefined)
const billingCycle = computed(() => Number(route.query.billing_cycle || 0) || undefined)
const { t, locale } = useI18n()
const api = useApi()
const { applyToQuery } = useSubSite()
const { countryItems, countryDictItems, provinceItems, cityItems, districtItems } = await useRegionCascade({
  country,
  provinceId: computed(() => provinceId.value || 0),
  cityId: computed(() => cityId.value || 0),
})
const { data, error } = await useAsyncData(
  () =>
    `parts-${locale.value}-${page.value}-${keyword.value}-${country.value}-${provinceId.value || 0}-${cityId.value || 0}-${threeCityId.value || 0}-${partType.value || 0}-${billingCycle.value || 0}`,
  () =>
    api.get<{
      list: Array<{
        id: number
        name: string
        com_name?: string
        city_name?: string
        salary?: number
        salary_type_n?: string
        billing_cycle_n?: string
        rec?: number
        is_rec?: boolean
      }>
      total: number
    }>(
      '/v1/wap/parts',
      applyToQuery({
        page: page.value,
        page_size: 20,
        keyword: keyword.value || undefined,
        country: country.value || undefined,
        province_id: provinceId.value,
        city_id: cityId.value,
        three_city_id: threeCityId.value,
        part_type: partType.value,
        billing_cycle: billingCycle.value,
      }),
    ),
)
const { data: partTypes } = await usePartCats()
const partRoots = computed(() =>
  (partTypes.value || []).filter((c) => !c.parent_id).sort((a, b) => a.id - b.id),
)
function partChildren(rootIdx: number) {
  const root = partRoots.value[rootIdx]
  if (!root) return []
  return (partTypes.value || []).filter((c) => Number(c.parent_id) === root.id)
}
const typeItems = computed(() => partChildren(0))
const cycleItems = computed(() => partChildren(2))
useSeoMeta({ title: t('wap_com_00311') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
useListLoginGate(error)
const list = computed(() => data.value?.list || [])
</script>

<template>
  <div class="site-pc">
    <NewsListShell :title="$t('wap_com_00311')" :error="error" :error-text="failMsg" :count="list.length">
      <form class="form" method="get" action="/parts">
        <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
        <button type="submit">{{ $t('common.search') }}</button>
      </form>
      <CountryFilterRow
        :label="$t('common.country')"
        :items="countryItems"
        :current="country"
        path="/parts"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="country"
        :label="$t('member_com_00378')"
        param="province_id"
        :items="provinceItems"
        :current="provinceId"
        path="/parts"
        :all-label="$t('common.all')"
        :extra-clear="{ city_id: undefined, three_city_id: undefined }"
      />
      <FilterRow
        v-if="provinceId && cityItems.length"
        :label="$t('common_02110')"
        param="city_id"
        :items="cityItems"
        :current="cityId"
        path="/parts"
        :all-label="$t('common.all')"
        :extra-clear="{ three_city_id: undefined }"
      />
      <FilterRow
        v-if="cityId && districtItems.length"
        :label="$t('member_com_00378')"
        param="three_city_id"
        :items="districtItems"
        :current="threeCityId"
        path="/parts"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="typeItems.length"
        :label="$t('wap_com_00311')"
        param="part_type"
        :items="typeItems"
        :current="partType"
        path="/parts"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="cycleItems.length"
        :label="$t('wap_user_00220')"
        param="billing_cycle"
        :items="cycleItems"
        :current="billingCycle"
        path="/parts"
        :all-label="$t('common.all')"
      />
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
  </div>
  <div class="site-h5">
    <form class="job_header_center" action="/parts" method="get" style="padding: 0.16rem 0.32rem">
      <input class="searchnew" name="keyword" :value="keyword" :placeholder="$t('common.search')" />
    </form>
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
          { key: 'part_type', label: $t('member_com_00307'), items: typeItems },
          { key: 'billing_cycle', label: $t('member_user_00199'), items: cycleItems },
        ]"
      />
    </div>
    <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
    <ul v-else class="part_box">
      <li v-for="row in list" :key="'h5-' + row.id" class="part_box_list">
        <NuxtLink :to="`/parts/${row.id}`">
          <div class="part_box_list_pd">
            <div class="part_box_jobname">
              <span v-if="row.is_rec" class="part_hot">{{ $t('wap_00582') }}</span>
              {{ row.name }}
              <span v-if="row.salary || row.salary_type_n" class="part_box_jobxz">
                {{ row.salary || '' }}
                <i class="part_box_jobxz_fh">{{ row.salary_type_n }}</i>
              </span>
            </div>
            <div class="part_box_jobxz_box">
              <div v-if="row.billing_cycle_n" class="Part_jsfs">{{ row.billing_cycle_n }}</div>
            </div>
            <div class="part_box_jobp">
              <span class="part_box_jobcity">{{ [row.city_name, row.com_name].filter(Boolean).join('-') }}</span>
            </div>
          </div>
        </NuxtLink>
      </li>
    </ul>
    <EmptyState v-if="!error && !list.length" />
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </div>
</template>
