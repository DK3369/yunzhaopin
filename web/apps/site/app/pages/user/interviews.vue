<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `yqms-${page.value}`,
  () => api.post('/v1/mcenter/yqms/list', { page: page.value, page_size: pageSize }),
)
const openId = ref(0)
const rejectId = ref(0)
const remark = ref('')
const msg = ref('')
async function accept(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/accept', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function reject(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/reject', { id, remark: remark.value })
    rejectId.value = 0
    remark.value = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function shield(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist', { yqms_id: id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function browseLabel(state?: number) {
  if (state === 3) return t('wap_com_00190')
  if (state === 4) return t('wap_user_00257')
  if (state === 2) return t('wap_user_00258')
  return t('wap_user_00260')
}
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('wap_user_00216') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_user_00216')"
    :sub="$t('member_user_00102')"
    :error="error"
    :empty="!error && !(data?.list || []).length"
    empty-to="/jobs"
    :empty-action="$t('wap_user_00254')"
  >
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('common.job') }}</div>
      <div class="user_new_time">{{ $t('wap_00040') }}</div>
      <div class="user_new_zt">{{ $t('member_user_00104') }}</div>
      <div class="user_new_yqh">{{ $t('member_user_00107') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="user_new_jobname">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.job_name || row.title || row.id }}</span>
        <div v-if="row.salary" class="user_new_jobxz">{{ row.salary }}</div>
        <div class="user_new_comname">
          <NuxtLink v-if="row.fid" :to="`/companies/${row.fid}`">{{ row.fname }}</NuxtLink>
        </div>
      </div>
      <div class="user_new_time">
        <span class="msg_zt_s">{{ row.datetime_n }}</span>
      </div>
      <div class="user_new_zt">
        <template v-if="row.is_browse === 3">{{ $t('wap_com_00190') }}</template>
        <template v-else-if="row.is_browse === 4">
          <span class="msg_zt_s">{{ $t('wap_user_00257') }}</span>
        </template>
        <template v-else-if="row.is_browse !== 3 && row.is_browse !== 4">
          <a href="javascript:;" class="user_new_bth" @click="accept(row.id)">{{ $t('wap_user_00262') }}</a>
          <a href="javascript:;" class="user_new_btjh" @click="rejectId = row.id">{{ $t('wap_01053') }}</a>
        </template>
      </div>
      <div class="user_new_yqh">
        <a href="javascript:;" class="user_new_yqh_a" @click="openId = openId === row.id ? 0 : row.id">{{ $t('member_user_00107') }}</a>
      </div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
      <p v-if="openId === row.id" class="invitation_cont">
        <div class="invitation_cont_p"><span class="invitation_cont_pn">{{ $t('wap_user_00255') }}</span><em class="audition_list_e">{{ row.intertime }}</em></div>
        <div class="invitation_cont_p"><span class="invitation_cont_pn">{{ $t('wap_user_00243') }}</span><em class="audition_list_e">{{ row.address }}</em></div>
        <div class="invitation_cont_p"><span class="invitation_cont_pn">{{ $t('common_02051') }}</span><em class="audition_list_e">{{ row.linkman }}</em> TEL：<em class="invitation_cont_tel">{{ row.linktel }}</em></div>
        <div v-if="row.content" class="audition_list"><span class="audition_list_span">{{ row.content }}</span></div>
      </p>
      <form v-if="rejectId === row.id" class="form verification_form" @submit.prevent="reject(row.id)">
        <MemberField :label="$t('wap_01053')">
          <input v-model="remark" />
        </MemberField>
        <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
      </form>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <div v-for="row in data?.list || []" :key="'h5-' + row.id" class="Posted_body_card">
          <div class="Posted_card_top" @click="openId = openId === row.id ? 0 : row.id">
            <div class="Posted_card_name">
              <i v-if="row.is_browse === 1" class="m_ms_new">NEW</i>
              {{ row.fname || row.job_name }}
            </div>
            <div class="Posted_card_pay">{{ browseLabel(row.is_browse) }}</div>
          </div>
          <div class="interview_time_box">
            <div class="card_interview_text">{{ $t('wap_user_00255') }}</div>
            <div class="card_interview_time">{{ row.intertime }}</div>
          </div>
          <div class="interview_job_box">
            <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="interview_box_post">
              <div class="card_interview_text">{{ $t('wap_user_00256') }}</div>
              <div class="card_interview_time">{{ row.job_name }}</div>
            </NuxtLink>
            <div class="interview_box_icons">
              <div v-if="row.is_browse !== 3 && row.is_browse !== 4" class="interview_box_icon" @click="accept(row.id)">{{ $t('wap_user_00262') }}</div>
              <div v-if="row.is_browse !== 3 && row.is_browse !== 4" class="interview_box_icon" @click="rejectId = row.id">{{ $t('wap_01053') }}</div>
              <div class="interview_box_icon" @click="remove(row.id)">
                <img src="/legacy/h5/images/resume_del.png" alt="" width="100%" height="100%" />
              </div>
            </div>
          </div>
          <p v-if="openId === row.id" class="interview_job_box">
            <div class="card_interview_text">{{ row.address }} {{ row.linkman }} {{ row.linktel }}</div>
            <div v-if="row.content" class="card_interview_time">{{ row.content }}</div>
          </p>
        </div>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
