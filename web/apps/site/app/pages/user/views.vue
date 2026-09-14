<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('look-resumes', () =>
  api.post('/v1/mcenter/look-resumes/list', { page: 1, page_size: 20 }),
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
      <div class="user_new_time">{{ $t('member_user_00104') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`" class="user_new_jobname">{{ row.com_name || row.com_id }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.com_name || row.id }}</span>
        <div class="user_new_comname">{{ row.com_job }} <template v-if="row.com_job_num">· {{ row.com_job_num }}</template></div>
      </div>
      <div class="user_new_time">{{ row.datetime_n }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :title="row.com_name || String(row.com_id || row.id)"
          :sub="row.com_job"
          :time="row.datetime_n"
          :to="row.com_id ? `/companies/${row.com_id}` : undefined"
        />
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
