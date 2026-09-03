<template>
  <div class="search_job_list site-pc">
    <div class="resume_newlist">
      <div class="resume_newlist_jobname">
        <NuxtLink :to="`/resumes/${row.uid}`" class="resume_newlist_job">{{ expectName }}</NuxtLink>
        <span v-if="row.is_top" class="lookjob">{{ $t('wap_user_00335') }}</span>
        <span v-if="row.in_talentpool" class="co_fav">{{ $t('wap_00378') }}</span>
        <span v-if="row.invited" class="co_fav">{{ $t('wap_00291') }}</span>
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
        <span v-for="c in cityTags" :key="c" class="user_tag">{{ c }}</span>
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
          <span v-for="j in expectJobs" :key="j" class="user_tag">{{ j }}</span>
        </div>
        <ul v-if="personTags.length" class="user_tag_user">
          <li v-for="tag in personTags" :key="tag" class="user_tag_user_a">
            <i class="user_tag_user_icon" />{{ tag }}
          </li>
        </ul>
      </div>
      <div v-if="salaryName" class="user_want">
        <div class="user_undergo">{{ $t('wap_user_00016') }}：{{ salaryName }}</div>
      </div>
      <div class="yun_look_right">
        <NuxtLink :to="`/resumes/${row.uid}`" class="yun_look_bth">{{ $t('wap_com_00427') }}</NuxtLink>
      </div>
    </div>
  </div>
  <NuxtLink class="site-h5 yun_newedition_resumelist" :to="`/resumes/${row.uid}`">
    <div class="yun_newedition_resumepic">
      <img :src="photo" alt="" />
    </div>
    <div class="yun_newedition_resume_wantjob">
      <span class="yun_newedition_resume_wantjob_n">{{ personName }}</span>
    </div>
    <div class="new_userlist_info">
      <template v-if="row.exp_n">{{ row.exp_n }}{{ $t('home.experience_suffix') }}</template>
      <template v-if="eduName"> · {{ eduName }}{{ $t('home.education_suffix') }}</template>
      <template v-if="row.age"> · {{ row.age }}{{ $t('home.age_suffix') }}</template>
    </div>
    <div class="new_userlist_p">
      {{ $t('wap_00586') }} {{ expectName }}
      <template v-if="salaryName">，{{ salaryName }}</template>
      <span v-if="row.is_top" class="yun_newedition_resume_zd">{{ $t('wap_user_00335') }}</span>
      <span v-if="row.in_talentpool" class="yun_newedition_resume_zd">{{ $t('wap_00378') }}</span>
      <span v-if="row.invited" class="yun_newedition_resume_zd">{{ $t('wap_00291') }}</span>
      <span v-else-if="row.lastupdate_n && !row.is_top" class="yun_newedition_resume_zd">{{ row.lastupdate_n }}</span>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { mediaUrl, PLACEHOLDER_LOGO } from '../utils/site'

function splitTags(raw: unknown): string[] {
  if (Array.isArray(raw)) return raw.map(String).filter(Boolean).slice(0, 5)
  if (typeof raw === 'string' && raw) return raw.split(/[,，|/]/).map((s) => s.trim()).filter(Boolean).slice(0, 5)
  return []
}

const props = defineProps<{ row: Record<string, unknown> }>()
const personName = computed(() =>
  String(props.row.display_name || props.row.name || props.row.uname || props.row.uid || ''),
)
const expectJobs = computed(() => {
  const tags = splitTags(props.row.expect_name || props.row.job_classid_n || props.row.expect)
  return tags.length ? tags : [personName.value]
})
const expectName = computed(() => expectJobs.value[0] || personName.value)
const salaryName = computed(() => String(props.row.expect_salary_n || props.row.salary_n || ''))
const eduName = computed(() => String(props.row.edu_n || props.row.education_n || ''))
const cityTags = computed(() =>
  splitTags(props.row.expect_city_n || props.row.city_two || props.row.living),
)
const personTags = computed(() => splitTags(props.row.tag))
const photo = computed(() =>
  mediaUrl(String(props.row.photo_n || props.row.photo || ''), PLACEHOLDER_LOGO),
)
</script>
