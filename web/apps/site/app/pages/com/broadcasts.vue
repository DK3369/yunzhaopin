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
    <table v-if="list.length" class="com_table mt20 site-pc">
      <tr>
        <th>{{ $t('common.message') }}</th>
        <th>{{ $t('member_user_00104') }}</th>
      </tr>
      <tr v-for="row in list" :key="row.id" @click="mark(row.id)">
        <td>{{ row.title }}</td>
        <td>{{ row.created_at_n }}</td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in list" :key="'h5-' + row.id" class="com_cardlist" @click="mark(row.id)">
        <div class="com_cardlist_tit">{{ row.title }}</div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('member_user_00104') }}</span>
          {{ row.created_at_n }}
        </div>
        <p>{{ row.body }}</p>
      </div>
    </div>
  </MemberPanel>
</template>
