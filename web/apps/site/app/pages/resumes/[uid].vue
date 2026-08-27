<script setup lang="ts">
import { mediaUrl, PLACEHOLDER_LOGO } from '~/utils/site'

const route = useRoute()
const { t, locale } = useI18n()
const uid = Number(route.params.uid)
const api = useApi()
const { data, error } = await useAsyncData(
  () => `resume-${locale.value}-${uid}`,
  () => api.get('/v1/wap/resumes/detail', { uid }),
)
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const name = computed(() => String(row.value.display_name || row.value.name || row.value.uname || ''))
const works = computed(() => (Array.isArray(row.value.works) ? row.value.works : []) as Record<string, unknown>[])
const edus = computed(() => (Array.isArray(row.value.edus) ? row.value.edus : []) as Record<string, unknown>[])
const skills = computed(() => (Array.isArray(row.value.skills) ? row.value.skills : []) as Record<string, unknown>[])
const expects = computed(() => (Array.isArray(row.value.expects) ? row.value.expects : []) as Record<string, unknown>[])
const expect0 = computed(() => expects.value[0] || {})
const expectJobs = computed(() =>
  expects.value
    .map((e) => String(e.name || e.job_class_n || e.job_classid_n || ''))
    .filter(Boolean)
    .slice(0, 5),
)
const expectCities = computed(() =>
  expects.value
    .map((e) => String(e.city_class_n || ''))
    .filter(Boolean)
    .slice(0, 5),
)
const expectTitle = computed(() => expectJobs.value[0] || '')
const expectCity = computed(() => expectCities.value[0] || '')
const expectSalary = computed(() => String(expect0.value.salary_n || ''))
const hasContact = computed(() => Boolean(row.value.telphone || row.value.email))
const photo = computed(() => mediaUrl(String(row.value.photo_n || row.value.photo || ''), PLACEHOLDER_LOGO))
const sexLabel = computed(() => {
  const n = Number(row.value.sex)
  if (n === 1) return t('common_02092')
  if (n === 2) return t('common_02069')
  return ''
})
const tags = computed(() => {
  const raw = row.value.tag
  if (Array.isArray(raw)) return raw.map(String).filter(Boolean).slice(0, 8)
  if (typeof raw === 'string' && raw) return raw.split(/[,，]/).map((s) => s.trim()).filter(Boolean).slice(0, 8)
  return [] as string[]
})
const fav = ref(false)
const actionMsg = ref('')
useSeoMeta({ title: () => name.value || t('common.resume') })
onMounted(async () => {
  try {
    const r = await api.post<{ exists?: boolean; favorited?: boolean }>('/v1/mcenter/favorites/exists', {
      kind: 3,
      target_id: uid,
    })
    fav.value = Boolean(r.exists || r.favorited)
  } catch {
    /* guest */
  }
})
async function download() {
  try {
    await api.post('/v1/mcenter/resume-downloads', { uid })
    actionMsg.value = t('common.confirm')
  } catch {
    await navigateTo('/login')
  }
}
async function toggleFav() {
  const eid = Number(row.value.def_job || expect0.value.id || 0)
  try {
    if (eid) {
      await api.post('/v1/mcenter/talent-pool', { eid, seeker_uid: uid })
      fav.value = true
    } else {
      const r = await api.post<{ favorited: boolean }>('/v1/mcenter/favorites', { kind: 3, target_id: uid })
      fav.value = Boolean(r.favorited)
    }
  } catch {
    await navigateTo('/login')
  }
}
async function report() {
  try {
    await api.post('/v1/mcenter/reports', { target_kind: 3, target_id: uid, reason_code: 'other', detail: '' })
    actionMsg.value = t('common.confirm')
  } catch {
    await navigateTo('/login')
  }
}
</script>

<template>
  <article v-if="error" class="site-inner">
    <h1>{{ $t('common.resume') }}</h1>
    <p class="muted">{{ $t('ui.load_failed') }}</p>
  </article>
  <article v-else>
    <div class="site-pc">
      <div class="yun_newedition_resume_top">
        <div class="w1200">
          <div class="yun_newedition_resume_current">
            {{ $t('common.home') }} > {{ $t('common.resume') }} > {{ $t('resume_00062') }}
          </div>
          <div class="yun_newedition_resume_info">
            <div class="yun_newedition_resume_info_c">
              <div class="yun_newedition_resume_info_pic">
                <img :src="photo" width="140" height="140" alt="" />
                <i class="resume_list_xb" :class="{ resume_list_xb_nv: Number(row.sex) === 2 }" />
              </div>
              <div>
                <span class="yun_newedition_resume_username">{{ name || $t('common.resume') }}</span>
                <span v-if="Number(row.idcard_status) === 1" class="yun_newedition_resume_rz yun_newedition_resume_rz_sm">{{
                  $t('member_com_00026')
                }}</span>
                <span v-if="Number(row.moblie_status) === 1" class="yun_newedition_resume_rz yun_newedition_resume_rz_sj">{{
                  $t('member_com_00071')
                }}</span>
                <span v-if="Number(row.email_status) === 1" class="yun_newedition_resume_rz yun_newedition_resume_rz_yx">{{
                  $t('wap_com_00186')
                }}</span>
              </div>
              <div class="yun_newedition_resume_basic">
                <template v-if="sexLabel">{{ sexLabel }}<span class="yun_newedition_resume_line">|</span></template>
                <template v-if="row.age">{{ row.age }}{{ $t('home.age_suffix') }}<span class="yun_newedition_resume_line">|</span></template>
                <template v-if="row.height">{{ row.height }}cm </template>
                <template v-if="row.weight">{{ row.weight }}kg <span class="yun_newedition_resume_line">|</span></template>
                <template v-if="row.exp_n">{{ row.exp_n }}{{ $t('home.experience_suffix') }}<span class="yun_newedition_resume_line">|</span></template>
                <template v-if="row.edu_n || row.education_n">{{ row.edu_n || row.education_n }}{{ $t('home.education_suffix') }}</template>
                <template v-if="row.living"><span class="yun_newedition_resume_line">|</span>{{ $t('common_02013') }}{{ row.living }}</template>
              </div>
              <div v-if="tags.length" class="yun_newedition_resume_fl">
                <span v-for="tag in tags" :key="tag">{{ tag }}</span>
              </div>
              <div class="yun_newedition_resume_data">
                <span v-if="row.lastupdate_n" class="yun_newedition_resume_data_time">{{ $t('wap_00225') }}：{{ row.lastupdate_n }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="w1200">
        <div class="yun_newedition_resume_cont">
          <div class="yun_newedition_resume_left">
            <div class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00460') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <ul class="yun_newedition_yx_list">
              <li style="width: 100%">
                <span class="yun_newedition_yx_name">{{ $t('wap_user_00055') }}：</span>
                <span v-for="j in expectJobs" :key="j" class="yun_newedition_yx_job">{{ j }}</span>
              </li>
              <li v-if="expectSalary">
                <span class="yun_newedition_yx_name">{{ $t('wap_user_00016') }}：</span>{{ expectSalary }}
              </li>
              <li v-if="expectCities.length" style="width: 100%">
                <span class="yun_newedition_yx_name">{{ $t('member_user_00198') }}：</span>
                <template v-for="c in expectCities" :key="c">{{ c }}&nbsp;</template>
              </li>
            </ul>
            <div v-if="hasContact">
              <div class="yun_newedition_tit">
                <span class="yun_newedition_tit_s">{{ $t('wap_00462') }}</span>
                <i class="yun_newedition_tit_line" />
              </div>
              <div class="tcktouch_box_p">
                {{ $t('member_user_00163') }}：<span class="tcktouch_box_p_sj">{{ row.telphone || row.email }}</span>
              </div>
            </div>
            <p v-else class="muted">{{ $t('ui.login_company') }}</p>
            <div v-if="row.description" class="yun_newedition_tit" style="margin-top: 16px">
              <span class="yun_newedition_tit_s">{{ $t('wap_00456') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-if="row.description" v-html="String(row.description)" />
            <div v-if="works.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00457') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="w in works" :key="String(w.id)" class="muted">
              <strong>{{ w.name }}</strong> {{ w.title || w.department }} {{ w.sdate_n }} - {{ w.edate_n }}
              <div v-html="String(w.content || '')" />
            </div>
            <div v-if="edus.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('home.education_suffix') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="e in edus" :key="String(e.id)" class="muted">
              {{ e.name }} {{ e.specialty }} {{ e.education_n }} {{ e.sdate_n }} - {{ e.edate_n }}
            </div>
            <div v-if="skills.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('common.more') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <p v-if="skills.length">{{ skills.map((s) => s.name).join(' / ') }}</p>
            <p style="margin-top: 16px">
              <button type="button" class="user_yqms" @click="download">{{ $t('resume_00029') }}</button>
              <a href="javascript:;" class="job_ceil_jobsc" @click.prevent="toggleFav">{{
                fav ? $t('wap_00378') : $t('wap_00379')
              }}</a>
              <a href="javascript:;" class="job_ceil_jobsc" @click.prevent="report">{{ $t('wap_com_00350') }}</a>
            </p>
            <p v-if="actionMsg" class="muted">{{ actionMsg }}</p>
          </div>
        </div>
      </div>
    </div>

    <div class="site-h5">
      <div class="Edit_your_resume_header" style="display: flex; align-items: center; justify-content: space-between; padding: 0.2rem 0.32rem">
        <NuxtLink to="/resumes" class="Edit_your_resume_header_left">
          <img src="/legacy/h5/images/return.png" alt="" width="24" height="24" />
        </NuxtLink>
        <div>
          <a href="javascript:;" class="new_user_headericon" @click.prevent="toggleFav">
            <img :src="fav ? '/legacy/h5/images/ysc_n.png' : '/legacy/h5/images/r_sc.png'" alt="" width="24" height="24" />
          </a>
          <a href="javascript:;" class="new_user_headerjb" @click.prevent="report">
            <img src="/legacy/h5/images/details_report.png" alt="" width="24" height="24" />
          </a>
        </div>
      </div>
      <div class="resume_body">
        <div class="resume_body_card">
          <div class="new_user_box">
            <span class="new_user_name">{{ name || $t('common.resume') }}</span>
            <div class="Edit_your_resume_card_name_logo">
              <img :src="photo" alt="" style="width: 100%; height: 100%; border-radius: 50%" />
              <i v-if="Number(row.sex) === 2" class="yun_newedition_resume_userxb" />
              <i v-else-if="Number(row.sex) === 1" class="yun_newedition_resume_userxb_n" />
            </div>
          </div>
          <div class="new_user_info">
            {{ row.exp_n }}{{ $t('wap_com_00305') }} · {{ row.edu_n || row.education_n }}{{ $t('wap_com_00301') }}
            <template v-if="row.age"> · {{ row.age }}{{ $t('common_02074') }}</template>
            <template v-if="row.living"> · {{ row.living }}</template>
          </div>
        </div>
        <div class="Preview_your_resume_intention">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00460') }}</div>
          </div>
          <div class="user_qwinfobox">
            <div class="user_qwinfo">
              <span>{{ expectTitle }}</span>
              <span v-if="expectCity">· {{ expectCity }}</span>
            </div>
            <div v-if="expectSalary" class="user_qwxz">{{ expectSalary }}</div>
          </div>
          <div v-if="row.lastupdate_n" class="Preview_your_resume_category">
            {{ row.lastupdate_n }} {{ $t('wap_00225') }}
          </div>
        </div>
        <div v-if="works.length" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00457') }}</div>
          </div>
          <div v-for="w in works" :key="String(w.id)">
            <strong>{{ w.name }}</strong> {{ w.sdate_n }} - {{ w.edate_n }}
            <div v-html="String(w.content || '')" />
          </div>
        </div>
        <div v-if="edus.length" class="Preview_your_resume_education">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('home.education_suffix') }}</div>
          </div>
          <div v-for="e in edus" :key="String(e.id)">
            {{ e.name }} {{ e.specialty }} {{ e.education_n }} {{ e.sdate_n }} - {{ e.edate_n }}
          </div>
        </div>
        <div class="resume_body_card">
          <p v-if="hasContact">{{ row.telphone || row.email }}</p>
          <p v-else class="muted">{{ $t('ui.login_company') }}</p>
          <button type="button" class="job_ceil_jobtd" @click="download">{{ $t('wap_com_00235') }}</button>
          <p v-if="actionMsg" class="muted">{{ actionMsg }}</p>
        </div>
      </div>
    </div>
  </article>
</template>
