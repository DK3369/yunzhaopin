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
async function read(id: number) {
  await api.post('/v1/mcenter/messages/read', { id })
  refresh()
}
async function remove(id: number) {
  await api.post('/v1/mcenter/messages/delete', { id })
  refresh()
}
async function readAll() {
  await api.post('/v1/mcenter/messages/read-all', {})
  refresh()
}
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
      </ul>
    </div>
    <p class="site-pc">
      <a href="javascript:;" class="com_bth cblue" @click="readAll">{{ $t('common.confirm') }}</a>
    </p>
    <table v-if="(data?.list || []).length" class="com_table mt20 site-pc">
      <tr>
        <th>{{ $t('common.message') }}</th>
        <th>{{ $t('member_user_00104') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in data?.list || []" :key="row.id">
        <td>{{ row.body || row.content || row.title || row.id }}</td>
        <td>{{ row.datetime_n }}</td>
        <td>
          <a href="javascript:;" class="com_bth cblue" @click="read(row.id)">{{ $t('common.confirm') }}</a>
          <a href="javascript:;" class="com_bth cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
        </td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in data?.list || []" :key="'h5-' + row.id" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.body || row.content || row.title || row.id }}</div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('member_user_00104') }}</span>
          {{ row.datetime_n }}
        </div>
        <div class="com_card_cz">
          <a href="javascript:;" class="com_bth cblue" @click="read(row.id)">{{ $t('common.confirm') }}</a>
          <span class="com_card_delete" @click="remove(row.id)" />
        </div>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
