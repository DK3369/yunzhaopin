<script setup lang="ts">
import { listFailMsg, type CompanyLike } from '~/utils/site'
import type { DictItem } from '~/utils/query'

const route = useRoute()
const { t, locale } = useI18n()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const rec = computed(() => route.query.rec === '1')
const cert = computed(() => route.query.cert === '1')
const hy = computed(() => numQuery(route.query.hy))
const provinceId = computed(() => numQuery(route.query.province_id))
const cityId = computed(() => numQuery(route.query.city_id))
const threeCityId = computed(() => numQuery(route.query.three_city_id))
const pr = computed(() => numQuery(route.query.pr))
const mun = computed(() => numQuery(route.query.mun))
const welfare = computed(() => numQuery(route.query.welfare))
const api = useApi()
const { applyToQuery } = useSubSite()
const { data, error } = await useAsyncData(
  () =>
    `companies-${locale.value}-${page.value}-${keyword.value}-${rec.value}-${cert.value}-${hy.value}-${provinceId.value}-${cityId.value}-${threeCityId.value}-${pr.value}-${mun.value}-${welfare.value}`,
  () =>
    api.get<{ list: CompanyLike[]; total: number }>(
      '/v1/wap/companies',
      applyToQuery({
        page: page.value,
        page_size: 20,
        keyword: keyword.value || undefined,
        rec: rec.value || undefined,
        cert: cert.value || undefined,
        hy: hy.value,
        province_id: provinceId.value,
        city_id: cityId.value,
        three_city_id: threeCityId.value,
        pr: pr.value,
        mun: mun.value,
        welfare: welfare.value,
        uptime: numQuery(route.query.uptime),
      }),
    ),
)
const { data: industries } = await useAsyncData(
  () => `dict-hy-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/industries').catch(() => [] as DictItem[]),
)
const { data: provinces } = await useAsyncData(
  () => `dict-city-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/cities').catch(() => [] as DictItem[]),
)
const { data: cities } = await useAsyncData(
  () => `dict-city-child-${locale.value}-${provinceId.value || 0}`,
  () =>
    provinceId.value
      ? api
          .get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: provinceId.value })
          .catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
const { data: districts } = await useAsyncData(
  () => `dict-city-dist-${locale.value}-${cityId.value || 0}`,
  () =>
    cityId.value
      ? api
          .get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: cityId.value })
          .catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
const { data: natures } = await useAsyncData(
  () => `dict-pr-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/company-natures').catch(() => [] as DictItem[]),
)
const { data: sizes } = await useAsyncData(
  () => `dict-mun-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/company-sizes').catch(() => [] as DictItem[]),
)
const { data: welfares } = await useAsyncData(
  () => `dict-welfare-com-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/welfares').catch(() => [] as DictItem[]),
)
useSeoMeta({ title: t('home.famous_companies') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
useListLoginGate(error)
const list = computed(() => data.value?.list || [])
</script>

<template>
  <div class="site-pc">
    <div class="yun_content" style="padding: 16px 0 40px">
      <form action="/companies" method="get" class="jobsearch_newbox" style="margin-bottom: 12px">
        <input class="Search_jobs_text" name="keyword" :value="keyword" :placeholder="$t('common.search')" />
        <input class="Search_jobs_submit" type="submit" :value="$t('common.search')" />
      </form>
      <div class="firmsearch_h1_box_title">
        <ul class="firmsearch_h1_box_list">
          <li :class="{ firmsearch_h1_box_cur: !rec }">
            <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { rec: undefined }) }">{{
              $t('common.all')
            }}</NuxtLink>
            <i class="firmsearch_h1_box_list_icon" />
          </li>
          <li :class="{ firmsearch_h1_box_cur: rec }">
            <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { rec: '1' }) }">{{
              $t('home.famous_companies')
            }}</NuxtLink>
            <i class="firmsearch_h1_box_list_icon firmsearch_h1_box_list_icon_jj png" />
          </li>
          <li :class="{ firmsearch_h1_box_cur: cert }">
            <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { cert: '1' }) }">{{
              $t('wap_00288')
            }}</NuxtLink>
            <i class="firmsearch_h1_box_list_icon" />
          </li>
        </ul>
        <div class="firmsearch_h1_box_line yun_bg_color" />
      </div>
      <FilterRow
        :label="$t('admin_user_company_00373')"
        param="hy"
        :items="industries || []"
        :current="hy"
        path="/companies"
        :all-label="$t('common.all')"
      />
      <FilterRow
        :label="$t('member_com_00378')"
        param="province_id"
        :items="provinces || []"
        :current="provinceId"
        path="/companies"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="provinceId && (cities || []).length"
        :label="$t('common_02110')"
        param="city_id"
        :items="cities || []"
        :current="cityId"
        path="/companies"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="cityId && (districts || []).length"
        :label="$t('member_com_00378')"
        param="three_city_id"
        :items="districts || []"
        :current="threeCityId"
        path="/companies"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="(natures || []).length"
        :label="$t('wap_com_00018')"
        param="pr"
        :items="natures || []"
        :current="pr"
        path="/companies"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="(sizes || []).length"
        :label="$t('wap_com_00019')"
        param="mun"
        :items="sizes || []"
        :current="mun"
        path="/companies"
        :all-label="$t('common.all')"
      />
      <FilterRow
        v-if="(welfares || []).length"
        :label="$t('company_00007')"
        param="welfare"
        :items="welfares || []"
        :current="welfare"
        path="/companies"
        :all-label="$t('common.all')"
      />
      <p v-if="error" class="muted">{{ failMsg }}</p>
      <div v-else class="firm_list_content">
        <div class="firm_list_content_box">
          <CompanyCard v-for="c in list" :key="c.uid" :company="c" variant="firm" />
        </div>
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
  <div class="site-h5">
    <div class="job_header_nav resumeAdeFlex">
      <div class="job_header_nav_left category" style="width: 30%">
        <ul>
          <li :class="{ active: !rec }">
            <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { rec: undefined }) }">{{
              $t('common.all')
            }}</NuxtLink>
          </li>
          <li :class="{ active: rec }">
            <NuxtLink :to="{ path: '/companies', query: mergeQuery(route.query, { rec: '1' }) }">{{
              $t('home.famous_companies')
            }}</NuxtLink>
          </li>
        </ul>
      </div>
      <H5FilterBar
        :all-label="$t('common.all')"
        :tabs="[
          {
            key: 'province_id',
            label: $t('common_02110'),
            items: provinces || [],
            childKey: 'city_id',
            childItems: cities || [],
            grandKey: 'three_city_id',
            grandItems: districts || [],
          },
          { key: 'hy', label: $t('admin_user_company_00373'), items: industries || [] },
          { key: 'pr', label: $t('wap_com_00018'), items: natures || [] },
          { key: 'mun', label: $t('wap_com_00019'), items: sizes || [] },
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
