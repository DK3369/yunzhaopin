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
