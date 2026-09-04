<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`site-desc-${id}`, async () => {
  if (!Number.isFinite(id) || id <= 0) return null
  try {
    return await api.post<{ title?: string; content?: string; name?: string; link_url?: string }>(
      '/v1/wap/descriptions/get',
      { id },
    )
  } catch {
    return null
  }
})
const link = computed(() => String(data.value?.link_url || '').trim())
if (import.meta.server && link.value && /^https?:\/\//i.test(link.value)) {
  await navigateTo(link.value, { external: true, redirectCode: 302 })
}
onMounted(() => {
  if (link.value && /^https?:\/\//i.test(link.value)) {
    navigateTo(link.value, { external: true })
  }
})
useSeoMeta({ title: () => String(data.value?.title || data.value?.name || t('ui.pages')) })
useHead({ link: [{ rel: 'canonical', href: `/get/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || data?.title || $t('ui.page_missing') }}</h1>
    <p v-if="link">
      <a :href="link" rel="nofollow noopener">{{ link }}</a>
    </p>
    <div v-else-if="data?.content" v-html="String(data.content)" />
    <p v-else class="muted">{{ $t('common_02409') }}</p>
  </article>
</template>
