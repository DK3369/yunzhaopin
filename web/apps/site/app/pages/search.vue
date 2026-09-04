<script setup lang="ts">
const HISTORY = { job: 'job_key_history', resume: 'resume_key_history' }

function readHistory(kind: 'job' | 'resume'): string[] {
  if (!import.meta.client) return []
  const raw = document.cookie
    .split(';')
    .map((x) => x.trim())
    .find((x) => x.startsWith(`${HISTORY[kind]}=`))
  if (!raw) return []
  return decodeURIComponent(raw.split('=').slice(1).join('='))
    .split(',')
    .map((s) => s.trim())
    .filter(Boolean)
    .slice(0, 10)
}

function pushHistory(kind: 'job' | 'resume', kw: string) {
  if (!import.meta.client || !kw) return
  const next = [kw, ...readHistory(kind).filter((x) => x !== kw)].slice(0, 10)
  document.cookie = `${HISTORY[kind]}=${encodeURIComponent(next.join(','))}; path=/; max-age=${60 * 60 * 24 * 30}; SameSite=Lax`
}

const route = useRoute()
const { t } = useI18n()
const kw = computed(() => String(route.query.kw || ''))
const scope = computed(() => String(route.query.scope || 'all'))
const api = useApi()
const jobHistory = ref<string[]>([])
const resumeHistory = ref<string[]>([])
onMounted(() => {
  jobHistory.value = readHistory('job')
  resumeHistory.value = readHistory('resume')
  if (kw.value) {
    if (scope.value === 'resume') pushHistory('resume', kw.value)
    else pushHistory('job', kw.value)
    jobHistory.value = readHistory('job')
    resumeHistory.value = readHistory('resume')
  }
})
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
    <p v-if="!kw" class="muted">{{ $t('default_00348') }}</p>
    <div v-if="!kw && (jobHistory.length || resumeHistory.length)">
      <h2>{{ $t('ui.search_history') }}</h2>
      <p>
        <NuxtLink v-for="h in jobHistory" :key="'j'+h" :to="`/search?scope=job&kw=${encodeURIComponent(h)}`">{{ h }}</NuxtLink>
      </p>
      <p>
        <NuxtLink v-for="h in resumeHistory" :key="'r'+h" :to="`/search?scope=resume&kw=${encodeURIComponent(h)}`">{{ h }}</NuxtLink>
      </p>
    </div>
    <template v-else>
      <h2>{{ $t('common.job') }}</h2>
      <p v-if="!(data?.jobs || []).length" class="muted">{{ $t('default_00033') }}</p>
      <div class="stack">
        <JobCard v-for="job in data?.jobs || []" :key="job.id" :job="job" />
      </div>
      <h2>{{ $t('common.company') }}</h2>
      <p v-if="!(data?.companies || []).length" class="muted">{{ $t('wap_00590') }}</p>
      <div class="stack">
        <CompanyCard v-for="c in data?.companies || []" :key="c.uid" :company="c" />
      </div>
      <h2>{{ $t('common.resume') }}</h2>
      <p v-if="!(resumeData?.list || []).length" class="muted">{{ $t('wap_com_00315') }}</p>
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
