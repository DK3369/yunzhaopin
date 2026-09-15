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
    <table v-if="list.length" class="com_table mt20 site-pc">
      <tr>
        <th>{{ $t('member_com_00148') }}</th>
      </tr>
      <tr v-for="row in list" :key="row.id" @click="mark(row.id)">
        <td>{{ row.reason }}</td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in list" :key="'h5-' + row.id" class="com_cardlist" @click="mark(row.id)">
        <div class="com_cardlist_tit">{{ row.reason }}</div>
      </div>
    </div>
  </MemberPanel>
</template>
