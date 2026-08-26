<script setup lang="ts">
import { listFailMsg, type CompanyLike } from '~/utils/site'
import type { DictItem } from '~/utils/query'

const route = useRoute()
const { t, locale } = useI18n()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const rec = computed(() => route.query.rec === '1')
const hy = computed(() => numQuery(route.query.hy))
const provinceId = computed(() => numQuery(route.query.province_id))
const api = useApi()
const { data, error } = await useAsyncData(
  () => `companies-${locale.value}-${page.value}-${keyword.value}-${rec.value}-${hy.value}-${provinceId.value}`,
  () =>
    api.get<{ list: CompanyLike[]; total: number }>('/v1/wap/companies', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
      rec: rec.value || undefined,
      hy: hy.value,
      province_id: provinceId.value,
    }),
)
const { data: industries } = await useAsyncData(
  () => `dict-hy-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/industries').catch(() => [] as DictItem[]),
)
const { data: provinces } = await useAsyncData(
  () => `regions-cn-1-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/regions', { country: 'CN', level: 1 }).catch(() => [] as DictItem[]),
)
useSeoMeta({ title: t('home.famous_companies') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
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
        </ul>
        <div class="firmsearch_h1_box_line yun_bg_color" />
      </div>
      <FilterRow
        :label="$t('common.company')"
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
      <p v-if="error" class="muted">{{ failMsg }}</p>
      <div v-else class="firm_list_content">
        <div class="firm_list_content_box">
          <CompanyCard v-for="c in list" :key="c.uid" :company="c" variant="firm" />
        </div>
        <p v-if="!list.length" class="muted">{{ $t('ui.no_data') }}</p>
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
    <form action="/companies" method="get" style="padding: 0.2rem 0.32rem">
      <input class="searchnew" name="keyword" :value="keyword" :placeholder="$t('common.search')" />
    </form>
    <H5FilterBar
      :all-label="$t('common.all')"
      :tabs="[
        { key: 'hy', label: $t('common.company'), items: industries || [] },
        { key: 'province_id', label: $t('common_02110'), items: provinces || [] },
      ]"
    />
    <div class="new_mq" style="margin: 0.2rem">
      <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
      <template v-else>
        <CompanyCard v-for="c in list" :key="c.uid" :company="c" variant="firm" />
        <p v-if="!list.length" class="muted" style="padding: 0.4rem">{{ $t('ui.no_data') }}</p>
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
