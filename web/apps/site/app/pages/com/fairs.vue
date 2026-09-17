<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = {
  id: number
  zid: number
  title?: string
  name?: string
  address?: string
  datetime_n?: string
  start_at_n?: string
  end_at_n?: string
  status: number
  statusbody?: string
  booth_name?: string
  job_names?: string
  notstart?: number
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-zph', () =>
  api.post<{ list: Row[]; total: number }>('/v1/mcenter/zph/my-reservation', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])
const msg = ref('')
const detail = ref<Row | null>(null)
const statusTip = ref('')

function statusText(status: number) {
  if (status === 1) return t('wap_user_00165')
  if (status === 2) return t('wap_user_00167')
  return t('wap_user_00166')
}

function showDetail(row: Row) {
  detail.value = row
  statusTip.value = ''
}

function showStatusBody(row: Row) {
  statusTip.value = row.statusbody?.trim() || t('wap_com_00409')
  detail.value = null
}

async function cancel(id: number) {
  if (!window.confirm(t('wap_01213'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/zph/cancel', { id })
    msg.value = t('common.success')
    detail.value = null
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_00558') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_00558')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !list.length"
    empty-to="/fairs"
    :empty-action="$t('member_com_00670')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <table v-if="list.length" class="com_table site-pc">
      <tr>
        <th>{{ $t('member_com_00293') }}</th>
        <th>{{ $t('member_com_00378') }}</th>
        <th>{{ $t('wap_user_00304') }}</th>
        <th>{{ $t('member_user_00181') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in list" :key="row.id">
        <td>
          <NuxtLink v-if="row.zid" :to="`/fairs/${row.zid}`" class="cblue" target="_blank">{{ row.title || row.name || row.zid }}</NuxtLink>
          <span v-else>{{ row.title || row.name || row.id }}</span>
        </td>
        <td>{{ row.address }}</td>
        <td>{{ row.datetime_n }}</td>
        <td>
          <span :class="{ wap_member_wtg: row.status === 2 }">{{ statusText(row.status) }}</span>
          <a v-if="row.status === 2" href="javascript:;" class="cblue" @click="showStatusBody(row)">{{ $t('wap_user_00164') }}</a>
        </td>
        <td>
          <a href="javascript:;" class="com_bth cblue" @click="showDetail(row)">{{ $t('ui.detail') }}</a>
          <a v-if="row.notstart === 1" href="javascript:;" class="com_bth cblue" @click="cancel(row.id)">{{ $t('common.cancel') }}</a>
        </td>
      </tr>
    </table>
    <div class="site-h5">
      <div
        v-for="row in list"
        :key="'h5-' + row.id"
        class="com_cardlist"
        @click="row.zid ? navigateTo(`/fairs/${row.zid}`) : undefined"
      >
        <div class="com_cardlist_tit">{{ row.title || row.name || row.zid || row.id }}</div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_com_00425') }}</span>
          {{ row.booth_name }}
        </div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_com_00424') }}</span>
          {{ row.address }}
        </div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_user_00087') }}</span>
          {{ row.start_at_n }}
        </div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_user_00096') }}</span>
          {{ row.end_at_n }}
        </div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_com_00406') }}</span>
          <span :class="{ wap_member_wtg: row.status === 2 }">{{ statusText(row.status) }}</span>
        </div>
        <div v-if="row.status === 2 && row.statusbody" class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_user_00164') }}</span>
          {{ row.statusbody }}
        </div>
        <div v-if="row.notstart === 1" class="com_card_cz">
          <span class="com_card_delete" @click.stop="cancel(row.id)" />
        </div>
      </div>
    </div>
    <div v-if="detail" class="com_release_box mt20">
      <ul>
        <li>{{ $t('member_com_00293') }}：{{ detail.title || detail.name }}</li>
        <li>{{ $t('wap_js_00088') }}：{{ detail.start_at_n }} ~ {{ detail.end_at_n }}</li>
        <li>{{ $t('wap_com_00425') }}：{{ detail.booth_name }}</li>
        <li>{{ $t('wap_com_00424') }}：{{ detail.address }}</li>
        <li v-if="detail.job_names">{{ $t('wap_com_00288') }}：{{ detail.job_names }}</li>
      </ul>
      <p>
        <a href="javascript:;" class="com_bth cblue" @click="detail = null">{{ $t('common.close') }}</a>
        <NuxtLink v-if="detail.zid" :to="`/fairs/${detail.zid}`" class="com_bth cblue">{{ $t('ui.detail') }}</NuxtLink>
      </p>
    </div>
    <p v-if="statusTip" class="mt10">{{ statusTip }}</p>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
