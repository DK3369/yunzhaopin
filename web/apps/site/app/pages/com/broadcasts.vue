<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = { id: number; title: string; body?: string; created_at_n?: string }

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-broadcasts', () =>
  api.post<{ list: Row[]; total: number }>('/v1/mcenter/broadcasts', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])

async function mark(id: number) {
  try {
    await api.post('/v1/mcenter/broadcasts/read', { id })
    await refresh()
  } catch {
    /* ignore */
  }
}

useSeoMeta({ title: t('common.message') })
</script>

<template>
  <MemberPanel :title="$t('common.message')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <article v-for="row in list" :key="row.id" class="look_resume_list" @click="mark(row.id)">
      <h3>{{ row.title }}</h3>
      <p>{{ row.body }}</p>
      <p class="muted">{{ row.created_at_n }}</p>
    </article>
  </MemberPanel>
</template>
