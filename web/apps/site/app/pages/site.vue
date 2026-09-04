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
}
type Group = { letter: string; list: SubSite[] }

const { t } = useI18n()
const api = useApi()
const { saveSite, clearSite } = useSubSite()
const { data } = await useAsyncData('sub-sites', () => api.get<Group[]>('/v1/wap/site/sub-sites').catch(() => [] as Group[]))
const groups = computed(() => (Array.isArray(data.value) ? data.value : []) as Group[])

function pick(row: SubSite) {
  saveSite(row)
  navigateTo('/')
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
        <a v-for="row in g.list" :key="row.id" href="javascript:;" @click.prevent="pick(row)">{{ row.title }}</a>
      </p>
    </div>
    <p v-if="!groups.length" class="muted">{{ $t('common_02409') }}</p>
  </section>
</template>
