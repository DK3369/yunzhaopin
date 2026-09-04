<script setup lang="ts">
import { mediaUrl, PLACEHOLDER_LOGO } from '~/utils/site'

const route = useRoute()
const { t, locale } = useI18n()
const uid = Number(route.params.uid)
const id = Number(route.params.id)
const api = useApi()
const { data, error } = await useAsyncData(
  () => `company-product-${locale.value}-${uid}-${id}`,
  () => api.get('/v1/wap/companies/products/detail', { uid, id }),
)
const row = computed(() => (data.value || {}) as Record<string, unknown>)
useSeoMeta({
  title: () => String(row.value.title || t('company_00020')),
  description: () => stripHtml(row.value.body || row.value.title),
})
</script>

<template>
  <article class="site-inner">
    <p>
      <NuxtLink :to="`/companies/${uid}`">{{ $t('common.company') }}</NuxtLink>
    </p>
    <h1>{{ row.title || $t('company_00020') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
    <img
      v-if="row.cover_n || row.cover"
      :src="mediaUrl(String(row.cover_n || row.cover || ''), PLACEHOLDER_LOGO)"
      alt=""
    />
    <div v-if="row.body" v-html="String(row.body)" />
  </article>
</template>
