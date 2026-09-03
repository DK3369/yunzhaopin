<script setup lang="ts">
import { type JobLike } from '~/utils/site'

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

const route = useRoute()
const { t } = useI18n()
const x = computed(() => String(route.query.x || ''))
const y = computed(() => String(route.query.y || ''))
const hasPoint = computed(() => x.value !== '' && y.value !== '')
const locFail = ref(false)
const api = useApi()
const { data, error } = await useAsyncData(
  () => `map-${x.value}-${y.value}`,
  () =>
    hasPoint.value
      ? api.get<NearJob[]>('/v1/wap/map/jobs', { x: Number(x.value), y: Number(y.value), radius_km: 5, limit: 50 })
      : Promise.resolve([] as NearJob[]),
)
const list = computed<JobLike[]>(() => {
  const raw = (Array.isArray(data.value) ? data.value : []) as NearJob[]
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
function locate() {
  locFail.value = false
  if (!import.meta.client || !navigator.geolocation) {
    locFail.value = true
    return
  }
  navigator.geolocation.getCurrentPosition(
    (pos) => {
      navigateTo({ path: '/map', query: { x: String(pos.coords.longitude), y: String(pos.coords.latitude) } })
    },
    () => {
      locFail.value = true
    },
  )
}
onMounted(() => {
  if (!hasPoint.value) locate()
})
useSeoMeta({ title: t('wap_00223') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00223') }}</h1>
    <p v-if="!hasPoint || locFail">
      <button type="button" @click="locate">{{ $t('wap_00223') }}</button>
    </p>
    <p v-else-if="error" class="muted">{{ $t('home.no_job_data') }}</p>
    <p v-else-if="!list.length" class="muted">{{ $t('home.no_recruiting_jobs') }}</p>
    <div v-else>
      <JobCard v-for="job in list" :key="job.id" :job="job" variant="search" />
    </div>
  </section>
</template>
