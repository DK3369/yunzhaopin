<script setup lang="ts">
import { seoJoin } from '~/utils/seo'

const token = String(useRoute().params.token || '')
const { t } = useI18n()
const api = useApi()
const { settings } = useSiteChrome()
const shareOn = computed(() => String(settings.value.sy_h5_share || '1') !== '2')
const { data, error } = await useAsyncData(`share-resume-${token}`, () =>
  api.post<{ display_name?: string; uid?: number; education?: number }>('/v1/wap/resume-share/view', { token }),
)
useSeoMeta({
  title: () => String(data.value?.display_name || t('common.share')),
  description: () => seoJoin([data.value?.display_name]),
})
</script>

<template>
  <article v-if="shareOn">
    <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
    <template v-else>
      <h1>{{ data?.display_name || $t('common.share') }}</h1>
      <p v-if="data?.uid"><NuxtLink :to="`/resumes/${data.uid}`">{{ $t('common.more') }}</NuxtLink></p>
    </template>
  </article>
  <p v-else class="muted">{{ $t('common_02409') }}</p>
</template>
