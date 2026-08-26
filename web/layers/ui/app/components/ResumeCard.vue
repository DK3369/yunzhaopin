<template>
  <div class="search_job_list site-pc">
    <div class="resume_newlist">
      <div class="resume_newlist_jobname">
        <NuxtLink :to="`/resumes/${row.uid}`" class="resume_newlist_job">{{ expectName }}</NuxtLink>
        <img
          v-if="Number(row.idcard_status) === 1"
          src="/legacy/pc/images/sf.png"
          alt=""
          class="user_rz_img png fl"
        />
        <img
          v-if="row.has_photo"
          src="/legacy/pc/images/profile.png"
          alt=""
          class="user_rz_img png fl"
        />
      </div>
      <div class="resume_newlist_user">
        <NuxtLink :to="`/resumes/${row.uid}`" class="resume_newlist_username">{{ personName }}</NuxtLink>
        <template v-if="row.age">{{ row.age }}{{ $t('home.age_suffix') }} · </template>
        <template v-if="row.exp_n">{{ row.exp_n }}{{ $t('home.experience_suffix') }} · </template>
        <template v-if="eduName">{{ eduName }}{{ $t('home.education_suffix') }}</template>
      </div>
      <div class="resume_newlist_city">
        <span v-if="cityName" class="user_tag">{{ cityName }}</span>
      </div>
      <div v-if="row.lastupdate_n" class="resume_newlist_date">{{ row.lastupdate_n }}</div>
    </div>
    <div class="resumeshow">
      <div class="user_photo_left">
        <NuxtLink :to="`/resumes/${row.uid}`">
          <img :src="photo" width="60" height="60" alt="" />
        </NuxtLink>
      </div>
      <div class="usersearch_job_left_siaber">
        <div class="user_listinfo_job">
          {{ $t('wap_user_00055') }}：
          <span class="user_tag">{{ expectName }}</span>
        </div>
      </div>
      <div class="yun_look_right">
        <NuxtLink :to="`/resumes/${row.uid}`" class="yun_look_bth">{{ $t('wap_com_00427') }}</NuxtLink>
      </div>
    </div>
  </div>
  <NuxtLink class="site-h5 table-card" :to="`/resumes/${row.uid}`">
    <div class="card_post">
      <i class="table-card-word">{{ expectName }}</i>
    </div>
    <div class="table-card-require">
      <i class="requir-area">{{ personName }}</i>
      <i v-if="meta" class="requir_area_parting_line" />
      <i v-if="meta" class="requir-area">{{ meta }}</i>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { mediaUrl, PLACEHOLDER_LOGO } from '../utils/site'

const props = defineProps<{ row: Record<string, unknown> }>()
const personName = computed(() =>
  String(props.row.display_name || props.row.name || props.row.uname || props.row.uid || ''),
)
const expectName = computed(() =>
  String(props.row.expect_name || props.row.job_classid_n || props.row.expect || personName.value),
)
const eduName = computed(() => String(props.row.edu_n || props.row.education_n || ''))
const cityName = computed(() =>
  String(props.row.expect_city_n || props.row.city_two || props.row.living || ''),
)
const meta = computed(() =>
  [props.row.exp_n, eduName.value, cityName.value].filter(Boolean).join(' · '),
)
const photo = computed(() =>
  mediaUrl(String(props.row.photo_n || props.row.photo || ''), PLACEHOLDER_LOGO),
)
</script>
