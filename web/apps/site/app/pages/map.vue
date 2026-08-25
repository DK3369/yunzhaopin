<script setup lang="ts">
const route = useRoute()
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
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as { id: number; name: string; com_name?: string; distance_km?: number }[])
useSeoMeta({ title: '地图找工作' })
</script>

<template>
  <section>
    <h1>地图找工作</h1>
    <form method="get" action="/map">
      <input name="x" :value="x" placeholder="经度 x" />
      <input name="y" :value="y" placeholder="纬度 y" />
      <button type="submit">附近职位</button>
    </form>
    <p v-if="!hasPoint" class="muted">请填写经纬度后查询附近职位。</p>
    <p v-else-if="error" class="muted">暂时无法加载附近职位。</p>
    <p v-else-if="!list.length" class="muted">附近暂无职位</p>
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
