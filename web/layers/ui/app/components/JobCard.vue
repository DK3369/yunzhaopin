<template>
  <!-- 首页楼层卡 -->
  <li v-if="variant === 'home'" class="site-pc">
    <div class="index_newjobname">
      <NuxtLink :to="`/jobs/${job.id}`" :title="job.name">{{ job.name }}</NuxtLink>
      <span class="index_newjob_info_xz">{{ salary }}</span>
    </div>
    <div class="index_newjob_info nowrap">
      {{ city }}
      <template v-if="job.exp_n">
        <i class="index_newjob_info_line">|</i>{{ job.exp_n }}
      </template>
      <template v-if="job.edu_n">
        <i class="index_newjob_info_line">|</i>{{ job.edu_n }}
      </template>
    </div>
    <div class="index_newjob_com nowrap">
      <img :src="logo" class="index_newjob_com_tx" alt="" />
      <div class="index_newjob_comname">
        <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`">{{ job.com_name }}</NuxtLink>
        <span v-else>{{ job.com_name }}</span>
      </div>
      <div class="index_newjob_cominfo">{{ job.job_hy || job.hy_n || '' }}</div>
    </div>
  </li>
  <NuxtLink v-if="variant === 'home'" class="site-h5" :to="`/jobs/${job.id}`" :title="job.name">
    <div class="table-card">
      <div class="card_post">
        <i class="table-card-word">{{ job.name }}</i>
        <i class="table-card-salary">{{ salary }}</i>
      </div>
      <div class="table-card-require">
        <i class="requir-area">{{ city }}</i>
        <i v-if="job.edu_n" class="requir_area_parting_line" />
        <i v-if="job.edu_n" class="requir-area">{{ job.edu_n }}</i>
        <i v-if="job.exp_n" class="requir_area_parting_line" />
        <i v-if="job.exp_n" class="requir-area">{{ job.exp_n }}</i>
      </div>
      <div class="index_company">
        <i class="index_company-logo">
          <img :src="logo" alt="" style="width: 100%" />
        </i>
        <i class="index_company-name">{{ job.com_name }}</i>
      </div>
    </div>
  </NuxtLink>

  <!-- PC 职位搜索卡 -->
  <div v-if="variant === 'search'" class="search_job_list site-pc">
    <div class="yunjoblist_new">
      <div class="yunjoblist_newname">
        <NuxtLink :to="`/jobs/${job.id}`" class="yunjoblist_newname_a" :title="job.name">{{ job.name }}</NuxtLink>
        <i v-if="job.newtime" class="job_newicon">new</i>
      </div>
      <div class="yunjoblist_newcomename">
        <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`" class="search_job_com_name">{{ job.com_name }}</NuxtLink>
        <span v-else class="search_job_com_name">{{ job.com_name }}</span>
      </div>
    </div>
    <div class="jobshow">
      <div class="search_job_left_siaber">
        <div class="company_det">
          <span class="search_job_l_xz">{{ salary }}</span>
          <span v-if="city" class="search_job_list_box_line">|</span>
          <span v-if="city" class="search_job_list_box_s">{{ city }}</span>
          <span v-if="job.exp_n" class="search_job_list_box_line">|</span>
          <span v-if="job.exp_n" class="search_job_list_box_s">{{ job.exp_n }}</span>
          <span v-if="job.edu_n" class="search_job_list_box_line">|</span>
          <span v-if="job.edu_n" class="search_job_list_box_s">{{ job.edu_n }}</span>
        </div>
      </div>
    </div>
  </div>

  <!-- H5 职位列表 tab_card -->
  <NuxtLink v-if="variant !== 'home'" class="site-h5" :to="`/jobs/${job.id}`" :title="job.name">
    <div class="tab_card">
      <div class="tab_card_top">
        <div class="tab_card_job">
          <i class="tab_card_job_name">{{ job.name }}</i>
          <i v-if="job.newtime" class="tab_card_new">new</i>
        </div>
        <i class="tab_card_pay">{{ salary }}</i>
      </div>
      <div class="newjob_info">
        <span>{{ city }}</span>
        <template v-if="job.exp_n">
          <i class="newjob_info_line" />
          <span>{{ job.exp_n }}</span>
        </template>
        <template v-if="job.edu_n">
          <i class="newjob_info_line" />
          <span>{{ job.edu_n }}</span>
        </template>
      </div>
      <div class="tab_card_bottom">
        <div class="card_bottom_logo">
          <img :src="logo" alt="" style="width: 100%" />
        </div>
        <i class="card_bottom_word">{{ job.com_name }}</i>
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { formatSalary, mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '../utils/site'

const props = withDefaults(
  defineProps<{ job: JobLike; variant?: 'home' | 'search' | 'tab' }>(),
  { variant: 'home' },
)
const { t } = useI18n()
const salary = computed(() => formatSalary(props.job, t('common.negotiable')))
const city = computed(
  () => props.job.job_city_two || props.job.city_two || props.job.job_city_one || '',
)
const logo = computed(() => mediaUrl(props.job.com_logo || props.job.logo, PLACEHOLDER_LOGO))
</script>
