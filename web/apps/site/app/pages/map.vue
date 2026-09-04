<script setup lang="ts">
import { type JobLike, type CompanyLike } from '~/utils/site'

type NearJob = {
  id: number
  uid?: number
  name: string
  com_name?: string
  min_salary?: number
  max_salary?: number
  city_name?: string
  distance_km?: number
}
type NearCompany = {
  uid: number
  name?: string
  city_name?: string
  logo_n?: string
  distance_km?: number
}

const route = useRoute()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { applyToQuery } = useSubSite()
const x = computed(() => String(route.query.x || ''))
const y = computed(() => String(route.query.y || ''))
const tab = computed(() => String(route.query.tab || 'jobs'))
const page = computed(() => Number(route.query.page || 1))
const hasPoint = computed(() => x.value !== '' && y.value !== '')
const locFail = ref(false)
const xInput = ref(String(route.query.x || ''))
const yInput = ref(String(route.query.y || ''))
const api = useApi()
const { data, error } = await useAsyncData(
  () => `map-${tab.value}-${x.value}-${y.value}-${page.value}`,
  () => {
    if (!hasPoint.value) return Promise.resolve({ list: [] as NearJob[], total: 0 })
    const q = applyToQuery({
      x: Number(x.value),
      y: Number(y.value),
      radius_km: tab.value === 'companies' ? 20 : 200,
      limit: 10,
      page: page.value,
    })
    if (tab.value === 'companies') {
      return api.get<{ list: NearCompany[]; total: number }>('/v1/wap/map/companies', q)
    }
    return api.get<{ list: NearJob[]; total: number }>('/v1/wap/map/jobs', q)
  },
)
const list = computed<JobLike[]>(() => {
  if (tab.value === 'companies') return []
  const raw = (Array.isArray(data.value?.list) ? data.value?.list : []) as NearJob[]
  return raw.map((row) => ({
    id: row.id,
    uid: row.uid,
    name: row.name,
    com_name: row.com_name,
    min_salary: row.min_salary,
    max_salary: row.max_salary,
    job_city_two: row.city_name,
    distance_km: row.distance_km,
  }))
})
const companies = computed<CompanyLike[]>(() => {
  if (tab.value !== 'companies') return []
  const raw = (Array.isArray(data.value?.list) ? data.value?.list : []) as NearCompany[]
  return raw.map((row) => ({
    uid: row.uid,
    name: row.name,
    city_two: row.city_name,
    logo_n: row.logo_n,
  }))
})
function locate() {
  locFail.value = false
  if (!import.meta.client || !navigator.geolocation) {
    locFail.value = true
    return
  }
  navigator.geolocation.getCurrentPosition(
    (pos) => {
      navigateTo({
        path: '/map',
        query: { ...route.query, x: String(pos.coords.longitude), y: String(pos.coords.latitude), page: 1 },
      })
    },
    () => {
      locFail.value = true
    },
  )
}
onMounted(() => {
  if (hasPoint.value) return
  const mx = String(settings.value.map_x || '').trim()
  const my = String(settings.value.map_y || '').trim()
  if (mx && my) {
    navigateTo({ path: '/map', query: { x: mx, y: my } })
    return
  }
  locate()
})
useSeoMeta({ title: t('default_00139') })
</script>

<template>
  <section>
    <h1>{{ $t('default_00139') }}</h1>
    <p>
      <NuxtLink :to="{ query: { ...route.query, tab: 'jobs', page: 1 } }">{{ $t('default_00246') }}</NuxtLink>
      <NuxtLink :to="{ query: { ...route.query, tab: 'companies', page: 1 } }">{{ $t('default_00114') }}</NuxtLink>
    </p>
    <p v-if="!hasPoint || locFail">
      <button type="button" @click="locate">{{ $t('common_05774') }}</button>
    </p>
    <form class="form" @submit.prevent="navigateTo({ path: '/map', query: { ...route.query, x: xInput, y: yInput, page: 1 } })">
      <input v-model="xInput" placeholder="x" />
      <input v-model="yInput" placeholder="y" />
      <button type="submit">{{ $t('common.search') }}</button>
    </form>
    <template v-if="hasPoint && !locFail">
      <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
      <template v-else-if="tab === 'companies'">
        <p v-if="!companies.length" class="muted">{{ $t('wap_00590') }}</p>
        <CompanyCard v-for="c in companies" :key="c.uid" :company="c" />
      </template>
      <template v-else>
        <p v-if="!list.length" class="muted">{{ $t('wap_00606') }}</p>
        <div v-else>
          <JobCard v-for="job in list" :key="job.id" :job="job" variant="search" />
        </div>
      </template>
    </template>
    <Pager
      :page="page"
      :page-size="10"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
    />
  </section>
</template>
