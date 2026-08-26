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
const hasContact = computed(() => Boolean(row.value.telphone || row.value.email))
const photo = computed(() => mediaUrl(String(row.value.photo_n || row.value.photo || ''), PLACEHOLDER_LOGO))
useSeoMeta({ title: () => name.value || t('common.resume') })
async function download() {
  try {
    await api.post('/v1/mcenter/resume-downloads', { uid })
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
    <div class="site-pc resume_body">
      <div class="resume_body_card">
        <div class="new_user_box">
          <span class="new_user_name">{{ name || $t('common.resume') }}</span>
          <div class="Edit_your_resume_card_name_logo">
            <img :src="photo" alt="" />
          </div>
        </div>
        <div v-if="row.exp_n || row.edu_n || row.education_n || row.age" class="new_user_info">
          {{ row.exp_n }} · {{ row.edu_n || row.education_n }} · {{ row.age }}
        </div>
        <p v-if="expects.length" class="muted">
          {{ $t('home.intention') }}
          {{ expects.map((e) => e.name || e.job_class_n || e.job_classid_n).join(' / ') }}
        </p>
        <div class="new_user_touchbox">
          <div v-if="hasContact" class="new_user_touch">
            <i class="new_user_touchiocn" />{{ row.telphone || row.email }}
          </div>
          <p v-else class="muted">{{ $t('ui.login_company') }}</p>
        </div>
      </div>
      <div v-if="row.description" class="resume_body_card" v-html="String(row.description)" />
      <div v-if="works.length" class="resume_body_card">
        <h2>{{ $t('home.experience_suffix') }}</h2>
        <div v-for="w in works" :key="String(w.id)" class="muted">
          <strong>{{ w.name }}</strong> {{ w.title || w.department }} {{ w.sdate_n }} - {{ w.edate_n }}
          <div v-html="String(w.content || '')" />
        </div>
      </div>
      <div v-if="edus.length" class="resume_body_card">
        <h2>{{ $t('home.education_suffix') }}</h2>
        <div v-for="e in edus" :key="String(e.id)" class="muted">
          {{ e.name }} {{ e.specialty }} {{ e.education_n }} {{ e.sdate_n }} - {{ e.edate_n }}
        </div>
      </div>
      <div v-if="skills.length" class="resume_body_card">
        <h2>{{ $t('common.more') }}</h2>
        <p>{{ skills.map((s) => s.name).join(' / ') }}</p>
      </div>
      <p style="margin-top: 16px">
        <button type="button" class="job_ceil_jobtd" @click="download">{{ $t('wap_com_00235') }}</button>
      </p>
    </div>

    <div class="site-h5 resume_body">
      <div class="resume_body_card">
        <div class="new_user_box">
          <span class="new_user_name">{{ name || $t('common.resume') }}</span>
          <div class="Edit_your_resume_card_name_logo">
            <img :src="photo" alt="" style="width: 100%; height: 100%; border-radius: 50%" />
          </div>
        </div>
        <div class="new_user_info">
          {{ row.exp_n }}{{ $t('wap_com_00305') }} · {{ row.edu_n || row.education_n }}{{ $t('wap_com_00301') }}
          <template v-if="row.age"> · {{ row.age }}</template>
        </div>
      </div>
      <div v-if="expects.length" class="Preview_your_resume_intention">
        <div class="Preview_your_resume_header">
          <div class="Preview_your_resume_word">{{ $t('wap_00460') }}</div>
        </div>
        <div class="Preview_your_resume_category">
          {{ expects.map((e) => e.name || e.job_class_n).join(' / ') }}
        </div>
        <div v-if="row.lastupdate_n" class="Preview_your_resume_category">{{ row.lastupdate_n }}</div>
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
      </div>
    </div>
  </article>
</template>
