<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-msgs-${page.value}`,
  () => api.post('/v1/mcenter/messages', { page: page.value, page_size: pageSize }),
)
const { data: dash } = await useAsyncData('com-msg-dash', () =>
  api
    .post<{
      applies_unread?: number
      job_msg_unanswered?: number
      unread_messages?: number
    }>('/v1/mcenter/com-dashboard', {})
    .catch(() => null),
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
    <p class="user_czbth">
      <a href="javascript:;" class="user_new_yqh_a" @click="readAll">{{ $t('common.confirm') }}</a>
    </p>
    <div v-if="(data?.list || []).length" class="sysynews_tit site-pc">
      <div class="sysynews_span sysynews_name">{{ $t('common.message') }}</div>
      <div class="sysynews_span sysynews_time">{{ $t('member_user_00104') }}</div>
      <div class="sysynews_span sysynews_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="sysynews_list site-pc">
      <div class="sysynews_span sysynews_name">{{ row.body || row.content || row.title || row.id }}</div>
      <div class="sysynews_span sysynews_time">{{ row.datetime_n }}</div>
      <div class="sysynews_span sysynews_cz">
        <a href="javascript:;" class="cblue" @click="read(row.id)">{{ $t('common.confirm') }}</a>
        <a href="javascript:;" class="List_dete cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <MemberSxNewsCard
        v-for="row in data?.list || []"
        :key="'h5-' + row.id"
        :title="String(row.body || row.content || row.title || row.id)"
        :time="row.datetime_n"
        :on-delete="() => remove(row.id)"
      />
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
