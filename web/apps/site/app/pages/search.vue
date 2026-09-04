<script setup lang="ts">
const route = useRoute()
const { t } = useI18n()
const kw = computed(() => String(route.query.kw || ''))
const scope = computed(() => String(route.query.scope || 'all'))
const api = useApi()
const { data } = await useAsyncData(
  () => `search-${scope.value}-${kw.value}`,
  () =>
    kw.value
      ? api.get('/v1/wap/search', { kw: kw.value, scope: scope.value })
      : Promise.resolve(null),
)
const { data: resumeData } = await useAsyncData(
  () => `search-resume-${kw.value}`,
  () =>
    kw.value
      ? api.get<{ list: Array<{ uid: number; name?: string; uname?: string; display_name?: string }> }>(
          '/v1/wap/resumes',
          { keyword: kw.value, page_size: 8 },
        )
      : Promise.resolve({ list: [] }),
)
useSeoMeta({ title: kw.value ? `${kw.value} - ${t('common.search')}` : t('common.search') })
</script>

<template>
  <section>
    <h1>{{ $t('common.search') }}</h1>
    <form class="form" method="get" action="/search">
      <select name="scope" :value="scope">
        <option value="all">{{ $t('common.all') }}</option>
        <option value="job">{{ $t('common.job') }}</option>
        <option value="company">{{ $t('common.company') }}</option>
        <option value="article">{{ $t('common.article') }}</option>
        <option value="resume">{{ $t('common.resume') }}</option>
      </select>
      <input name="kw" :value="kw" :placeholder="$t('common.search')" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <p v-if="!kw" class="muted">{{ $t('wap_00376') }}</p>
    <template v-else>
      <h2>{{ $t('common.job') }}</h2>
      <p v-if="!(data?.jobs || []).length" class="muted">{{ $t('common_02402') }}</p>
      <div class="stack">
        <JobCard v-for="job in data?.jobs || []" :key="job.id" :job="job" />
      </div>
      <h2>{{ $t('common.company') }}</h2>
      <p v-if="!(data?.companies || []).length" class="muted">{{ $t('common_02402') }}</p>
      <div class="stack">
        <CompanyCard v-for="c in data?.companies || []" :key="c.uid" :company="c" />
      </div>
      <h2>{{ $t('common.resume') }}</h2>
      <p v-if="!(resumeData?.list || []).length" class="muted">{{ $t('common_02402') }}</p>
      <div class="stack">
        <NuxtLink
          v-for="r in resumeData?.list || []"
          :key="r.uid"
          :to="`/resumes/${r.uid}`"
        >
          {{ r.display_name || r.name || r.uname || $t('common_02430') }}
        </NuxtLink>
      </div>
      <p>
        <NuxtLink :to="`/resumes?keyword=${encodeURIComponent(kw)}`">{{ $t('common.view_more') }}</NuxtLink>
      </p>
    </template>
  </section>
</template>
