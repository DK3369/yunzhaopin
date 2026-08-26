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
      </select>
      <input name="kw" :value="kw" :placeholder="$t('common.search')" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <p v-if="!kw" class="muted">{{ $t('home.search_placeholder') }}</p>
    <template v-else>
      <h2>{{ $t('common.job') }}</h2>
      <p v-if="!(data?.jobs || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
      <div class="stack">
        <JobCard v-for="job in data?.jobs || []" :key="job.id" :job="job" />
      </div>
      <h2>{{ $t('common.company') }}</h2>
      <p v-if="!(data?.companies || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
      <div class="stack">
        <CompanyCard v-for="c in data?.companies || []" :key="c.uid" :company="c" />
      </div>
    </template>
  </section>
</template>
