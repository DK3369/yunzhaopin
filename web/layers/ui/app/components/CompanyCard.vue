<template>
  <li class="site-pc">
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
      <span class="index_mq_box_hot_n">{{ company.city_two || company.city_one || '' }}</span>
    </div>
  </li>
  <NuxtLink class="site-h5 mqnewlist" :to="`/companies/${company.uid}`">
    <div class="mqnew">
      <div class="mqnewimg">
        <img :src="logo" alt="" style="width: 100%" />
      </div>
      <div class="mqnew_comname">{{ title }}</div>
      <div class="mqnew_comjob">{{ company.hy_n || company.city_two || '' }}</div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { companyName, mediaUrl, PLACEHOLDER_LOGO, type CompanyLike } from '../utils/site'

const props = defineProps<{ company: CompanyLike }>()
const title = computed(() => companyName(props.company))
const logo = computed(() => mediaUrl(props.company.logo_n || props.company.logo, PLACEHOLDER_LOGO))
</script>
