<script setup lang="ts">
const x = defineModel<string>('x', { default: '' })
const y = defineModel<string>('y', { default: '' })
const mapEl = ref<HTMLDivElement | null>(null)
const { settings } = useSiteChrome()
const ready = ref(false)

onMounted(async () => {
  const key = String(settings.value.map_key || '').trim()
  const secret = String(settings.value.map_secret || settings.value.map_security || '').trim()
  const ok = await loadAmap(key, secret)
  ready.value = ok
  await nextTick()
  if (!ok || !mapEl.value) return
  const AMap = (window as unknown as { AMap: { Map: new (...a: unknown[]) => { on: Function; setCenter: Function }; Marker: new (...a: unknown[]) => { setPosition: Function } } }).AMap
  const center = mapCenter(settings.value, x.value, y.value)
  const map = new AMap.Map(mapEl.value, { zoom: 13, center })
  let marker: { setPosition: (pos: [number, number]) => void } | null = null
  const put = (lng: number, lat: number) => {
    x.value = String(lng)
    y.value = String(lat)
    const pos: [number, number] = [lng, lat]
    if (marker) marker.setPosition(pos)
    else marker = new AMap.Marker({ position: pos, map })
  }
  if (x.value && y.value) put(Number(x.value), Number(y.value))
  map.on('click', (e: { lnglat: { getLng: () => number; getLat: () => number } }) => {
    put(e.lnglat.getLng(), e.lnglat.getLat())
  })
})
</script>

<template>
  <div>
    <div ref="mapEl" class="map-pick" />
    <p v-if="!ready" class="muted">{{ $t('ui.lng') }} / {{ $t('ui.lat') }}</p>
  </div>
</template>

<style scoped>
.map-pick {
  height: 240px;
  width: 100%;
  margin: 0.5rem 0;
  background: #f3f4f6;
}
</style>
