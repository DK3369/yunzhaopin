<script setup lang="ts">
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData('announcements', () =>
  api.get('/v1/wap/announcements', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('common.site_notice') })
</script>

<template>
  <section>
    <h1>{{ $t('common.site_notice') }}</h1>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('home.no_job_data') }}</p>
    <div class="stack">
      <NuxtLink v-for="a in data?.list || []" :key="a.id" :to="`/announcements/${a.id}`">
        {{ a.title }}
      </NuxtLink>
    </div>
  </section>
</template>
