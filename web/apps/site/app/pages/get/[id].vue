<script setup lang="ts">
import { descHref, isExternalHref } from '~/utils/site'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`site-desc-${id}`, async () => {
  if (!Number.isFinite(id) || id <= 0) return null
  try {
    return await api.post<{
      id?: number
      title?: string
      content?: string
      name?: string
      link_url?: string
    }>('/v1/wap/descriptions/get', { id })
  } catch {
    return null
  }
})

const mapped = computed(() =>
  data.value ? descHref({ id, link_url: data.value.link_url }) : `/get/${id}`,
)
const ext = computed(() => isExternalHref(data.value?.link_url))

if (import.meta.server && ext.value) {
  await navigateTo(String(data.value?.link_url), { external: true, redirectCode: 302 })
} else if (import.meta.server && mapped.value && mapped.value !== `/get/${id}`) {
  await navigateTo(mapped.value, { redirectCode: 302 })
}

onMounted(() => {
  if (ext.value) {
    navigateTo(String(data.value?.link_url), { external: true })
    return
  }
  if (mapped.value && mapped.value !== `/get/${id}`) {
    navigateTo(mapped.value)
  }
})

const title = computed(() => String(data.value?.name || data.value?.title || t('ui.pages')))
const showCms = computed(() => !ext.value && mapped.value === `/get/${id}`)
useSeoMeta({ title: () => title.value })
useHead({ link: [{ rel: 'canonical', href: `/get/${id}` }] })
</script>

<template>
  <AboutShell v-if="showCms" :title="title" :current-id="id" :current-name="data?.name">
    <div v-if="data?.content" v-html="String(data.content)" />
    <p v-else class="muted">{{ $t('common_02409') }}</p>
  </AboutShell>
</template>
