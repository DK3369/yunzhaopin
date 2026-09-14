<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = { id: number; sid: number; title?: string; status: number; datetime_n?: string }

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-specials-mine', () =>
  api.post<{ list: Row[]; total: number }>('/v1/mcenter/specials/mine', { page: 1, page_size: 20 }),
)
const msg = ref('')
const list = computed(() => data.value?.list || [])

async function remove(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/specials/delete', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_com_00310') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00310')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <div v-for="row in list" :key="row.id" class="issue_post_body_card">
      <div class="Posted_card_top">
        <NuxtLink :to="`/specials/${row.sid}`" class="Posted_card_name">{{ row.title || row.sid }}</NuxtLink>
        <div class="Posted_card_pay">{{ row.datetime_n }}</div>
      </div>
      <a href="javascript:;" class="List_dete cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
