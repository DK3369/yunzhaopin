<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `look-resumes-${page.value}`,
  () => api.post('/v1/mcenter/look-resumes/list', { page: page.value, page_size: pageSize }),
)
const msg = ref('')
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/look-resumes/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_com_00407') })
const total = computed(() => inferTotal(data.value))
</script>

<template>
  <MemberPanel
    :title="$t('wap_com_00407')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !(data?.list || []).length"
    :empty-text="$t('ui.no_who_viewed')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li class="m_tabactive">{{ $t('wap_com_00407') }}</li>
          <li @click="navigateTo('/user/looks')">{{ $t('wap_user_00275') }}</li>
        </ul>
      </div>
    </div>
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('common.company') }}</div>
      <div class="user_new_job">{{ $t('wap_01536') }}</div>
      <div class="user_new_zt">{{ $t('member_user_00197') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`" class="user_new_jobname">{{ row.com_name || row.com_id }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.com_name || row.id }}</span>
        <div class="look_myresume_comxz">{{ row.com_pr || row.pr_n }} <span v-if="row.com_mun || row.mun_n" class="look_myresume_comline">|</span> {{ row.com_mun || row.mun_n }}</div>
      </div>
      <div class="user_new_job">
        <div v-if="row.com_job" class="user_new_joball">{{ row.com_job }}</div>
        <span v-if="row.com_job_num" class="user_new_joball_n">{{ row.com_job_num }}</span>
        <NuxtLink v-if="row.com_name" to="/jobs" class="user_new_joball_more">{{ $t('common.more') }}</NuxtLink>
      </div>
      <div class="user_new_zt">{{ row.datetime_n }}</div>
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
          :title="row.com_name || String(row.com_id || row.id)"
          :sub="row.com_job"
          :time="row.datetime_n"
          :to="row.com_id ? `/companies/${row.com_id}` : undefined"
          :tags="[row.com_pr || row.pr_n, row.com_mun || row.mun_n].filter((x): x is string => Boolean(x))"
          :look-job="row.com_job"
          :look-text="$t('wap_user_00276')"
          :on-look-del="() => remove(row.id)"
        />
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
