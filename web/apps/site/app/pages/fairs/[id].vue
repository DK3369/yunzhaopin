<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`fair-${id}`, () => api.get('/v1/wap/zph/detail', { id }))
useSeoMeta({ title: () => String(data.value?.title || t('ui.fairs')) })
useHead({ link: [{ rel: 'canonical', href: `/fairs/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || $t('zph_00001') }}</h1>
    <p v-if="data?.address" class="muted">{{ data.address }} · {{ data.start_at_n }}</p>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.title" class="muted">{{ $t('wap_00603') }}</p>
  </article>
</template>
