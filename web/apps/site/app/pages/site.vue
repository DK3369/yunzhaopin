<script setup lang="ts">
type SubSite = {
  id: number
  title: string
  domain: string
  province?: number | null
  city_id?: number | null
  three_city_id?: number | null
  hy?: number | null
  fz_type?: number
  web_name?: string | null
  web_title?: string | null
  web_logo?: string | null
  mode?: number
  indexdir?: string | null
}
type CityItem = { id: number; name: string; letter: string; site: SubSite }
type Group = { letter: string; list: CityItem[] }
type Payload = { city_groups?: Group[]; hy?: SubSite[] }

const { t } = useI18n()
const api = useApi()
const { saveSite, clearSite } = useSubSite()
const { data } = await useAsyncData('sub-sites', () =>
  api.get<Payload>('/v1/wap/site/sub-sites').catch(() => ({ city_groups: [] as Group[], hy: [] as SubSite[] })),
)
const groups = computed(() => (Array.isArray(data.value?.city_groups) ? data.value.city_groups : []) as Group[])
const hyList = computed(() => (Array.isArray(data.value?.hy) ? data.value.hy : []) as SubSite[])

function goSite(row: SubSite) {
  saveSite(row)
  const mode = Number(row.mode || 0)
  if (mode === 1 && row.domain) {
    const host = String(row.domain).replace(/^https?:\/\//, '')
    if (import.meta.client) {
      window.location.href = `${window.location.protocol}//${host}`
      return
    }
  }
  if (mode === 2 && row.indexdir) {
    const dir = String(row.indexdir).replace(/^\/+|\/+$/g, '')
    if (dir) {
      navigateTo(`/${dir}/`)
      return
    }
  }
  navigateTo('/')
}

function pickCity(item: CityItem) {
  goSite(item.site)
}

useSeoMeta({ title: t('ui.pick_site') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.pick_site') }}</h1>
    <p><a href="javascript:;" @click.prevent="clearSite(); navigateTo('/')">{{ $t('common.all') }}</a></p>
    <div v-for="g in groups" :key="g.letter">
      <h2>{{ g.letter }}</h2>
      <p>
        <a v-for="row in g.list" :key="row.id" href="javascript:;" @click.prevent="pickCity(row)">{{ row.name || row.site.title }}</a>
      </p>
    </div>
    <div v-if="hyList.length">
      <p>
        <a v-for="row in hyList" :key="'hy-' + row.id" href="javascript:;" @click.prevent="goSite(row)">{{ row.title }}</a>
      </p>
    </div>
    <p v-if="!groups.length && !hyList.length" class="muted">{{ $t('common_02409') }}</p>
  </section>
</template>
