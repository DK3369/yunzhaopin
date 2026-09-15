<script setup lang="ts">
import { ABOUT_CODE_NAMES, descListMatchesCode } from '~/utils/site'

type PageDoc = {
  id?: number
  title?: string
  content?: string
  name?: string
  link_url?: string
}

const LEGAL = new Set(['about', 'contact', 'privacy', 'protocol'])
const code = String(useRoute().params.code || '')
const { t, locale } = useI18n()
const api = useApi()

const { data } = await useAsyncData(
  () => `site-page-${code}-${locale.value}`,
  async () => {
  if (LEGAL.has(code)) {
    try {
      return await api.get<PageDoc>('/v1/wap/legal', { slug: code })
    } catch {
      /* fall through */
    }
  }
  for (const name of ABOUT_CODE_NAMES[code] || []) {
    const row = await api.post<PageDoc>('/v1/wap/descriptions/by-name', { name }).catch(() => null)
    if (row?.content || row?.title || row?.name) return row
  }
  try {
    const listed = await api.post<{ list: Array<{ id: number; link_url?: string }> }>(
      '/v1/wap/descriptions',
      { page: 1, page_size: 80 },
    )
    const hit = (listed.list || []).find((r) => descListMatchesCode(r.link_url, code))
    if (hit?.id) {
      return await api.post<PageDoc>('/v1/wap/descriptions/get', { id: hit.id })
    }
  } catch {
    /* fall through */
  }
  try {
    return await api.get<PageDoc>('/v1/wap/site/pages', { code })
  } catch {
    return null
  }
})

const title = computed(() => String(data.value?.name || data.value?.title || t('ui.pages')))
useSeoMeta({ title: () => title.value })
useHead({ link: [{ rel: 'canonical', href: `/pages/${code}` }] })
</script>

<template>
  <AboutShell :title="title" :current-id="data?.id" :current-name="data?.name">
    <div v-if="data?.content" v-html="String(data.content)" />
    <p v-else class="muted">{{ $t('common_02409') }}</p>
  </AboutShell>
</template>
