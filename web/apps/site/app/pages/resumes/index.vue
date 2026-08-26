<script setup lang="ts">
import type { DictItem } from '~/utils/query'

const route = useRoute()
const { t, locale } = useI18n()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const education = computed(() => numQuery(route.query.education))
const api = useApi()
const { data } = await useAsyncData(
  () => `resumes-${locale.value}-${page.value}-${keyword.value}-${education.value}`,
  () =>
    api.get('/v1/wap/resumes', {
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
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
    <div class="index_resume_user_list index_zw_item site-pc">
      <ul>
        <li v-for="r in data?.list || []" :key="r.uid">
          <div class="index_resume_user">
            <NuxtLink :to="`/resumes/${r.uid}`" class="index_resume_username">
              {{ r.display_name || r.name || r.uname }}
            </NuxtLink>
          </div>
          <div class="index_resume_userinfo">
            {{ r.exp_n }}<i v-if="r.exp_n && r.edu_n" class="index_resume_userinfo_line">|</i>{{ r.edu_n }}
          </div>
          <div class="index_resume_useryx">
            {{ $t('home.intention') }}<span class="index_resume_useryx_n">{{ r.job_classid_n || r.expect || '' }}</span>
          </div>
        </li>
      </ul>
    </div>
    <div class="site-h5 stack">
      <H5FilterBar :all-label="$t('common.all')" :tabs="[{ key: 'education', label: $t('wap_00238'), items: edus || [] }]" />
      <NuxtLink v-for="r in data?.list || []" :key="r.uid" :to="`/resumes/${r.uid}`" class="table-card">
        <div class="card_post">
          <i class="table-card-word">{{ r.display_name || r.name || r.uname }}</i>
        </div>
        <div class="table-card-require">
          <i class="requir-area">{{ r.exp_n }}</i>
          <i v-if="r.edu_n" class="requir_area_parting_line" />
          <i v-if="r.edu_n" class="requir-area">{{ r.edu_n }}</i>
        </div>
      </NuxtLink>
    </div>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </section>
</template>
