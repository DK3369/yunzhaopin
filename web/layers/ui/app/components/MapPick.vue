<script setup lang="ts">
type PoiHit = { name: string; address: string; lng: number; lat: number }

const x = defineModel<string>('x', { default: '' })
const y = defineModel<string>('y', { default: '' })
const props = defineProps<{ preset?: string }>()
const mapEl = ref<HTMLDivElement | null>(null)
const { settings } = useSiteChrome()
const ready = ref(false)
const keyword = ref('')
const hits = ref<PoiHit[]>([])
const searching = ref(false)
type AMapInst = {
  on: (ev: string, fn: (e: { lnglat: { getLng: () => number; getLat: () => number } }) => void) => void
  setCenter: (pos: [number, number]) => void
}
let putFn: ((lng: number, lat: number) => void) | null = null

watch(
  () => props.preset,
  (v) => {
    if (v && !keyword.value) keyword.value = v
  },
  { immediate: true },
)

onMounted(async () => {
  const key = String(settings.value.map_key || '').trim()
  const secret = String(settings.value.map_secret || settings.value.map_security || '').trim()
  const ok = await loadAmap(key, secret)
  ready.value = ok
  await nextTick()
  if (!ok || !mapEl.value) return
  const AMap = (
    window as unknown as {
      AMap: {
        Map: new (...a: unknown[]) => AMapInst
        Marker: new (...a: unknown[]) => { setPosition: Function }
      }
    }
  ).AMap
  const center = mapCenter(settings.value, x.value, y.value)
  const map = new AMap.Map(mapEl.value, { zoom: 13, center })
  const put = (lng: number, lat: number) => {
    x.value = String(lng)
    y.value = String(lat)
    const pos: [number, number] = [lng, lat]
    if (marker) marker.setPosition(pos)
    else marker = new AMap.Marker({ position: pos, map })
    map.setCenter(pos)
  }
  putFn = put
  if (x.value && y.value) put(Number(x.value), Number(y.value))
  map.on('click', (e: { lnglat: { getLng: () => number; getLat: () => number } }) => {
    put(e.lnglat.getLng(), e.lnglat.getLat())
    hits.value = []
  })
})

function locOf(raw: unknown): { lng: number; lat: number } | null {
  if (!raw || typeof raw !== 'object') return null
  const loc = raw as { lng?: number; lat?: number; getLng?: () => number; getLat?: () => number }
  const lng = typeof loc.getLng === 'function' ? loc.getLng() : Number(loc.lng)
  const lat = typeof loc.getLat === 'function' ? loc.getLat() : Number(loc.lat)
  if (!Number.isFinite(lng) || !Number.isFinite(lat)) return null
  return { lng, lat }
}

function searchPlace() {
  const q = keyword.value.replace(/\s+/g, '')
  if (q.length < 2) return
  const AMap = (window as unknown as { AMap?: { plugin: Function } }).AMap
  if (!AMap?.plugin) return
  searching.value = true
  AMap.plugin(['AMap.PlaceSearch'], () => {
    const Ctor = (window as unknown as { AMap: { PlaceSearch: new (o: Record<string, unknown>) => { search: Function } } }).AMap
      .PlaceSearch
    const ps = new Ctor({ pageSize: 8, pageIndex: 1 })
    ps.search(q, (status: string, result: { poiList?: { pois?: Array<Record<string, unknown>> } }) => {
      searching.value = false
      if (status !== 'complete') {
        hits.value = []
        return
      }
      hits.value = (result.poiList?.pois || [])
        .map((p) => {
          const xy = locOf(p.location)
          if (!xy) return null
          return {
            name: String(p.name || ''),
            address: String(p.address || p.pname || ''),
            lng: xy.lng,
            lat: xy.lat,
          }
        })
        .filter((p): p is PoiHit => Boolean(p))
    })
  })
}

function pickHit(hit: PoiHit) {
  putFn?.(hit.lng, hit.lat)
  keyword.value = hit.name
  hits.value = []
}
</script>

<template>
  <div>
    <div class="joblist_mapsearch_box">
      <input
        v-model="keyword"
        type="text"
        class="joblist_mapsearch_text"
        :placeholder="$t('admin_00149')"
        @keydown.enter.prevent="searchPlace"
      />
      <input type="button" class="joblist_mapsearch_bth" :value="$t('member_com_00608')" @click="searchPlace" />
      <div v-if="hits.length" class="comEleaseMaps">
        <ul>
          <li v-for="(hit, i) in hits" :key="i">
            <a href="javascript:;" @click.prevent="pickHit(hit)">{{ hit.name }} {{ hit.address }}</a>
          </li>
        </ul>
      </div>
    </div>
    <div ref="mapEl" class="map-pick" />
    <p class="muted">
      {{ $t('member_com_00605') }} {{ x || '—' }}
      {{ $t('member_com_00606') }} {{ y || '—' }}
    </p>
    <p v-if="!ready" class="muted">{{ $t('ui.lng') }} / {{ $t('ui.lat') }}</p>
    <p v-if="searching" class="muted">{{ $t('common.search') }}…</p>
  </div>
</template>

<style scoped>
.map-pick {
  height: 240px;
  width: 100%;
  margin: 0.5rem 0;
  background: #f3f4f6;
}
.joblist_mapsearch_box {
  position: relative;
  margin: 0.5rem 0;
}
.comEleaseMaps {
  position: absolute;
  z-index: 8;
  left: 0;
  right: 0;
  background: #fff;
  border: 1px solid #e5e7eb;
  max-height: 12rem;
  overflow: auto;
}
.comEleaseMaps a {
  display: block;
  padding: 0.4rem 0.6rem;
}
</style>
