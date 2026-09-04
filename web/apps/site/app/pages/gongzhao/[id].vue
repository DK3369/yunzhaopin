<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`gz-${id}`, () => api.get('/v1/wap/gongzhao/detail', { id }))
useSeoMeta({ title: () => String(data.value?.title || t('ui.gongzhao')) })
useHead({ link: [{ rel: 'canonical', href: `/gongzhao/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || $t('common_02409') }}</h1>
    <p v-if="data?.start_at_n" class="muted">{{ data.start_at_n }} — {{ data.end_at_n }}</p>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.title" class="muted">{{ $t('common_02409') }}</p>
  </article>
</template>
