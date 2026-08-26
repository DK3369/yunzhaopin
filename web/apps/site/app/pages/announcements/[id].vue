<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`ann-${id}`, () =>
  api.get('/v1/wap/announcements/detail', { id }),
)
useSeoMeta({ title: () => String(data.value?.title || t('ui.announcements')) })
</script>

<template>
  <article>
    <h1>{{ data?.title || $t('home.no_job_data') }}</h1>
    <div v-if="data?.content || data?.body" v-html="data?.content || data?.body" />
    <p v-else class="muted">{{ $t('home.no_job_data') }}</p>
  </article>
</template>
