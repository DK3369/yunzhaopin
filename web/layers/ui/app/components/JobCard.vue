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

  <!-- PC 职位搜索卡，对齐 default/job/search.htm -->
  <div v-if="variant === 'search'" class="search_job_list site-pc">
    <div class="yunjoblist_new">
      <div class="yunjoblist_newname">
        <NuxtLink :to="`/jobs/${job.id}`" class="yunjoblist_newname_a" :title="job.name">{{ job.name }}</NuxtLink>
        <i v-if="job.newtime" class="job_newicon">new</i>
        <img v-if="job.is_urgent" src="/legacy/pc/images/jobjp.png" alt="" class="co_zzjp png" />
        <img v-if="job.is_rec" src="/legacy/pc/images/jobtj.png" alt="" class="co_zzjp png" />
      </div>
      <div class="yunjoblist_newcomename">
        <NuxtLink v-if="job.uid" :to="`/companies/${job.uid}`" class="search_job_com_name">{{ job.com_name }}</NuxtLink>
        <span v-else class="search_job_com_name">{{ job.com_name }}</span>
        <i v-if="Number(job.yyzz_status) === 1" class="job_qy_rz_icon" />
      </div>
    </div>
    <div class="jobshow">
      <div class="search_job_left_siaber">
        <div class="company_det">
          <span class="search_job_l_xz">{{ salary }}</span>
          <span v-if="city" class="search_job_list_box_line">|</span>
          <span v-if="city" class="search_job_list_box_s">
            <em class="com_search_job_em">{{ city }}</em>
          </span>
          <template v-if="job.exp_n">
            <span class="search_job_list_box_line">|</span>
            <span class="search_job_list_box_s">
              <em class="com_search_job_em">{{ job.exp_n }}{{ $t('home.experience_suffix') }}</em>
            </span>
          </template>
          <template v-if="job.edu_n">
            <span class="search_job_list_box_line">|</span>
            <span class="search_job_list_box_s">
              <em class="com_search_job_em">{{ job.edu_n }}{{ $t('home.education_suffix') }}</em>
            </span>
          </template>
        </div>
      </div>
      <div class="company_det_c_name">
        <div class="company_det_hy">
          <template v-if="job.job_hy || job.hy_n">{{ job.job_hy || job.hy_n }}</template>
          <i v-if="job.pr_n" class="company_det_hy_line">|</i>{{ job.pr_n }}
          <i v-if="job.mun_n" class="company_det_hy_line">|</i>{{ job.mun_n }}
        </div>
      </div>
      <div class="yun_joblist_ope">
        <NuxtLink :to="`/jobs/${job.id}`" class="search_job_Apply_fast">{{ $t('wap_00574') }}</NuxtLink>
      </div>
      <div class="job_bottom">
        <div class="job_bottomleft">
          <div v-if="welfare.length" class="job_welfare_tag">
            <span v-for="w in welfare" :key="w" class="job_welfare_tag_s">{{ w }}</span>
          </div>
        </div>
        <span v-if="posted" class="yunjoblist_new_time">{{ posted }}</span>
      </div>
    </div>
  </div>

  <!-- PC 企业详情在招，对齐 company/default/index.htm firm_post -->
  <div v-if="variant === 'firm'" class="firm_post site-pc">
    <div class="com_details_com_otherjob_l">
      <div class="com_details_com_otherjob_name">
        <NuxtLink :to="`/jobs/${job.id}`">{{ job.name }}</NuxtLink>
      </div>
      <div class="com_details_com_otherjob_info">
        <template v-if="job.exp_n">{{ job.exp_n }}{{ $t('home.experience_suffix') }}</template>
        <span v-if="job.exp_n && job.edu_n" class="com_details_line">|</span>
        <template v-if="job.edu_n">{{ job.edu_n }}{{ $t('home.education_suffix') }}</template>
      </div>
    </div>
    <div class="com_details_com_otherjob_c">
      <div class="com_details_com_otherjob_xz">{{ salary }}</div>
      <div class="com_details_com_otherjob_city">{{ cityTwo }}</div>
    </div>
    <div class="com_details_com_otherjob_r">
      <div class="com_details_com_otherjob_time">{{ posted }}</div>
      <NuxtLink :to="`/jobs/${job.id}`" class="com_details_com_otherjob_sq">{{ $t('wap_00574') }}</NuxtLink>
    </div>
  </div>

  <!-- H5 企业详情在招职位，对齐 company_show.htm comnew_joblist -->
  <NuxtLink v-if="variant === 'com'" class="site-h5" :to="`/jobs/${job.id}`" :title="job.name">
    <div class="comnew_joblist">
      <div class="comnew_jobtop">
        <div class="comnew_jobname">{{ job.name }}</div>
        <span v-if="posted" class="comnew_jobinfo_time">{{ posted }}</span>
      </div>
      <div class="comnew_jobxz">{{ salary }}</div>
      <div class="comnew_jobinfo">
        <span>{{ city }}</span>
        <template v-if="job.edu_n"> · {{ job.edu_n }}{{ $t('home.education_suffix') }}</template>
        <template v-if="job.exp_n"> · {{ job.exp_n }}{{ $t('home.experience_suffix') }}</template>
      </div>
      <div v-if="welfare.length" class="welfare">
        <span v-for="w in welfare" :key="w" class="welfare_n">{{ w }}</span>
      </div>
    </div>
  </NuxtLink>

  <!-- H5 职位列表，对齐 wap/job.htm tab_card -->
  <NuxtLink v-if="variant === 'search' || variant === 'tab'" class="site-h5" :to="`/jobs/${job.id}`" :title="job.name">
    <div class="tab_card">
      <div v-if="Number(job.fact_status) === 1" class="ptyhybox">
        <div class="ptyhy">
          <i class="ptyhy_icon" />{{ $t('wap_00274') }}
        </div>
      </div>
      <div class="tab_card_top">
        <div class="tab_card_job">
          <i class="tab_card_job_name">{{ job.name }}</i>
          <i v-if="job.newtime" class="tab_card_new">new</i>
        </div>
        <i class="tab_card_pay">{{ salary }}</i>
      </div>
      <div class="newjob_info">
        <span>{{ cityH5 }}</span>
        <template v-if="job.exp_n">
          <i class="newjob_info_line" />
          <span>{{ job.exp_n }}{{ $t('home.experience_suffix') }}</span>
        </template>
        <template v-if="job.edu_n">
          <i class="newjob_info_line" />
          <span>{{ job.edu_n }}{{ $t('home.education_suffix') }}</span>
        </template>
        <span class="newjob_fw">
          <img v-if="job.is_rec" src="/legacy/h5/images/icon_recommend.png" alt="" />
          <img v-if="job.is_urgent" src="/legacy/h5/images/jp.png" alt="" />
        </span>
      </div>
      <div v-if="welfare.length" class="welfare">
        <span v-for="w in welfare" :key="w" class="welfare_n">{{ w }}</span>
      </div>
      <div class="tab_card_bottom">
        <div class="card_bottom_logo">
          <img :src="logo" alt="" style="width: 100%" />
        </div>
        <i class="card_bottom_word">{{ job.com_name }}</i>
        <i v-if="Number(job.yyzz_status) === 1" class="job_qy_rz_icon" />
        <div v-if="posted" class="zdnow">{{ posted }}</div>
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { formatSalary, formatUnixDate, mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '../utils/site'

const props = withDefaults(
  defineProps<{ job: JobLike; variant?: 'home' | 'search' | 'tab' | 'com' | 'firm' }>(),
  { variant: 'home' },
)
const { t } = useI18n()
const salary = computed(() => formatSalary(props.job, t('common.negotiable'), t('ui.yuan')))
const city = computed(() => {
  const one = props.job.job_city_one || ''
  const two = props.job.job_city_two || props.job.city_two || ''
  if (one && two) return `${one}-${two}`
  return two || one
})
const cityTwo = computed(() => props.job.job_city_two || props.job.city_two || city.value)
const cityH5 = computed(
  () => props.job.job_city_three || props.job.job_city_two || props.job.city_two || props.job.job_city_one || '',
)
const logo = computed(() => mediaUrl(props.job.com_logo || props.job.logo, PLACEHOLDER_LOGO))
const posted = computed(() => props.job.lastupdate_n || formatUnixDate(props.job.lastupdate))
const welfare = computed(() => {
  const w = props.job.welfare_n || props.job.welfare
  if (Array.isArray(w)) return w.map(String).filter(Boolean)
  if (typeof w === 'string') return w.split(/[,，]/).map((s) => s.trim()).filter(Boolean)
  return [] as string[]
})
</script>
