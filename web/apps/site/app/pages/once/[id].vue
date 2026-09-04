<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`once-${id}`, () => api.get('/v1/wap/once-jobs/show', { id }))
useSeoMeta({ title: () => String(data.value?.title || data.value?.companyname || t('ui.once')) })
useHead({ link: [{ rel: 'canonical', href: `/once/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || data?.companyname || $t('wap_00630') }}</h1>
    <p v-if="data?.companyname" class="muted">{{ data.companyname }}</p>
    <p v-if="data?.address" class="muted">{{ data.address }}</p>
    <p v-if="data?.linkman_masked" class="muted">{{ $t('ui.linkman') }} {{ data.linkman_masked }} · {{ data.linktel_masked }}</p>
    <p v-if="data?.require">{{ data.require }}</p>
    <p v-else-if="!data?.companyname && !data?.title" class="muted">{{ $t('wap_00630') }}</p>
  </article>
</template>
