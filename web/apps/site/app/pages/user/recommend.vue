<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('recommend-jobs', () =>
  api.post('/v1/mcenter/recommend/jobs', { limit: 20 }).catch(() => []),
)
const list = computed(() => (Array.isArray(data.value) ? data.value : data.value?.list || data.value || []))
useSeoMeta({ title: t('wap_user_00211') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_user_00211')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !list.length"
    empty-to="/jobs"
    :empty-action="$t('wap_user_00254')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div v-if="list.length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('wap_00925') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in list" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink :to="`/jobs/${row.id}`" class="user_new_jobname">{{ row.name }}</NuxtLink>
        <div class="user_new_comname">
          <NuxtLink v-if="row.uid" :to="`/companies/${row.uid}`">{{ row.com_name }}</NuxtLink>
        </div>
      </div>
      <div class="user_new_time">
        <span class="user_new_xz_n">{{ row.min_salary }} - {{ row.max_salary }}</span>
      </div>
      <div class="user_new_cz">
        <NuxtLink :to="`/jobs/${row.id}`" class="user_new_yqh_a">{{ $t('common.more') }}</NuxtLink>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in list"
          :key="'h5-' + row.id"
          :title="row.name"
          :pay="`${row.min_salary || ''} - ${row.max_salary || ''}`"
          :sub="row.com_name"
          :to="`/jobs/${row.id}`"
        />
      </div>
    </div>
  </MemberPanel>
</template>
