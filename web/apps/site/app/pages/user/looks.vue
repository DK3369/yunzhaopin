<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `look-jobs-mine-${page.value}`,
  () => api.post('/v1/mcenter/look-jobs/mine', { page: page.value, page_size: pageSize }),
)
const msg = ref('')
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/look-jobs/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function salaryOf(row: { minsalary?: number; maxsalary?: number }) {
  if (row.minsalary || row.maxsalary) return `${row.minsalary || ''} - ${row.maxsalary || ''}`
  return ''
}
useSeoMeta({ title: t('wap_user_00275') })
const total = computed(() => inferTotal(data.value))
</script>

<template>
  <MemberPanel
    :title="$t('wap_user_00275')"
    :sub="$t('member_user_00102')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !(data?.list || []).length"
    empty-to="/jobs"
    :empty-action="$t('wap_user_00254')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li @click="navigateTo('/user/views')">{{ $t('wap_com_00407') }}</li>
          <li class="m_tabactive">{{ $t('wap_user_00275') }}</li>
        </ul>
      </div>
    </div>
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('member_user_00106') }}</div>
      <div class="user_new_zt">{{ $t('member_user_00197') }}</div>
      <div class="user_new_yqh">{{ $t('member_user_00107') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="interview_application_name">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else class="interview_application_name">{{ row.job_name || row.id }}</span>
        <div class="user_new_comname">{{ row.com_name }}</div>
      </div>
      <div class="user_new_time">
        <span class="user_new_xz_n">{{ salaryOf(row) }}</span>
      </div>
      <div class="user_new_zt">{{ row.datetime_n }}</div>
      <div class="user_new_yqh">{{ row.status_n || row.status || '' }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          variant="issue"
          :title="row.job_name || $t('common.job')"
          :pay="salaryOf(row)"
          :sub="row.com_name"
          :time="row.datetime_n"
          :to="row.job_id ? `/jobs/${row.job_id}` : undefined"
          :look-text="$t('wap_user_00275')"
          :on-look-del="() => remove(row.id)"
        />
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
