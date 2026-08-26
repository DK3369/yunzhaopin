<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`hr-${id}`, () => api.get('/v1/wap/hr-docs/detail', { id }))
useSeoMeta({ title: () => String(data.value?.name || t('ui.hr')) })
useHead({ link: [{ rel: 'canonical', href: `/hr/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || $t('home.no_job_data') }}</h1>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.name" class="muted">{{ $t('home.no_job_data') }}</p>
  </article>
</template>
