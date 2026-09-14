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
    <div class="resume_Prompt_box">
      <div class="resume_Prompt"><i class="resume_Prompt_icon" />{{ $t('member_user_00191') }}</div>
    </div>
    <div v-if="list.length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_time">{{ $t('wap_00925') }}</div>
      <div class="user_new_zt">{{ $t('member_user_00194') }}</div>
      <div class="user_new_yqh">{{ $t('member_user_00196') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in list" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink :to="`/jobs/${row.id}`" class="user_new_jobname ppjobname">{{ row.name }}</NuxtLink>
        <div class="user_new_comname">
          <NuxtLink v-if="row.uid" :to="`/companies/${row.uid}`">{{ row.com_name }}</NuxtLink>
        </div>
      </div>
      <div class="user_new_time">
        <span class="user_new_xz_n">{{ row.min_salary }} - {{ row.max_salary }}</span>
      </div>
      <div class="user_new_zt">
        <span>{{ row.edu_n || row.job_edu }}</span>
        <span class="look_myresume_comline">|</span>
        <span>{{ row.exp_n || row.job_exp }}</span>
      </div>
      <div class="user_new_yqh ppxz">{{ row.pre || row.match || '' }}%</div>
      <div class="user_new_cz">
        <NuxtLink :to="`/jobs/${row.id}`" class="user_new_yqh_a">{{ $t('wap_com_00235') }}</NuxtLink>
      </div>
    </div>
    <div class="site-h5 main_member_cot_box">
      <div
        v-for="row in list"
        :key="'h5-' + row.id"
        class="com_member_hr"
        @click="navigateTo(`/jobs/${row.id}`)"
      >
        <div class="com_member_hr_name">{{ row.name }}</div>
        <div class="user_member_box">
          <div class="com_member_company">{{ row.com_name }}</div>
          <div class="com_member_particulars">
            <div>{{ row.min_salary }}-{{ row.max_salary }}</div>
            <div v-if="row.edu_n || row.job_edu"> | {{ row.edu_n || row.job_edu }}{{ $t('home.education_suffix') }}</div>
            <div v-if="row.exp_n || row.job_exp"> | {{ row.exp_n || row.job_exp }}{{ $t('home.experience_suffix') }}</div>
          </div>
          <div class="com_member_matched_degree">{{ $t('wap_user_00273') }}{{ row.pre || row.match || '' }}%</div>
        </div>
      </div>
    </div>
  </MemberPanel>
</template>
