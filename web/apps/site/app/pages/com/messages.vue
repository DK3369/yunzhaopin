<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-msgs-${page.value}`,
  () => api.post('/v1/mcenter/messages', { page: page.value, page_size: pageSize }),
)
const { data: dash } = await useAsyncData(
  'com-dash',
  () =>
    api
      .post<{
        applies_unread?: number
        job_msg_unanswered?: number
        unread_messages?: number
      }>('/v1/mcenter/com-dashboard/full', {})
      .catch(() => null),
  reuseAsyncCache(),
)
const { data: unread } = await useAsyncData(
  'mcenter-unread-summary',
  () =>
    api
      .post<{ broadcasts?: number; warnings?: number }>('/v1/mcenter/messages/unread-summary', {})
      .catch(() => null),
)
const picked = ref<number[]>([])
const list = computed(() => (data.value?.list || []) as Array<Record<string, unknown>>)
const allPicked = computed({
  get: () => list.value.length > 0 && picked.value.length === list.value.length,
  set: (v: boolean) => {
    picked.value = v ? list.value.map((r) => Number(r.id)) : []
  },
})
function isUnread(row: Record<string, unknown>) {
  if (row.is_read === false) return true
  if (row.is_read === true) return false
  return Number(row.remind_status) === 1
}
function rowTime(row: Record<string, unknown>) {
  return String(row.created_at_n || row.datetime_n || '')
}
function rowBody(row: Record<string, unknown>) {
  return String(row.body || row.content || row.title || row.id || '')
}
function rowParts(row: Record<string, unknown>) {
  const parts = row.parts
  if (Array.isArray(parts) && parts.length) {
    return parts as Array<{ n?: string; to?: string }>
  }
  return [{ n: rowBody(row) }]
}
function togglePick(id: number) {
  picked.value = picked.value.includes(id) ? picked.value.filter((x) => x !== id) : [...picked.value, id]
}
async function read(id: number) {
  await api.post('/v1/mcenter/messages/read', { id })
  refresh()
}
async function remove(id: number) {
  await api.post('/v1/mcenter/messages/delete', { id })
  picked.value = picked.value.filter((x) => x !== id)
  refresh()
}
async function readAll() {
  await api.post('/v1/mcenter/messages/read-all', {})
  refresh()
}
async function removePicked() {
  if (!picked.value.length) return
  await api.post('/v1/mcenter/messages/delete', { ids: picked.value })
  picked.value = []
  refresh()
}
async function readPicked() {
  for (const id of picked.value) await api.post('/v1/mcenter/messages/read', { id })
  picked.value = []
  refresh()
}
watch(page, () => {
  picked.value = []
})
useSeoMeta({ title: t('common.message') })
const total = computed(() => inferTotal(data.value))
</script>

<template>
  <MemberPanel :title="$t('common.message')" :error="error" :empty="false">
    <div class="site-pc job_list_tit">
      <ul>
        <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('common.message') }}</a></li>
        <li>
          <NuxtLink to="/com/applications">{{ $t('wap_00794') }}<span v-if="dash?.applies_unread">({{ dash.applies_unread }})</span></NuxtLink>
        </li>
        <li>
          <NuxtLink to="/com/job-messages">{{ $t('wap_com_00408') }}<span v-if="dash?.job_msg_unanswered">({{ dash.job_msg_unanswered }})</span></NuxtLink>
        </li>
        <li>
          <NuxtLink to="/com/broadcasts">{{ $t('ui.broadcasts') }}<span v-if="unread?.broadcasts">({{ unread.broadcasts }})</span></NuxtLink>
        </li>
        <li>
          <NuxtLink to="/com/warnings">{{ $t('ui.warnings') }}<span v-if="unread?.warnings">({{ unread.warnings }})</span></NuxtLink>
        </li>
      </ul>
    </div>
    <div class="m_taball category site-h5">
      <div class="m_taballbox">
        <ul>
          <li class="m_taballactive">{{ $t('common.message') }}</li>
          <li @click="navigateTo('/com/broadcasts')">
            {{ $t('ui.broadcasts') }}
            <span v-if="unread?.broadcasts" class="zp_num">{{ unread.broadcasts }}</span>
          </li>
          <li @click="navigateTo('/com/warnings')">
            {{ $t('ui.warnings') }}
            <span v-if="unread?.warnings" class="zp_num">{{ unread.warnings }}</span>
          </li>
        </ul>
      </div>
    </div>
    <p class="site-pc">
      <a href="javascript:;" class="com_bth cblue" @click="readAll">{{ $t('member_user_00463') }}</a>
    </p>
    <table v-if="list.length" class="com_table mt20 site-pc">
      <tr>
        <th width="25">
          <label><input v-model="allPicked" type="checkbox" class="com_job_list_check" /></label>
        </th>
        <th>{{ $t('common.message') }}</th>
        <th>{{ $t('member_user_00104') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in list" :key="Number(row.id)">
        <td align="center">
          <input type="checkbox" class="com_job_list_check" :checked="picked.includes(Number(row.id))" @change="togglePick(Number(row.id))" />
        </td>
        <td :style="isUnread(row) ? 'font-weight:bold' : ''">
          <template v-for="(p, i) in rowParts(row)" :key="'pc-' + Number(row.id) + '-' + i"><NuxtLink v-if="p.to" :to="p.to" class="sys_a">{{ p.n }}</NuxtLink><span v-else>{{ p.n }}</span></template>
        </td>
        <td>{{ rowTime(row) }}</td>
        <td>
          <a href="javascript:;" class="com_bth cblue" @click="read(Number(row.id))">{{ $t('member_user_00462') }}</a>
          <a href="javascript:;" class="com_bth cblue" @click="remove(Number(row.id))">{{ $t('common.delete') }}</a>
        </td>
      </tr>
    </table>
    <div v-if="list.length" class="com_Release_job_bot site-pc">
      <label class="com_Release_job_qx"><input v-model="allPicked" type="checkbox" class="com_job_list_check" /> {{ $t('common.all') }}</label>
      <a href="javascript:;" class="c_btn_02" @click="removePicked">{{ $t('common.delete') }}</a>
      <a href="javascript:;" class="c_btn_02" @click="readPicked">{{ $t('member_user_00462') }}</a>
      <a href="javascript:;" class="c_btn_02" @click="readAll">{{ $t('member_user_00463') }}</a>
    </div>
    <div class="site-h5">
      <div v-for="row in list" :key="'h5-' + Number(row.id)" class="com_cardlist">
        <label>
          <input type="checkbox" :checked="picked.includes(Number(row.id))" @change="togglePick(Number(row.id))" />
        </label>
        <div class="com_cardlist_tit" :style="isUnread(row) ? 'font-weight:bold' : ''">
          <template v-for="(p, i) in rowParts(row)" :key="'h5-' + Number(row.id) + '-' + i"><NuxtLink v-if="p.to" :to="p.to" class="sys_a">{{ p.n }}</NuxtLink><span v-else>{{ p.n }}</span></template>
        </div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('member_user_00104') }}</span>
          {{ rowTime(row) }}
        </div>
        <div class="com_card_cz">
          <a href="javascript:;" class="com_bth cblue" @click="read(Number(row.id))">{{ $t('member_user_00462') }}</a>
          <span class="com_card_delete" @click="remove(Number(row.id))" />
        </div>
      </div>
      <div v-if="list.length" class="com_Release_job_bot">
        <a href="javascript:;" class="c_btn_02" @click="removePicked">{{ $t('common.delete') }}</a>
        <a href="javascript:;" class="c_btn_02" @click="readPicked">{{ $t('member_user_00462') }}</a>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
