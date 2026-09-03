<template>
  <li
    v-if="variant === 'home'"
    class="site-pc"
    :class="{ current1: hover }"
    @mouseenter="hover = true"
    @mouseleave="hover = false"
  >
    <div class="index_mq_box_pic">
      <NuxtLink :to="`/companies/${company.uid}`" class="tlogo_p_a" :title="title">
        <img class="on" :src="logo" :alt="title" />
      </NuxtLink>
    </div>
    <div class="index_mq_box_name nowrap">{{ title }}</div>
    <div class="index_mq_box_info">
      {{ company.mun_n }}
      <i v-if="company.mun_n && company.hy_n" class="index_newjob_info_line">|</i>
      {{ company.hy_n }}
    </div>
    <div class="index_mq_box_hot">
      <span class="index_mq_box_hot_n">{{ jobNumLabel }}</span>{{ $t('home.hot_recruiting_jobs') }}
    </div>
    <div class="index_mq_box_cont_showall">
      <div class="index_mq_box_cont_showall_c">
        <div class="index_mq_box_cont_bg" />
        <div class="index_mq_box_cont_showjob">
          <div class="index_mq_box_cont_showjob_c">
            <div class="index_mq_box_cont_showcomname">
              <NuxtLink :to="`/companies/${company.uid}`" :title="title">{{ title }}</NuxtLink>
            </div>
            <div class="index_mq_box_cont_showcomname_linebox">
              <i class="index_mq_box_cont_showcomname_line" />
            </div>
            <template v-if="openJobs.length">
              <div v-for="job in openJobs" :key="job.id" class="index_mq_box_cont_showjoblist">
                <NuxtLink :to="`/jobs/${job.id}`">{{ job.name }}</NuxtLink>
              </div>
              <div v-if="showMoreJobs" class="index_mq_box_cont_showjobmore">
                <NuxtLink :to="`/companies/${company.uid}`">{{ $t('common.view_more') }}</NuxtLink>
              </div>
            </template>
            <div v-else class="index_mq_box_cont_showjobmore">
              <a>{{ $t('home.no_recruiting_jobs') }}</a>
            </div>
          </div>
        </div>
      </div>
    </div>
  </li>
  <NuxtLink v-if="variant === 'home'" class="site-h5 mqnewlist" :to="`/companies/${company.uid}`">
    <div class="mqnew">
      <div class="mqnewimg">
        <img :src="logo" alt="" style="width: 100%" />
      </div>
      <div class="mqnew_comname">{{ title }}</div>
      <div class="mqnew_comjob">{{ jobNumLabel }}{{ $t('home.job_openings') }}</div>
    </div>
  </NuxtLink>

  <div v-else-if="variant === 'firm'" class="firm_list site-pc">
    <div class="firm_det">
      <div class="firm_list_leftsidebar">
        <div class="firm_list_logo">
          <NuxtLink :to="`/companies/${company.uid}`">
            <img :src="logo" width="100" height="100" :alt="title" />
          </NuxtLink>
        </div>
      </div>
      <div class="firm_list_rightsidebar">
        <div class="firm_name">
          <span>
            <NuxtLink :to="`/companies/${company.uid}`" class="firm_name_a" :title="title">{{ title }}</NuxtLink>
          </span>
        </div>
        <div class="firm_qy_list">
          <span v-if="company.city_two || company.city_one" class="firm_qy_list_s">
            {{ company.city_one }}{{ company.city_two ? `-${company.city_two}` : '' }}
          </span>
          <span v-if="company.hy_n" class="firm_qy_list_s">{{ company.hy_n }}</span>
          <span v-if="company.mun_n" class="firm_qy_list_s">{{ company.mun_n }}</span>
          <span v-if="company.pr_n" class="firm_qy_list_s">{{ company.pr_n }}</span>
          <img
            v-if="Number(company.yyzz_status) === 1"
            src="/legacy/pc/images/disc_icon10.png"
            alt=""
            class="png"
            width="16"
          />
          <span v-if="Number(company.fact_status) === 1" class="firm_qy_list_s">{{ $t('wap_00274') }}</span>
        </div>
        <div v-if="welfareTags.length" class="welfare">
          <span v-for="w in welfareTags" :key="w" class="welfare_n">{{ w }}</span>
        </div>
        <div v-if="typeof company.job_num === 'number'" class="firm_qy_job_list">
          <div class="firm_qy_job_list_name">{{ $t('wap_00185') }}：</div>
          <div class="firm_qy_job_list_r">
            <span v-if="company.job_num" class="firm_qy_job_tag">{{ company.job_num }}</span>
            <div v-else class="firm_qy_job_no">{{ $t('home.no_recruiting_jobs') }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <NuxtLink v-if="variant === 'firm'" class="site-h5 job_list" :to="`/companies/${company.uid}`" :title="title">
    <div class="com_list_box">
      <div class="com_list_t_box">
        <div class="com_list_logo_box">
          <img :src="logo" alt="" />
        </div>
        <div class="com_list_box_c">
          <h3>{{ title }}</h3>
        </div>
        <div class="com_list_box_js">
          <span v-if="company.city_one || company.city_two" class="com_list_box_js_n">
            {{ company.city_one }}{{ company.city_two ? `-${company.city_two}` : '' }}
          </span>
          <span v-if="company.mun_n" class="com_list_box_js_n">{{ company.mun_n }}</span>
          <span v-if="company.pr_n" class="com_list_box_js_n">{{ company.pr_n }}</span>
          <span v-if="typeof company.job_num === 'number'" class="com_list_box_js_n">{{ jobNumLabel }}{{ $t('home.job_openings') }}</span>
        </div>
        <div v-if="welfareTags.length" class="welfare">
          <span v-for="w in welfareTags" :key="w" class="welfare_n">{{ w }}</span>
        </div>
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { companyName, mediaUrl, PLACEHOLDER_LOGO, type CompanyLike } from '../utils/site'

const props = withDefaults(defineProps<{ company: CompanyLike; variant?: 'home' | 'firm' }>(), {
  variant: 'home',
})
const { t } = useI18n()
const hover = ref(false)
const title = computed(() => companyName(props.company, t('common.company')))
const logo = computed(() => {
  if (props.variant === 'home') {
    return mediaUrl(
      props.company.hot_pic_n || props.company.hot_pic || props.company.logo_n || props.company.logo,
      PLACEHOLDER_LOGO,
    )
  }
  return mediaUrl(props.company.logo_n || props.company.logo, PLACEHOLDER_LOGO)
})
const jobNumLabel = computed(() => {
  if (typeof props.company.job_num === 'number') return String(props.company.job_num)
  return '0'
})
const openJobs = computed(() => (props.company.open_jobs || []).slice(0, 3))
const showMoreJobs = computed(() => Number(props.company.job_num || 0) > openJobs.value.length)
const welfareTags = computed(() => {
  const w = props.company.welfare_n
  if (Array.isArray(w)) return w.map(String).filter(Boolean).slice(0, 6)
  if (typeof w === 'string' && w) return w.split(/[,，]/).map((s) => s.trim()).filter(Boolean).slice(0, 6)
  return [] as string[]
})
</script>
