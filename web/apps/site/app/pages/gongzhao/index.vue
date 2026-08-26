<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(
  () => `gz-${page.value}`,
  () => api.get('/v1/wap/gongzhao', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: t('ui.gongzhao') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.gongzhao') }}</h1>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
    <div class="stack">
      <SimpleCard
        v-for="row in data?.list || []"
        :key="row.id"
        :to="`/gongzhao/${row.id}`"
        :title="row.title"
        :meta="row.start_at_n"
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
