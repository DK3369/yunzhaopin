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
const { settings } = useSiteChrome()

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

type Mission = {
  base_info?: boolean
  logo?: boolean
  signin?: boolean
  email_checked?: boolean
  phone_checked?: boolean
  weixin_bind?: boolean
  map?: boolean
  banner?: boolean
  yyzz?: boolean
  question?: boolean
  answer?: boolean
  answerpl?: boolean
}
const { data: mission, refresh: refreshMission } = await useAsyncData('com-integral-mission', () =>
  api.post<Mission>('/v1/mcenter/integral/mission', {}).catch(() => null),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('com-sign-status', () =>
  api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
function pts(key: string) {
  const n = String(settings.value[key] || '').trim()
  return n ? `+${n}` : ''
}
const tasks = computed(() => [
  { done: mission.value?.signin || signSt.value?.signed_today, title: t('wap_user_00114'), reward: pts('integral_signin'), to: '', doneText: t('wap_00989'), go: t('wap_user_00118'), sign: true },
  { done: false, title: t('wap_user_00108'), reward: pts('integral_invite_reg'), to: '/invite', doneText: '', go: t('wap_user_00121'), sign: false },
  { done: mission.value?.logo, title: t('wap_com_00180'), reward: pts('integral_avatar'), to: '/com/profile', doneText: t('wap_user_00123'), go: t('wap_user_00116'), sign: false },
  { done: mission.value?.phone_checked, title: t('wap_user_00109'), reward: pts('integral_mobliecert'), to: '/com/binding', doneText: t('wap_user_00128'), go: t('wap_user_00120'), sign: false },
  { done: mission.value?.weixin_bind, title: t('wap_user_00115'), reward: pts('integral_bind_wx'), to: '/com/binding', doneText: t('wap_user_00127'), go: t('wap_user_00119'), sign: false },
  { done: mission.value?.map, title: t('wap_com_00182'), reward: pts('integral_map'), to: '/com/profile', doneText: t('wap_com_00189'), go: t('wap_com_00185'), sign: false },
  { done: mission.value?.yyzz, title: t('wap_com_00181'), reward: pts('integral_comcert'), to: '/com/cert', doneText: t('wap_user_00128'), go: t('wap_user_00120'), sign: false },
  { done: mission.value?.base_info, title: t('wap_00990'), reward: pts('integral_userinfo'), to: '/com/profile', doneText: t('wap_user_00125'), go: t('wap_user_00117'), sign: false },
  { done: mission.value?.email_checked, title: t('wap_user_00122'), reward: pts('integral_emailcert'), to: '/com/binding', doneText: t('wap_user_00128'), go: t('wap_com_00186'), sign: false },
  { done: mission.value?.banner, title: t('wap_com_00033'), reward: pts('integral_banner'), to: '/com/banners', doneText: t('wap_user_00123'), go: t('wap_com_00183'), sign: false },
  { done: mission.value?.question, title: t('wap_user_00112'), reward: pts('integral_question'), to: '/questions', doneText: t('wap_00992'), go: t('wap_com_00184'), sign: false },
  { done: mission.value?.answer, title: t('wap_user_00113'), reward: pts('integral_answer'), to: '/questions', doneText: t('wap_com_00188'), go: t('wap_user_00113'), sign: false },
  { done: mission.value?.answerpl, title: t('wap_00994'), reward: pts('integral_answerpl'), to: '/questions', doneText: t('wap_com_00187'), go: t('wap_00994'), sign: false },
])
async function sign() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sign', {})
    msg.value = t('common.success')
    await refreshSign()
    await refreshMission()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

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

      <h2>{{ $t('wap_01021') }}</h2>
      <article v-for="(row, i) in tasks" :key="i" class="job-card">
        <p>{{ row.title }} <span v-if="row.reward" class="muted">{{ row.reward }}</span></p>
        <p v-if="row.done" class="muted">{{ row.doneText }}</p>
        <p v-else-if="row.sign">
          <button type="button" :disabled="!!signSt?.signed_today" @click="sign">{{ row.go }}</button>
        </p>
        <p v-else-if="row.to">
          <NuxtLink :to="row.to">{{ row.go }}</NuxtLink>
        </p>
      </article>

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
