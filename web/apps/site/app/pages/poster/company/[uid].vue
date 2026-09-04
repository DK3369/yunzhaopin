<script setup lang="ts">
const route = useRoute()
const uid = Number(route.params.uid)
const { t } = useI18n()
const api = useApi()
const { data: tpls } = await useAsyncData(`com-poster-tpls`, () =>
  api.post<Array<{ id: number; title: string; pic?: string }>>('/v1/wap/posters/templates', { kind: 'company' }).catch(() => []),
)
const hb = computed(() => Number(route.query.hb || 0) || undefined)
const { data: spec } = await useAsyncData(
  () => `com-poster-${uid}-${hb.value || 0}`,
  () => api.post('/v1/wap/posters', { kind: 'company', id: uid, hb: hb.value }).catch(() => null),
)
useSeoMeta({ title: t('ui.poster') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.poster') }}</h1>
    <p>
      <NuxtLink v-for="row in tpls || []" :key="row.id" :to="{ query: { hb: row.id } }">{{ row.title }}</NuxtLink>
    </p>
    <p v-if="spec?.template?.pic"><img :src="spec.template.pic" alt="" /></p>
    <pre v-if="spec?.fields">{{ spec.fields }}</pre>
    <p v-else class="muted">{{ $t('common_02409') }}</p>
  </section>
</template>
