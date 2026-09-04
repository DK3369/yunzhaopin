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
const { settings } = useSiteChrome()
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
    <p v-if="!hasPoint || locFail">
      <button type="button" @click="locate">{{ $t('common_05774') }}</button>
    </p>
    <p v-else-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
    <p v-else-if="!list.length" class="muted">{{ $t('wap_00606') }}</p>
    <div v-else>
      <JobCard v-for="job in list" :key="job.id" :job="job" variant="search" />
    </div>
  </section>
</template>
