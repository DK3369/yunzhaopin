<script setup lang="ts">
const route = useRoute()
const { t, locale } = useI18n()
const uid = Number(route.params.uid)
const api = useApi()
const { data } = await useAsyncData(
  () => `resume-${locale.value}-${uid}`,
  () => api.get('/v1/wap/resumes/detail', { uid }),
)
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const name = computed(() => String(row.value.display_name || row.value.name || row.value.uname || ''))
const works = computed(() => (Array.isArray(row.value.works) ? row.value.works : []) as Record<string, unknown>[])
const edus = computed(() => (Array.isArray(row.value.edus) ? row.value.edus : []) as Record<string, unknown>[])
const skills = computed(() => (Array.isArray(row.value.skills) ? row.value.skills : []) as Record<string, unknown>[])
const expects = computed(() => (Array.isArray(row.value.expects) ? row.value.expects : []) as Record<string, unknown>[])
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
  <article>
    <div class="resume_body">
      <div class="resume_body_card">
        <div class="new_user_box">
          <span class="new_user_name">{{ name || $t('common.resume') }}</span>
        </div>
        <div v-if="row.exp_n || row.edu_n || row.education_n" class="new_user_info">
          {{ row.exp_n }} · {{ row.edu_n || row.education_n }} · {{ row.age }}
        </div>
        <p v-if="expects.length" class="muted">
          {{ $t('home.intention') }}
          {{ expects.map((e) => e.name || e.job_classid_n).join(' / ') }}
        </p>
      </div>
      <div v-if="row.description" class="resume_body_card" v-html="String(row.description)" />
      <p v-else-if="!name" class="muted">{{ $t('auth.company_only_view') }}</p>
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
  </article>
</template>
