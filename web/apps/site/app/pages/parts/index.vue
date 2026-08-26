<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(
  () => `parts-${page.value}`,
  () => api.get('/v1/wap/parts', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: t('ui.part') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.part') }}</h1>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/parts/${row.id}`"
        :title="row.name"
        :meta="`${row.com_name || ''} · ${row.city_name || ''}`"
      />
    </div>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { page: p } })"
    />
  </section>
</template>
