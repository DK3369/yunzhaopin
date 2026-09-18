<script setup lang="ts">
import { isComMemberPath, isMemberModuleOn, isUnauthErr } from '~/utils/site'

type Row = {
  id: number
  uid?: number
  to_uid?: number
  name: string
  integral: number
  num: number
  status: number
  status_n: string
  created_at_n: string
}

const api = useApi()
const { t } = useI18n()
const route = useRoute()
const { settings, me } = useSiteChrome()
const mallOn = computed(() => isMemberModuleOn(settings.value, '/redeem'))
const isCom = computed(() => isComMemberPath(route.path))
const { page, pageSize, inferTotal, go } = useMemberListPage()
const msg = ref('')
const tab = ref<'mine' | 'sent' | 'received'>('mine')
watch(tab, () => go(1))
const tabs = computed(() => [
  { value: 'mine' as const, label: t('ui.gift_mine'), on: tab.value === 'mine', select: () => { tab.value = 'mine' } },
  { value: 'sent' as const, label: t('ui.gift_sent'), on: tab.value === 'sent', select: () => { tab.value = 'sent' } },
  { value: 'received' as const, label: t('ui.gift_received'), on: tab.value === 'received', select: () => { tab.value = 'received' } },
])
const { data, error, refresh } = await useAsyncData(
  () => `redeem-orders-${page.value}-${tab.value}`,
  () =>
    api.post<{ list: Row[]; total: number }>('/v1/mcenter/redeem/orders', {
      page: page.value,
      page_size: pageSize,
      tab: tab.value,
    }),
  { watch: [tab] },
)
const list = computed(() => data.value?.list || [])
const total = computed(() => inferTotal(data.value))
const myUid = computed(() => Number(me.value?.uid || 0))

function canCancel(row: Row) {
  return row.status === 0 && Number(row.uid || 0) === myUid.value
}

async function cancel(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/redeem/orders/cancel', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_user_00170') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00170')" :error="error && !isUnauthErr(error) ? error : undefined">
    <template #pcTabs><MemberComVipTabs :kind="isCom ? 'com' : 'user'" /></template>
    <template #h5Tabs><MemberComVipTabs :kind="isCom ? 'com' : 'user'" /></template>
    <MemberComScreen v-if="isCom" :tabs="tabs" />
    <div v-else class="site-pc job_list_tit">
      <ul>
        <li v-for="item in tabs" :key="'pc-' + item.value" :class="{ job_list_tit_cur: item.on }">
          <a href="javascript:;" @click.prevent="item.select()">{{ item.label }}</a>
        </li>
      </ul>
    </div>
    <div v-if="!isCom" class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li
            v-for="item in tabs"
            :key="'h5-' + item.value"
            :class="{ m_tabactive: item.on }"
            @click="item.select()"
          >
            {{ item.label }}
          </li>
        </ul>
      </div>
    </div>
    <p v-if="mallOn" class="muted">
      <NuxtLink to="/redeem" class="cblue">{{ $t('wap_00398') }}</NuxtLink>
    </p>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-if="!list.length" class="muted">
      {{ $t('default_00284') }}
      <NuxtLink v-if="mallOn" to="/redeem" class="cblue">{{ $t('wap_00398') }}</NuxtLink>
    </p>
    <div v-if="list.length" class="site-pc paylist_tit">
      <span class="paylist_span paylist_dh">{{ $t('wap_user_00103') }}</span>
      <span class="paylist_span paylist_money">{{ $t('wap_user_00008') }}</span>
      <span class="paylist_span paylist_zt">{{ $t('member_user_00104') }}</span>
    </div>
    <div v-for="row in list" :key="row.id" class="site-pc paylist_list">
      <span class="paylist_span paylist_dh">{{ row.name }} × {{ row.num }}</span>
      <span class="paylist_span paylist_money">{{ row.integral }}</span>
      <span class="paylist_span paylist_zt">{{ row.status_n }} · {{ row.created_at_n }}</span>
      <span class="paylist_span paylist_cz">
        <a v-if="canCancel(row)" href="javascript:;" class="cblue" @click="cancel(row.id)">{{ $t('common.cancel') }}</a>
      </span>
    </div>
    <div class="site-h5 detail_body">
      <div v-if="list.length" class="detail_body_card">
        <ul>
          <li v-for="row in list" :key="'h5-' + row.id">
            <div class="detail_box">
              <div class="detail_box_title">{{ row.name }} × {{ row.num }}</div>
              <div class="detail_box_time">{{ row.status_n }} · {{ row.created_at_n }}</div>
            </div>
            <div class="detail_integral">{{ row.integral }}</div>
            <div class="detail_box_cz">
              <a v-if="canCancel(row)" href="javascript:;" @click="cancel(row.id)">{{ $t('common.cancel') }}</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
