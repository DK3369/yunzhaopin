<script setup lang="ts">
const route = useRoute()
const { t } = useI18n()
const x = computed(() => String(route.query.x || ''))
const y = computed(() => String(route.query.y || ''))
const hasPoint = computed(() => x.value !== '' && y.value !== '')
const api = useApi()
const { data, error } = await useAsyncData(
  () => `map-${x.value}-${y.value}`,
  () =>
    hasPoint.value
      ? api.get('/v1/wap/map/jobs', { x: Number(x.value), y: Number(y.value), radius_km: 5, limit: 50 })
      : Promise.resolve([]),
)
const list = computed(
  () =>
    (Array.isArray(data.value) ? data.value : []) as {
      id: number
      name: string
      com_name?: string
      distance_km?: number
    }[],
)
function locate() {
  if (!import.meta.client || !navigator.geolocation) return
  navigator.geolocation.getCurrentPosition((pos) => {
    navigateTo({ path: '/map', query: { x: String(pos.coords.longitude), y: String(pos.coords.latitude) } })
  })
}
onMounted(() => {
  if (!hasPoint.value) locate()
})
useSeoMeta({ title: t('ui.map') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.map') }}</h1>
    <p>
      <button type="button" @click="locate">{{ $t('common.search') }}</button>
    </p>
    <form method="get" action="/map">
      <input name="x" :value="x" placeholder="x" />
      <input name="y" :value="y" placeholder="y" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="!hasPoint" class="muted">{{ $t('common.search') }}</p>
    <p v-else-if="error" class="muted">{{ $t('home.no_job_data') }}</p>
    <p v-else-if="!list.length" class="muted">{{ $t('home.no_recruiting_jobs') }}</p>
    <div v-else class="stack">
      <SimpleCard
        v-for="row in list"
        :key="row.id"
        :to="`/jobs/${row.id}`"
        :title="row.name"
        :meta="`${row.com_name || ''} · ${row.distance_km ?? ''} km`"
      />
    </div>
  </section>
</template>
