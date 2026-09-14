<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = { id: number; reason: string; is_read: number; created_at?: number }

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-warnings', () =>
  api.post<{ list: Row[]; total: number }>('/v1/mcenter/warnings', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])

async function mark(id: number) {
  try {
    await api.post('/v1/mcenter/warnings/read', { id })
    await refresh()
  } catch {
    /* ignore */
  }
}

useSeoMeta({ title: t('member_com_00148') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00148')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <div v-if="(data?.list || []).length" class="sysynews_tit site-pc">
      <div class="sysynews_span sysynews_name">{{ $t('member_com_00148') }}</div>
    </div>
    <div v-for="row in list" :key="row.id" class="sysynews_list site-pc" @click="mark(row.id)">
      <div class="sysynews_span sysynews_name">{{ row.reason }}</div>
    </div>
    <div class="site-h5 m_cardbox">
      <MemberSxNewsCard v-for="row in list" :key="'h5-' + row.id" :title="row.reason" />
    </div>
  </MemberPanel>
</template>
