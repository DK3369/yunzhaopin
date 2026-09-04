<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`reward-${id}`, () =>
  api.get('/v1/wap/redeem/rewards/detail', { id }),
)
useSeoMeta({ title: () => String(data.value?.name || t('ui.redeem')) })
useHead({ link: [{ rel: 'canonical', href: `/redeem/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || $t('wap_00611') }}</h1>
    <p v-if="data?.integral" class="muted">{{ data.integral }} {{ $t('ui.integral') }} · {{ $t('ui.stock') }} {{ data.remaining }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.name" class="muted">{{ $t('wap_00611') }}</p>
  </article>
</template>
