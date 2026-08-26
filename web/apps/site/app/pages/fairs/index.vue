<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(
  () => `fairs-${page.value}`,
  () => api.get('/v1/wap/zph', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: t('wap_00223') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00223') }}</h1>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/fairs/${row.id}`"
        :title="row.title"
        :meta="`${row.city_name || ''} · ${row.start_at_n || ''}`"
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
