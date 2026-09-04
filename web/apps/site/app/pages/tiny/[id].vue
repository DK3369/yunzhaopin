<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`tiny-${id}`, () => api.get('/v1/wap/tiny-resumes/show', { id }))
useSeoMeta({ title: () => String(data.value?.username || t('ui.tiny')) })
useHead({ link: [{ rel: 'canonical', href: `/tiny/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.username || $t('common_02409') }}</h1>
    <p v-if="data?.job" class="muted">{{ data.job }} · {{ data.exp }}</p>
    <p v-if="data?.production">{{ data.production }}</p>
    <p v-else-if="!data?.username" class="muted">{{ $t('common_02409') }}</p>
  </article>
</template>
