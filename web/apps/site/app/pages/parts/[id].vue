<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`part-${id}`, () => api.get('/v1/wap/parts/detail', { id }))
useSeoMeta({ title: () => String(data.value?.name || t('ui.part')) })
useHead({ link: [{ rel: 'canonical', href: `/parts/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || $t('member_com_00477') }}</h1>
    <p v-if="data?.com_name || data?.address" class="muted">{{ data.com_name }} {{ data.address }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.name" class="muted">{{ $t('member_com_00477') }}</p>
  </article>
</template>
