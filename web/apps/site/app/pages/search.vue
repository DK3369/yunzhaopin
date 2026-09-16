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
const hotScope = computed(() => {
  if (scope.value === 'company' || scope.value === 'article' || scope.value === 'resume') return scope.value
  return 'job'
})
const { data: hots } = await useAsyncData(
  () => `search-hot-${hotScope.value}`,
  () =>
    api
      .get<Array<{ keyword: string }>>('/v1/wap/hot-searches', { scope: hotScope.value, limit: 12 })
      .catch(() => [] as Array<{ keyword: string }>),
)
const HIST_KEY = 'phpyun_search_history'
type HistItem = { kw: string; scope: string }
const history = ref<HistItem[]>([])
function loadHist() {
  try {
    const raw = localStorage.getItem(HIST_KEY)
    const parsed = raw ? JSON.parse(raw) : []
    if (!Array.isArray(parsed)) {
      history.value = []
      return
    }
    history.value = parsed
      .map((x: unknown) => {
        if (typeof x === 'string') return { kw: x, scope: 'all' }
        const row = x as HistItem
        return { kw: String(row?.kw || '').trim(), scope: String(row?.scope || 'all') }
      })
      .filter((x: HistItem) => x.kw)
      .slice(0, 12)
  } catch {
    history.value = []
  }
}
function pushHist(k: string, s: string) {
  const word = k.trim()
  if (!word || !import.meta.client) return
  const next = [{ kw: word, scope: s }, ...history.value.filter((x) => x.kw !== word || x.scope !== s)].slice(0, 12)
  history.value = next
  localStorage.setItem(HIST_KEY, JSON.stringify(next))
}
function clearHist() {
  history.value = []
  localStorage.removeItem(HIST_KEY)
}
onMounted(() => {
  loadHist()
  if (kw.value) pushHist(kw.value, scope.value)
})
watch(kw, (v) => {
  if (v) pushHist(v, scope.value)
})
useSeoMeta({ title: kw.value ? `${kw.value} - ${t('common.search')}` : t('common.search') })
</script>

<template>
  <section class="site-pc">
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
    <template v-else>
      <template v-if="scope === 'all' || scope === 'job'">
        <h2>{{ $t('common.job') }}</h2>
        <p v-if="!(data?.jobs || []).length" class="muted">{{ $t('default_00033') }}</p>
        <div class="stack">
          <JobCard v-for="job in data?.jobs || []" :key="job.id" :job="job" />
        </div>
      </template>
      <template v-if="scope === 'all' || scope === 'company'">
        <h2>{{ $t('common.company') }}</h2>
        <p v-if="!(data?.companies || []).length" class="muted">{{ $t('wap_00590') }}</p>
        <div class="stack">
          <CompanyCard v-for="c in data?.companies || []" :key="c.uid" :company="c" />
        </div>
      </template>
      <template v-if="scope === 'all' || scope === 'article'">
        <h2>{{ $t('common.article') }}</h2>
        <p v-if="!(data?.articles || []).length" class="muted">{{ $t('common_02409') }}</p>
        <div class="stack">
          <SimpleCard
            v-for="a in data?.articles || []"
            :key="a.id"
            :to="`/articles/${a.id}`"
            :title="a.title_all || a.title"
            :meta="a.datetime_n || a.category"
          />
        </div>
      </template>
      <template v-if="scope === 'all' || scope === 'resume'">
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
    </template>
  </section>
  <div class="site-h5">
    <div class="wap_search_header">
      <NuxtLink to="/" class="wap_search_headerqx" />
      <div class="wap_search_header_c">
        <form method="get" action="/search">
          <input type="hidden" name="scope" :value="scope" />
          <div class="wap_search_text">
            <input name="kw" class="input_search" :value="kw" :placeholder="$t('admin_00245')" />
          </div>
          <div class="wap_search_hbth">
            <input type="submit" class="searchbtn_input" :value="$t('common.search')" />
          </div>
        </form>
      </div>
    </div>
    <div class="search_history_tag_box search-h5-scopes">
      <NuxtLink
        class="search_history_tag"
        :class="{ on: scope === 'all' }"
        :to="{ query: { scope: 'all', kw: kw || undefined } }"
      >{{ $t('common.all') }}</NuxtLink>
      <NuxtLink
        class="search_history_tag"
        :class="{ on: scope === 'job' }"
        :to="{ query: { scope: 'job', kw: kw || undefined } }"
      >{{ $t('common.job') }}</NuxtLink>
      <NuxtLink
        class="search_history_tag"
        :class="{ on: scope === 'company' }"
        :to="{ query: { scope: 'company', kw: kw || undefined } }"
      >{{ $t('common.company') }}</NuxtLink>
      <NuxtLink
        class="search_history_tag"
        :class="{ on: scope === 'article' }"
        :to="{ query: { scope: 'article', kw: kw || undefined } }"
      >{{ $t('common.article') }}</NuxtLink>
      <NuxtLink
        class="search_history_tag"
        :class="{ on: scope === 'resume' }"
        :to="{ query: { scope: 'resume', kw: kw || undefined } }"
      >{{ $t('common.resume') }}</NuxtLink>
    </div>
    <template v-if="!kw">
      <div v-if="history.length" class="Search_jobs_body">
        <div class="search_history_tit">
          <span>{{ $t('wap_00384') }}</span>
          <span class="search_history_qc" @click="clearHist">{{ $t('wap_01250') }}</span>
        </div>
        <div class="search_history_tag_box">
          <NuxtLink
            v-for="(h, i) in history"
            :key="'hist-' + i + h.kw"
            class="search_history_tag"
            :to="`/search?scope=${encodeURIComponent(h.scope)}&kw=${encodeURIComponent(h.kw)}`"
          >{{ h.kw }}</NuxtLink>
        </div>
      </div>
      <div class="Search_jobs_body">
        <div class="search_history_tit">{{ $t('wap_00385') }}</div>
        <div class="search_history_tag_box">
          <p v-if="!(hots || []).length" class="search_history_no">{{ $t('common_02180') }}</p>
          <NuxtLink
            v-for="h in hots || []"
            :key="'hot-' + h.keyword"
            class="search_history_tag"
            :to="`/search?scope=${encodeURIComponent(scope)}&kw=${encodeURIComponent(h.keyword)}`"
          >{{ h.keyword }}</NuxtLink>
        </div>
      </div>
    </template>
    <template v-else>
      <template v-if="scope === 'all' || scope === 'job'">
        <div class="Search_jobs_body">
          <div class="search_history_tit">{{ $t('common.job') }}</div>
          <p v-if="!(data?.jobs || []).length" class="muted">{{ $t('default_00033') }}</p>
          <JobCard v-for="job in data?.jobs || []" :key="'h5j-' + job.id" :job="job" />
        </div>
      </template>
      <template v-if="scope === 'all' || scope === 'company'">
        <div class="Search_jobs_body">
          <div class="search_history_tit">{{ $t('common.company') }}</div>
          <p v-if="!(data?.companies || []).length" class="muted">{{ $t('wap_00590') }}</p>
          <CompanyCard v-for="c in data?.companies || []" :key="'h5c-' + c.uid" :company="c" />
        </div>
      </template>
      <template v-if="scope === 'all' || scope === 'article'">
        <div class="Search_jobs_body">
          <div class="search_history_tit">{{ $t('common.article') }}</div>
          <p v-if="!(data?.articles || []).length" class="muted">{{ $t('common_02409') }}</p>
          <SimpleCard
            v-for="a in data?.articles || []"
            :key="'h5a-' + a.id"
            :to="`/articles/${a.id}`"
            :title="a.title_all || a.title"
            :meta="a.datetime_n || a.category"
          />
        </div>
      </template>
      <template v-if="scope === 'all' || scope === 'resume'">
        <div class="Search_jobs_body">
          <div class="search_history_tit">{{ $t('common.resume') }}</div>
          <p v-if="!(resumeData?.list || []).length" class="muted">{{ $t('wap_com_00315') }}</p>
          <NuxtLink
            v-for="r in resumeData?.list || []"
            :key="'h5r-' + r.uid"
            class="search_history_tag"
            :to="`/resumes/${r.uid}`"
          >
            {{ r.display_name || r.name || r.uname || $t('common_02430') }}
          </NuxtLink>
          <p>
            <NuxtLink :to="`/resumes?keyword=${encodeURIComponent(kw)}`">{{ $t('common.view_more') }}</NuxtLink>
          </p>
        </div>
      </template>
    </template>
  </div>
</template>
