<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = {
  id: number
  name: string
  integral: number
  num: number
  status: number
  status_n: string
  created_at_n: string
}

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const msg = ref('')
const { data, error, refresh } = await useAsyncData(
  () => `redeem-orders-${page.value}`,
  () => api.post<{ list: Row[]; total: number }>('/v1/mcenter/redeem/orders', { page: page.value, page_size: pageSize }),
)
const list = computed(() => data.value?.list || [])
const total = computed(() => inferTotal(data.value))

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
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-if="!list.length" class="muted">{{ $t('default_00284') }}</p>
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
        <a v-if="row.status === 0" href="javascript:;" class="cblue" @click="cancel(row.id)">{{ $t('common.cancel') }}</a>
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
              <a v-if="row.status === 0" href="javascript:;" @click="cancel(row.id)">{{ $t('common.cancel') }}</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
