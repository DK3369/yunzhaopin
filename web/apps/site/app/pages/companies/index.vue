<script setup lang="ts">
import { listFailMsg, mediaUrl, PLACEHOLDER_LOGO, type CompanyLike } from '~/utils/site'

const route = useRoute()
const { t, locale } = useI18n()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const rec = computed(() => route.query.rec === '1')
const hy = computed(() => numQuery(route.query.hy))
const country = computed(() => countryQuery(route.query.country))
const provinceId = computed(() => numQuery(route.query.province_id))
const cityId = computed(() => numQuery(route.query.city_id))
const threeCityId = computed(() => numQuery(route.query.three_city_id))
const pr = computed(() => numQuery(route.query.pr))
const mun = computed(() => numQuery(route.query.mun))
const welfare = computed(() => numQuery(route.query.welfare))
const api = useApi()
const { applyToQuery } = useSubSite()
const { countryItems, countryDictItems, provinceItems, cityItems, districtItems, dicts } = await useRegionCascade({
  country,
  provinceId: computed(() => provinceId.value || 0),
  cityId: computed(() => cityId.value || 0),
})
const listQuery = computed(() =>
  applyToQuery({
    page: page.value,
    page_size: 20,
    keyword: keyword.value || undefined,
    rec: rec.value || undefined,
    hy: hy.value,
    country: country.value || undefined,
    province_id: provinceId.value,
    city_id: cityId.value,
    three_city_id: threeCityId.value,
    pr: pr.value,
    mun: mun.value,
    welfare: welfare.value,
    uptime: numQuery(route.query.uptime),
  }),
)
const { data, error } = await useAsyncData(
  () =>
    `companies-${locale.value}-${page.value}-${keyword.value}-${rec.value}-${hy.value}-${country.value}-${provinceId.value}-${cityId.value}-${threeCityId.value}-${pr.value}-${mun.value}-${welfare.value}`,
  () => api.get<{ list: CompanyLike[]; total: number }>('/v1/wap/companies', listQuery.value),
)
const { data: recSide } = await useAsyncData(
  () => `companies-sidebar-${locale.value}`,
  () =>
    api
      .get<{ rec?: CompanyLike[] }>('/v1/wap/companies/sidebar', applyToQuery({}))
      .catch(() => ({ rec: [] as CompanyLike[] })),
)
const industries = computed(() => dicts.value?.industries ?? [])
const natures = computed(() => dicts.value?.company_natures ?? [])
const sizes = computed(() => dicts.value?.company_sizes ?? [])
const welfares = computed(() => dicts.value?.welfares ?? [])
useSeoMeta({ title: t('default_00113') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
useListLoginGate(error)
const list = computed(() => data.value?.list || [])
const recCompanies = computed(() => recSide.value?.rec || [])
</script>

<template>
  <div class="site-pc">
    <div class="yun_jobbody">
      <div class="yun_content">
        <div class="firm_search_box">
          <div class="firm_search_boxbor">
            <form action="/companies" method="get">
              <input class="firm_search_text" name="keyword" :value="keyword" :placeholder="$t('admin_system_00198')" />
              <input class="firm_search_submit yun_bg_color" type="submit" :value="$t('common.search')" />
            </form>
          </div>
        </div>
        <div class="clear" />
        <div class="fiem_seach_chlose">
          <FilterRow
            skin="firm"
            :label="$t('admin_user_company_00373')"
            param="hy"
            :items="industries || []"
            :current="hy"
            path="/companies"
            :all-label="$t('common.all')"
          />
          <CountryFilterRow
            skin="firm"
            :label="$t('common.country')"
            :items="countryItems"
            :current="country"
            path="/companies"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="country"
            skin="firm"
            :label="$t('member_com_00378')"
            param="province_id"
            :items="provinceItems"
            :current="provinceId"
            path="/companies"
            :all-label="$t('common.all')"
            :extra-clear="{ city_id: undefined, three_city_id: undefined }"
          />
          <FilterRow
            v-if="provinceId && cityItems.length"
            skin="firm"
            :label="$t('common_02110')"
            param="city_id"
            :items="cityItems"
            :current="cityId"
            path="/companies"
            :all-label="$t('common.all')"
            :extra-clear="{ three_city_id: undefined }"
          />
          <FilterRow
            v-if="cityId && districtItems.length"
            skin="firm"
            :label="$t('member_com_00378')"
            param="three_city_id"
            :items="districtItems"
            :current="threeCityId"
            path="/companies"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="(natures || []).length"
            skin="firm"
            :label="$t('wap_com_00159')"
            param="pr"
            :items="natures || []"
            :current="pr"
            path="/companies"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="(sizes || []).length"
            skin="firm"
            :label="$t('wap_com_00163')"
            param="mun"
            :items="sizes || []"
            :current="mun"
            path="/companies"
            :all-label="$t('common.all')"
          />
          <FilterRow
            v-if="(welfares || []).length"
            skin="firm"
            :label="$t('wap_com_00167')"
            param="welfare"
            :items="welfares || []"
            :current="welfare"
            path="/companies"
            :all-label="$t('common.all')"
          />
        </div>
        <div class="firmsearch_h1_box_title">
          <ul class="firmsearch_h1_box_list">
            <li :class="{ firmsearch_h1_box_cur: !rec }">
              <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { rec: undefined, cert: undefined }) }">{{
                $t('default_00113')
              }}</NuxtLink>
              <i class="firmsearch_h1_box_list_icon" />
            </li>
            <li :class="{ firmsearch_h1_box_cur: rec }">
              <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { rec: '1', cert: undefined }) }">{{
                $t('home.famous_companies')
              }}</NuxtLink>
              <i class="firmsearch_h1_box_list_icon firmsearch_h1_box_list_icon_jj png" />
            </li>
          </ul>
          <div class="firmsearch_h1_box_line yun_bg_color" />
        </div>
        <div class="firm_right">
          <div class="firm_right_box">
            <div class="firm_list_content">
              <p v-if="error" class="muted">{{ failMsg }}</p>
              <div v-else class="firm_list_content_box">
                <CompanyCard v-for="c in list" :key="c.uid" :company="c" variant="firm" />
                <EmptyState v-if="!list.length" />
              </div>
              <Pager
                :page="page"
                :page-size="20"
                :total="data?.total || 0"
                @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
              />
            </div>
          </div>
        </div>
        <div v-if="recCompanies.length" class="firm_rightm">
          <div class="firm_rightm_box">
            <div class="firm_rightm_box_tit">
              <i class="firm_rightm_box_tit_icon yun_bg_color" />
              <span class="firm_rightm_box_tit_s">{{ $t('common_02401') }}</span>
            </div>
            <ul class="firm_rightm_tjlist">
              <li v-for="c in recCompanies" :key="'rec-' + c.uid">
                <NuxtLink :to="`/companies/${c.uid}`" :title="c.name">
                  <img :src="mediaUrl(c.logo_n || c.logo, PLACEHOLDER_LOGO)" alt="" width="100" height="100" />
                  <div class="firm_rightm_tjlistname">{{ c.name }}</div>
                </NuxtLink>
              </li>
            </ul>
          </div>
        </div>
        <div class="clear" />
      </div>
    </div>
  </div>
  <div class="site-h5">
    <div class="job_header_nav resumeAdeFlex">
      <div class="job_header_nav_left category" style="width: 30%">
        <ul>
          <li :class="{ active: !rec }">
            <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { rec: undefined, cert: undefined }) }">{{
              $t('common.all')
            }}</NuxtLink>
          </li>
          <li :class="{ active: rec }">
            <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { rec: '1', cert: undefined }) }">{{
              $t('wap_00299')
            }}</NuxtLink>
          </li>
        </ul>
      </div>
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
          { key: 'hy', label: $t('admin_user_company_00373'), items: industries || [] },
          {
            key: 'more',
            label: $t('wap_00238'),
            kind: 'more',
            items: [],
            groups: [
              { label: $t('wap_com_00159'), param: 'pr', items: natures || [] },
              { label: $t('wap_com_00163'), param: 'mun', items: sizes || [] },
              { label: $t('wap_com_00167'), param: 'welfare', items: welfares || [] },
            ],
          },
        ]"
      />
    </div>
    <form action="/companies" method="get" style="padding: 0.2rem 0.32rem">
      <input class="searchnew" name="keyword" :value="keyword" :placeholder="$t('common.search')" />
    </form>
    <div class="com_list_pd">
      <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
      <template v-else>
        <CompanyCard v-for="c in list" :key="c.uid" :company="c" variant="firm" />
        <EmptyState v-if="!list.length" />
      </template>
    </div>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </div>
</template>
