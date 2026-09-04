<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`special-${id}`, () => api.get('/v1/wap/specials/detail', { id }))
useSeoMeta({ title: () => String(data.value?.title || t('ui.specials')) })
useHead({ link: [{ rel: 'canonical', href: `/specials/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || $t('common_02409') }}</h1>
    <p v-if="data?.intro" class="muted">{{ data.intro }}</p>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.title" class="muted">{{ $t('common_02409') }}</p>
  </article>
</template>
