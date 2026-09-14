<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `job-msg-mine-${page.value}`,
  () => api.post('/v1/mcenter/job-messages/mine', { page: page.value, page_size: pageSize }),
)
const msg = ref('')
function statusLabel(status?: number) {
  if (status === 2) return t('wap_user_00167')
  if (status === 1) return t('wap_user_00165')
  return t('wap_user_00166')
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/job-messages/hide', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('member_user_00115') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00115')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="resume_Prompt_box">
      <div class="resume_Prompt"><i class="resume_Prompt_icon" />{{ $t('wap_com_00408') }}</div>
    </div>
    <div v-if="(data?.list || []).length" class="job_Consulting_tit site-pc">
      <span class="job_Consulting_span job_Consulting_jobname">{{ $t('wap_com_00288') }}</span>
      <span class="job_Consulting_span job_Consulting_comname">{{ $t('wap_com_00157') }}</span>
      <span class="job_Consulting_span job_Consulting_jobtime">{{ $t('member_user_00061') }}</span>
      <span class="job_Consulting_span job_Consulting_jobtime">{{ $t('wap_com_00406') }}</span>
      <span class="job_Consulting_span job_Consulting_jobcz">{{ $t('member_user_00048') }}</span>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="job_Consulting_list site-pc">
      <div class="job_Consulting_span job_Consulting_jobname">
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="job_Consulting_jobname_a">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else>{{ row.job_name || $t('common_02082') }}</span>
      </div>
      <div class="job_Consulting_span job_Consulting_comname">
        <NuxtLink v-if="row.job_uid" :to="`/companies/${row.job_uid}`">{{ row.com_name }}</NuxtLink>
      </div>
      <div class="job_Consulting_span job_Consulting_jobtime">{{ row.datetime_n }}</div>
      <div class="job_Consulting_span job_Consulting_jobtime">
        {{ statusLabel(row.status) }}
        <span v-if="row.statusbody" class="com_m_line">|</span>
        <span v-if="row.statusbody">{{ row.statusbody }}</span>
      </div>
      <div class="job_Consulting_span job_Consulting_jobcz">
        <a href="javascript:;" class="List_dete cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
      <div class="job_Consulting_box">
        <i class="List_Title_span_zx_icon" />
        <div class="job_Consulting_my">
          <div class="job_Consulting_my_box">
            <span class="job_Consulting_my_ask">{{ $t('member_user_00480') }}</span>{{ row.content }}
          </div>
        </div>
        <div v-if="row.reply" class="job_Consulting_my">
          <div class="job_Consulting_com">
            <i class="job_Consulting_icon" />
            <div>
              {{ $t('wap_user_00155') }}：{{ row.reply }}
              <div v-if="row.reply_time_n || row.reply_time" class="job_Consulting_hftime">{{ $t('admin_user_00369') }}：{{ row.reply_time_n || row.reply_time }}</div>
            </div>
          </div>
        </div>
        <div v-else class="job_Consulting_my">
          <span class="job_Consulting_zt"><i class="job_Consulting_zticon" />{{ $t('member_user_00481') }}</span>
        </div>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <div v-for="row in data?.list || []" :key="'h5-' + row.id" class="m_cardbg">
          <div class="com_member_hr_name">
            <NuxtLink v-if="row.job_uid" :to="`/companies/${row.job_uid}`">{{ row.com_name }}</NuxtLink>
            <span v-else>{{ row.com_name }}</span>
            <div class="wap_member_date_r">{{ statusLabel(row.status) }}</div>
          </div>
          <div class="mag_show">
            <div v-if="row.job_name" class="com_member_hr_p1">
              <span class="member_c9">{{ $t('wap_user_00163') }}</span>{{ row.job_name }}
            </div>
            <div class="com_member_hr_p1">
              <span class="member_c9">{{ $t('wap_user_00162') }}</span>{{ row.content }}
            </div>
            <div v-if="row.reply" class="com_member_hr_p1">
              <span class="member_c9">{{ $t('wap_user_00155') }}</span>{{ row.reply }}
            </div>
            <div v-else class="com_member_hr_p1">
              <span class="member_c9">{{ $t('wap_user_00155') }}</span>
              <font color="red">{{ $t('wap_01138') }}</font>
            </div>
            <div class="com_member_hr_p1" style="position: relative">
              <span class="member_c9">{{ $t('wap_01137') }}</span>{{ row.datetime_n }}
              <div class="sx_new_icon">
                <a href="javascript:;" @click="remove(row.id)">
                  <img src="/legacy/h5/images/resume_del.png" alt="" />
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
