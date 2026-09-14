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
  <MemberPanel :title="$t('wap_user_00008')">
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <div class="site-h5 yun_usermember_financebg">
        <div class="yun_usermember_integral">
          {{ $t('wap_01018') }}{{ $t('wap_user_00008') }}：
          <span class="yun_usermember_integral_n">{{ bal?.balance ?? 0 }}</span>
        </div>
      </div>
      <div class="site-h5 yun_usermember_integral_b">
        <div class="yun_usermember_integral_line" />
        <ul class="yun_usermember_integral_nav">
          <li><NuxtLink to="/com/member-right"><i class="yun_usermember_integral_nav_icon yun_usermember_integral_nav_iconmx" />{{ $t('wap_com_00097') }}</NuxtLink></li>
          <li><NuxtLink to="/com/pay"><i class="yun_usermember_integral_nav_icon yun_usermember_integral_nav_icongz" />{{ $t('member_com_00041') }}</NuxtLink></li>
          <li><NuxtLink to="/com/orders"><i class="yun_usermember_integral_nav_icon yun_usermember_integral_nav_iconsc" />{{ $t('common_02029') }}</NuxtLink></li>
        </ul>
      </div>
      <div class="site-pc job_list_tit">
        <ul>
          <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('wap_user_00008') }}</a></li>
          <li><NuxtLink to="/com/pay">{{ $t('member_com_00041') }}</NuxtLink></li>
          <li><NuxtLink to="/com/orders">{{ $t('common_02029') }}</NuxtLink></li>
        </ul>
      </div>
      <p class="site-pc muted">{{ $t('ui.balance') }} {{ bal?.balance ?? 0 }}</p>
      <div class="site-pc integral_list_box">
        <ul class="integral_list">
          <li v-for="(row, i) in tasks" :key="i">
            <div class="integral_list_n integral_list_n_c1">{{ row.reward }}</div>
            <div class="integral_listname">{{ row.title }}</div>
            <div class="integral_list_p">{{ row.reward }}</div>
            <div class="integral_list_bth">
              <span v-if="row.done" class="integral_list_bth_s">{{ row.doneText }}</span>
              <a v-else-if="row.sign" href="javascript:;" class="integral_list_bth_a" @click="sign">{{ row.go }}</a>
              <NuxtLink v-else-if="row.to" :to="row.to" class="integral_list_bth_a">{{ row.go }}</NuxtLink>
            </div>
          </li>
        </ul>
      </div>
      <div class="site-h5 yun_usermember_integral_box">
        <div class="yun_usermember_integral_box_h1">{{ $t('wap_01021') }}</div>
        <ul class="yun_usermember_integral_list">
          <li v-for="(row, i) in tasks" :key="'h5-' + i">
            <div class="yun_integral_name">{{ row.title }}</div>
            <div class="yun_integral_n">{{ row.reward }}</div>
            <span v-if="row.done" class="yun_integral_bth">{{ row.doneText }}</span>
            <a v-else-if="row.sign" href="javascript:;" class="yun_integral_a" @click="sign">{{ row.go }}</a>
            <NuxtLink v-else-if="row.to" :to="row.to" class="yun_integral_a">{{ row.go }}</NuxtLink>
          </li>
        </ul>
      </div>
      <MemberResumeH1 :title="$t('wap_01020')" />
      <div v-for="row in consumes?.list || []" :key="row.id" class="site-pc paylist_list">
        <span class="paylist_span paylist_span_dh">{{ row.detail }}</span>
        <span class="paylist_span paylist_span_money">{{ row.delta }}</span>
        <span class="paylist_span paylist_span_time">{{ row.ctime_n }}</span>
      </div>
      <MemberPager :page="consumePage" :page-size="PAGE_SIZE" :total="Number(consumes?.total || 0)" @update:page="(p) => (consumePage = p)" />
      <MemberResumeH1 :title="$t('wap_user_00170')" />
      <div v-for="row in exchanges?.list || []" :key="row.id" class="site-pc paylist_list">
        <span class="paylist_span paylist_span_dh">{{ row.item_name || row.item_id }}</span>
        <span class="paylist_span paylist_span_money">{{ row.cost }}</span>
        <span class="paylist_span paylist_span_time">{{ row.created_at_n || row.created_at }}</span>
      </div>
      <MemberPager :page="exchangePage" :page-size="PAGE_SIZE" :total="Number(exchanges?.total || 0)" @update:page="(p) => (exchangePage = p)" />
      <form class="com_release_box" @submit.prevent="transfer">
        <ul>
          <MemberReleaseRow label="uid"><input v-model.number="toUid" type="number" min="1" /></MemberReleaseRow>
          <MemberReleaseRow :label="$t('wap_user_00008')"><input v-model.number="points" type="number" min="1" /></MemberReleaseRow>
          <MemberReleaseRow :label="$t('ui.desc')"><input v-model="note" class="com_release_textnew_text" /></MemberReleaseRow>
        </ul>
        <button type="submit" class="btn_01">{{ $t('common.submit') }}</button>
      </form>
      <div v-for="row in transfers?.list || []" :key="row.id" class="site-pc paylist_list">
        <span class="paylist_span paylist_span_dh">{{ row.from_uid }} → {{ row.to_uid }}</span>
        <span class="paylist_span paylist_span_money">{{ row.points }}</span>
        <span class="paylist_span paylist_span_time">{{ row.note }}</span>
      </div>
      <MemberPager :page="transferPage" :page-size="PAGE_SIZE" :total="Number(transfers?.total || 0)" @update:page="(p) => (transferPage = p)" />
      <p v-if="msg">{{ msg }}</p>
    </template>
    <p>
      <NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink>
    </p>
  </MemberPanel>
</template>

<style scoped>
.balance {
  font-size: 1.25rem;
  font-weight: 600;
}
</style>
