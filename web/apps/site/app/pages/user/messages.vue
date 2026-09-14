<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('msgs', () =>
  api.post('/v1/mcenter/messages', { page: 1, page_size: 20 }),
)
const { data: dash } = await useAsyncData('user-msg-dash', () =>
  api
    .post<{ wkyqnum?: number; commsgnum?: number; sxnum?: number }>('/v1/mcenter/dashboard', {})
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
</script>

<template>
  <MemberPanel :title="$t('common.message')" :error="error" :empty="!error && !(data?.list || []).length">
    <div class="site-pc job_list_tit">
      <ul>
        <li class="job_list_tit_cur">
          <a href="javascript:;">{{ $t('common.message') }}</a>
        </li>
        <li>
          <NuxtLink to="/user/interviews">{{ $t('wap_user_00216') }}<span v-if="dash?.wkyqnum">({{ dash.wkyqnum }})</span></NuxtLink>
        </li>
        <li>
          <NuxtLink to="/user/consults">{{ $t('wap_user_00364') }}<span v-if="dash?.commsgnum">({{ dash.commsgnum }})</span></NuxtLink>
        </li>
      </ul>
    </div>
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li class="m_tabactive">{{ $t('common.message') }}</li>
          <li @click="navigateTo('/user/interviews')">{{ $t('wap_user_00216') }}</li>
          <li @click="navigateTo('/user/consults')">{{ $t('wap_user_00364') }}</li>
        </ul>
      </div>
    </div>
    <p class="user_czbth">
      <a href="javascript:;" class="user_new_yqh_a" @click="readAll">{{ $t('common.confirm') }}</a>
    </p>
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('common.message') }}</div>
      <div class="user_new_time">{{ $t('member_user_00104') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <span class="user_new_jobname">{{ row.body || row.content || row.title || row.id }}</span>
      </div>
      <div class="user_new_time">{{ row.datetime_n }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_a" @click="read(row.id)">{{ $t('common.confirm') }}</a>
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :title="String(row.body || row.content || row.title || row.id)"
          :time="row.datetime_n"
        />
      </div>
    </div>
  </MemberPanel>
</template>
