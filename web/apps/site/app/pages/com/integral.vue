<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type ConsumeRow = {
  id: number
  opera: number
  detail: string
  delta: number
  ctime_n: string
}
type ExchangeRow = {
  id: number
  item_id: number
  item_name?: string
  cost: number
  status: number
  created_at: number
  created_at_n?: string
}

const api = useApi()
const { t } = useI18n()

const PAGE_SIZE = 20
const consumePage = ref(1)
const exchangePage = ref(1)
const transferPage = ref(1)
const toUid = ref(0)
const points = ref(0)
const note = ref('')
const msg = ref('')

const { data: bal, error } = await useAsyncData('com-integral-balance', () =>
  api.post<{ balance: number }>('/v1/mcenter/integral/balance', {}),
)
const { data: consumes } = await useAsyncData(
  'com-integral-consumes',
  () =>
    api
      .post<{ list: ConsumeRow[]; total: number }>('/v1/mcenter/integral/consumes', {
        page: consumePage.value,
        page_size: PAGE_SIZE,
      })
      .catch(() => ({ list: [] as ConsumeRow[], total: 0 })),
  { watch: [consumePage] },
)
const { data: exchanges } = await useAsyncData(
  'com-integral-history',
  () =>
    api
      .post<{ list: ExchangeRow[]; total: number }>('/v1/mcenter/integral/history', {
        page: exchangePage.value,
        page_size: PAGE_SIZE,
      })
      .catch(() => ({ list: [] as ExchangeRow[], total: 0 })),
  { watch: [exchangePage] },
)
const { data: transfers, refresh: refreshTransfers } = await useAsyncData(
  'com-integral-transfers',
  () =>
    api
      .post<{ list: Array<{ id: number; from_uid: number; to_uid: number; points: number; note: string; created_at: number }>; total: number }>(
        '/v1/mcenter/integral/transfers',
        { page: transferPage.value, page_size: PAGE_SIZE },
      )
      .catch(() => ({ list: [], total: 0 })),
  { watch: [transferPage] },
)

async function transfer() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/integral/transfer', {
      to_uid: toUid.value,
      points: points.value,
      note: note.value,
    })
    msg.value = t('common.success')
    toUid.value = 0
    points.value = 0
    note.value = ''
    await refreshTransfers()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_user_00008') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00008') }}</h1>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <p class="balance">{{ $t('ui.balance') }}: {{ bal?.balance ?? 0 }}</p>
      <nav class="stack">
        <NuxtLink to="/com/member-right" class="job-card">{{ $t('wap_com_00097') }}</NuxtLink>
        <NuxtLink to="/com/pay" class="job-card">{{ $t('member_com_00041') }}</NuxtLink>
        <NuxtLink to="/com/orders" class="job-card">{{ $t('common_02029') }}</NuxtLink>
      </nav>

      <h2>{{ $t('wap_01020') }}</h2>
      <p v-if="!(consumes?.list || []).length" class="muted">{{ $t('ui.no_data') }}</p>
      <article v-for="row in consumes?.list || []" :key="row.id" class="job-card">
        <p>{{ row.detail }} · {{ row.delta }}</p>
        <p class="muted">{{ row.ctime_n }}</p>
      </article>
      <Pager
        v-model:page="consumePage"
        :page-size="PAGE_SIZE"
        :total="Number(consumes?.total || 0)"
      />

      <h2>{{ $t('wap_user_00170') }}</h2>
      <p v-if="!(exchanges?.list || []).length" class="muted">{{ $t('default_00284') }}</p>
      <article v-for="row in exchanges?.list || []" :key="row.id" class="job-card">
        <p>{{ row.item_name || row.item_id }} · {{ row.cost }}</p>
        <p class="muted">{{ row.created_at_n || row.created_at }}</p>
      </article>
      <Pager
        v-model:page="exchangePage"
        :page-size="PAGE_SIZE"
        :total="Number(exchanges?.total || 0)"
      />

      <h2>{{ $t('common.submit') }}</h2>
      <form class="form" @submit.prevent="transfer">
        <input v-model.number="toUid" type="number" min="1" placeholder="uid" />
        <input v-model.number="points" type="number" min="1" />
        <input v-model="note" :placeholder="$t('ui.desc')" />
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <article v-for="row in transfers?.list || []" :key="row.id" class="job-card">
        <p>{{ row.from_uid }} → {{ row.to_uid }} · {{ row.points }}</p>
        <p class="muted">{{ row.note }}</p>
      </article>
      <Pager
        v-model:page="transferPage"
        :page-size="PAGE_SIZE"
        :total="Number(transfers?.total || 0)"
      />
      <p v-if="msg">{{ msg }}</p>
    </template>
    <p>
      <NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink>
    </p>
  </section>
</template>

<style scoped>
.balance {
  font-size: 1.25rem;
  font-weight: 600;
}
</style>
