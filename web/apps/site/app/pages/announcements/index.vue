<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData('announcements', () =>
  api.get<{ list: Array<{ id: number; title: string; datetime_n?: string }> }>('/v1/wap/announcements', {
    page: 1,
    page_size: 20,
  }),
)
useSeoMeta({ title: t('common.site_notice') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <NewsListShell :title="$t('common.site_notice')" :error="error" :error-text="failMsg" :count="list.length">
    <SimpleCard v-for="a in list" :key="a.id" :to="`/announcements/${a.id}`" :title="a.title" :meta="a.datetime_n" />
  </NewsListShell>
</template>
