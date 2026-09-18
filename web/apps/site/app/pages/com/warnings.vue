<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'

type Row = { id: number; reason: string; is_read: number; created_at?: number }

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-warnings-${page.value}`,
  () => api.post<{ list: Row[]; total: number }>('/v1/mcenter/warnings', { page: page.value, page_size: pageSize }),
)
const list = computed(() => data.value?.list || [])
const total = computed(() => inferTotal(data.value, list.value))

async function mark(id: number) {
  try {
    await api.post('/v1/mcenter/warnings/read', { id })
    await refresh()
  } catch {
    /* ignore */
  }
}

function unread(row: Row) {
  return Number(row.is_read) !== 1
}

useSeoMeta({ title: t('ui.warnings') })
</script>

<template>
  <MemberPanel :title="$t('ui.warnings')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <table v-if="list.length" class="com_table mt20 site-pc">
      <tr>
        <th>{{ $t('ui.warnings') }}</th>
        <th>{{ $t('member_user_00104') }}</th>
      </tr>
      <tr v-for="row in list" :key="row.id" @click="mark(row.id)">
        <td :style="unread(row) ? 'font-weight:bold' : ''">{{ row.reason }}</td>
        <td>{{ formatUnixDate(row.created_at) }}</td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in list" :key="'h5-' + row.id" class="com_cardlist" @click="mark(row.id)">
        <div class="com_cardlist_tit" :style="unread(row) ? 'font-weight:bold' : ''">{{ row.reason }}</div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('member_user_00104') }}</span>
          {{ formatUnixDate(row.created_at) }}
        </div>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
