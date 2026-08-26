<template>
  <li class="site-pc">
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
  <NuxtLink class="site-h5" :to="`/jobs/${job.id}`" :title="job.name">
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
</template>

<script setup lang="ts">
import { formatSalary, mediaUrl, PLACEHOLDER_LOGO, type JobLike } from '../utils/site'

const props = defineProps<{ job: JobLike }>()
const salary = computed(() => formatSalary(props.job))
const city = computed(
  () => props.job.job_city_two || props.job.city_two || props.job.job_city_one || '',
)
const logo = computed(() => mediaUrl(props.job.com_logo || props.job.logo, PLACEHOLDER_LOGO))
</script>
