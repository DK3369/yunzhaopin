<script setup lang="ts">
import { listFailMsg } from '~/utils/site'
import type { DictItem } from '~/utils/query'

const route = useRoute()
const { t, locale } = useI18n()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const education = computed(() => numQuery(route.query.education))
const api = useApi()
const { data, error } = await useAsyncData(
  () => `resumes-${locale.value}-${page.value}-${keyword.value}-${education.value}`,
  () =>
    api.get<{ list: Array<Record<string, unknown>>; total: number }>('/v1/wap/resumes', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
      education: education.value,
    }),
)
const { data: edus } = await useAsyncData(
  () => `dict-edu-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/educations').catch(() => [] as DictItem[]),
)
useSeoMeta({ title: t('common.resume') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <section>
    <h1>{{ $t('common.resume') }}</h1>
    <form action="/resumes" method="get" class="jobsearch_newbox">
      <input class="Search_jobs_text" name="keyword" :value="keyword" :placeholder="$t('common.search')" />
      <input class="Search_jobs_submit" type="submit" :value="$t('common.search')" />
    </form>
    <FilterRow
      :label="$t('home.education_suffix')"
      param="education"
      :items="edus || []"
      :current="education"
      path="/resumes"
      :all-label="$t('common.all')"
    />
    <p v-if="error" class="muted">{{ failMsg }}</p>
    <template v-else>
      <p v-if="!list.length" class="muted">{{ $t('ui.no_data') }}</p>
      <div class="site-h5">
        <H5FilterBar :all-label="$t('common.all')" :tabs="[{ key: 'education', label: $t('wap_00238'), items: edus || [] }]" />
      </div>
      <ResumeCard v-for="r in list" :key="String(r.uid)" :row="r" />
    </template>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </section>
</template>
