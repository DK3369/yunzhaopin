<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

type ChildRow = {
  id: number
  name?: string
  title?: string
  specialty?: string
  role?: string
  department?: string
  content?: string
  sdate?: number
  edate?: number
  sdate_n?: string
  edate_n?: string
  date_n?: string
  years?: number
  level?: number
  education_n?: string
  salary?: number
  salary_n?: string
  job_class_n?: string
  city_class_n?: string
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('my-resume', () =>
  api.post('/v1/mcenter/resume/list', {}),
)
const { data: expects, refresh: refreshExpects } = await useAsyncData('my-expects', () =>
  api.post('/v1/mcenter/resume/expects/list', {}).catch(() => []),
)
const { data: works, refresh: refreshWorks } = await useAsyncData('my-works', () =>
  api.post('/v1/mcenter/resume/works/list', {}).catch(() => []),
)
const { data: edus, refresh: refreshEdus } = await useAsyncData('my-edus', () =>
  api.post('/v1/mcenter/resume/edus/list', {}).catch(() => []),
)
const { data: projects, refresh: refreshProjects } = await useAsyncData('my-projects', () =>
  api.post('/v1/mcenter/resume/projects/list', {}).catch(() => []),
)
const { data: skills, refresh: refreshSkills } = await useAsyncData('my-skills', () =>
  api.post('/v1/mcenter/resume/skills/list', {}).catch(() => []),
)
const { data: trainings, refresh: refreshTrainings } = await useAsyncData('my-trainings', () =>
  api.post('/v1/mcenter/resume/trainings/list', {}).catch(() => []),
)
const { data: certs, refresh: refreshCerts } = await useAsyncData('my-certs', () =>
  api.post('/v1/mcenter/resume/certs/list', {}).catch(() => []),
)
const { data: others, refresh: refreshOthers } = await useAsyncData('my-others', () =>
  api.post('/v1/mcenter/resume/others/list', {}).catch(() => []),
)
const { data: languages, refresh: refreshLanguages } = await useAsyncData('my-languages', () =>
  api.post('/v1/mcenter/resume/languages/list', {}).catch(() => []),
)
const { data: shows, refresh: refreshShows } = await useAsyncData('my-resume-gallery', () =>
  api.post('/v1/mcenter/galleries/list', { kind: 'resume', page: 1, page_size: 20 }).catch(() => ({ list: [] })),
)
const { data: shareTokens, refresh: refreshShares } = await useAsyncData('my-resume-shares', () =>
  api
    .post<{ list?: Array<{ token: string; view_count?: number; expires_at_n?: string; active?: boolean }> }>(
      '/v1/mcenter/resume-share-tokens/list',
      { page: 1, page_size: 20 },
    )
    .catch(() => ({ list: [] })),
)
const { data: dicts } = await usePublicDicts()
const eduDict = computed(() => dicts.value?.educations_user ?? [])
const expDict = computed(() => dicts.value?.experiences_user ?? [])
const { data: completion, refresh: refreshCompletion } = await useAsyncData('resume-edit-completion', () =>
  api.post<{ score?: number; missing?: string[] }>('/v1/mcenter/resume/completion', {}).catch(() => null),
)
const integrity = computed(() => Number(completion.value?.score || 0))
const missingBits = computed(() => completion.value?.missing || [])
function missingLabel(k: string) {
  const map: Record<string, string> = {
    basic_info: t('wap_00269'),
    photo: t('wap_user_00204'),
    expect: t('wap_00460'),
    education: t('wap_00459'),
    work: t('wap_00457'),
    skill_or_language_or_project: t('wap_00450'),
  }
  return map[k] || k
}
const form = reactive({
  name: '',
  sex: 1,
  birthday: '',
  telphone: '',
  email: '',
  photo: '',
  education: 0,
  exp: 0,
  living: '',
  domicile: '',
  height: '',
  weight: '',
  address: '',
  description: '',
  qq: '',
  marriage: 0,
})
watch(
  data,
  (row) => {
    if (!row) return
    form.name = String(row.name || '')
    form.sex = Number(row.sex || 1)
    form.birthday = String(row.birthday || '')
    form.telphone = String(row.telphone || '')
    form.email = String(row.email || '')
    form.photo = String(row.photo || '')
    form.education = Number(row.education || 0)
    form.exp = Number(row.exp || 0)
    form.living = String(row.living || '')
    form.domicile = String(row.domicile || '')
    form.height = String(row.height || '')
    form.weight = String(row.weight || '')
    form.address = String(row.address || '')
    form.description = String(row.description || '')
    form.qq = String(row.qq || '')
    form.marriage = Number(row.marriage || 0)
  },
  { immediate: true },
)
const expectForm = reactive({ id: 0, name: '', salary: 8000, type: 57 })
const workForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', department: '', title: '', content: '' })
const eduForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', specialty: '', education: 65 })
const projectForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', role: '', content: '' })
const skillForm = reactive({ id: 0, name: '', level: 0, years: 0 })
const trainingForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', title: '', content: '' })
const certForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', title: '', content: '' })
const otherForm = reactive({ id: 0, name: '', content: '' })
const languageForm = reactive({ id: 0, name: '', level: 0 })
const galleryTitle = ref('')
const shareTtl = ref(604800)
const msg = ref('')

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}
async function saveResume() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume', { ...form })
    if (!hasResume.value && expectForm.name) {
      await api.post('/v1/mcenter/resume/expects', { ...expectForm }).catch(() => null)
      await refreshExpects()
    }
    msg.value = t('common.success')
    await refresh()
    await refreshCompletion()
    if (openSec.value === 'basic') openSec.value = ''
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function onAvatar(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  msg.value = ''
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/avatar', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    form.photo = r.key || r.url
    await api.post('/v1/mcenter/resume', { photo: form.photo })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function onShow(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  msg.value = ''
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/resume-photo', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    await api.post('/v1/mcenter/galleries/create', {
      kind: 'resume',
      title: galleryTitle.value,
      picurl: r.key || r.url,
    })
    galleryTitle.value = ''
    msg.value = t('common.success')
    await refreshShows()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function removeShow(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/galleries/delete', { kind: 'resume', ids: [id] })
    msg.value = t('common.success')
    await refreshShows()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function createShare() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-share-tokens', { ttl_secs: shareTtl.value })
    msg.value = t('common.success')
    await refreshShares()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function revokeShare(token: string) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-share-tokens/revoke', { token })
    msg.value = t('common.success')
    await refreshShares()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function refreshResume() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/refresh', {})
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function saveExpect() {
  msg.value = ''
  try {
    if (expectForm.id) await api.post('/v1/mcenter/resume/expects/update', { ...expectForm })
    else await api.post('/v1/mcenter/resume/expects', { ...expectForm })
    msg.value = t('common.success')
    openSec.value = ''
    await refreshExpects()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function saveChild(
  kind: string,
  body: Record<string, unknown>,
  reload: () => Promise<unknown>,
) {
  msg.value = ''
  try {
    const id = Number(body.id || 0)
    if (id) await api.post(`/v1/mcenter/resume/${kind}/update`, body)
    else await api.post(`/v1/mcenter/resume/${kind}`, body)
    msg.value = t('common.success')
    openSec.value = ''
    resetSec(kind)
    await reload()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function delChild(kind: string, row: ChildRow, reload: () => Promise<unknown>) {
  msg.value = ''
  try {
    await api.post(`/v1/mcenter/resume/${kind}/update`, {
      id: row.id,
      name: row.name || '-',
      status: 2,
    })
    msg.value = t('common.success')
    await reload()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
const openSec = ref('')
function toggleSec(name: string) {
  openSec.value = openSec.value === name ? '' : name
}
function resetSec(name: string) {
  if (name === 'expect' || name === 'expects') Object.assign(expectForm, { id: 0, name: '', salary: 8000, type: 57 })
  if (name === 'work' || name === 'works') Object.assign(workForm, { id: 0, name: '', sdate_n: '', edate_n: '', department: '', title: '', content: '' })
  if (name === 'edu' || name === 'edus') Object.assign(eduForm, { id: 0, name: '', sdate_n: '', edate_n: '', specialty: '', education: 65 })
  if (name === 'project' || name === 'projects') Object.assign(projectForm, { id: 0, name: '', sdate_n: '', edate_n: '', role: '', content: '' })
  if (name === 'skill' || name === 'skills') Object.assign(skillForm, { id: 0, name: '', level: 0, years: 0 })
  if (name === 'training' || name === 'trainings') Object.assign(trainingForm, { id: 0, name: '', sdate_n: '', edate_n: '', title: '', content: '' })
  if (name === 'cert' || name === 'certs') Object.assign(certForm, { id: 0, name: '', sdate_n: '', edate_n: '', title: '', content: '' })
  if (name === 'other' || name === 'others') Object.assign(otherForm, { id: 0, name: '', content: '' })
  if (name === 'language' || name === 'languages') Object.assign(languageForm, { id: 0, name: '', level: 0 })
}
function openAdd(name: string) {
  if (openSec.value === name) {
    openSec.value = ''
    return
  }
  resetSec(name)
  openSec.value = name
}
function openExpect() {
  const row = expectRows.value[0]
  if (row) {
    expectForm.id = row.id
    expectForm.name = String(row.name || '')
    expectForm.salary = Number(row.salary || 8000)
  } else resetSec('expect')
  openSec.value = openSec.value === 'expect' ? '' : 'expect'
}
function childList(v: unknown): ChildRow[] {
  if (!v) return []
  if (Array.isArray(v)) return v as ChildRow[]
  if (typeof v === 'object' && v && 'list' in v) return ((v as { list?: ChildRow[] }).list || []) as ChildRow[]
  return []
}
const expectRows = computed(() => childList(expects.value))
const workRows = computed(() => childList(works.value))
const eduRows = computed(() => childList(edus.value))
const projectRows = computed(() => childList(projects.value))
const skillRows = computed(() => childList(skills.value))
const trainingRows = computed(() => childList(trainings.value))
const certRows = computed(() => childList(certs.value))
const otherRows = computed(() => childList(others.value))
const languageRows = computed(() => childList(languages.value))
const hasResume = computed(() => {
  if (String(form.name || data.value?.name || '').trim()) return true
  if (Number(data.value?.def_job || 0) > 0) return true
  return expectRows.value.length > 0
})
function timeOf(row: ChildRow) {
  if (row.date_n) return String(row.date_n)
  const s = row.sdate_n || ''
  const e = row.edate_n || ''
  if (s || e) return `${s}-${e || '—'}`
  return ''
}
function fillWork(row: ChildRow) {
  workForm.id = row.id
  workForm.name = String(row.name || '')
  workForm.title = String(row.title || '')
  workForm.department = String(row.department || '')
  workForm.content = String(row.content || '')
  workForm.sdate_n = String(row.sdate_n || '')
  workForm.edate_n = String(row.edate_n || '')
  openSec.value = 'work'
}
function fillEdu(row: ChildRow) {
  eduForm.id = row.id
  eduForm.name = String(row.name || '')
  eduForm.specialty = String(row.specialty || '')
  eduForm.sdate_n = String(row.sdate_n || '')
  eduForm.edate_n = String(row.edate_n || '')
  openSec.value = 'edu'
}
function fillProject(row: ChildRow) {
  projectForm.id = row.id
  projectForm.name = String(row.name || '')
  projectForm.role = String(row.role || '')
  projectForm.content = String(row.content || '')
  projectForm.sdate_n = String(row.sdate_n || '')
  projectForm.edate_n = String(row.edate_n || '')
  openSec.value = 'project'
}
function fillSkill(row: ChildRow) {
  skillForm.id = row.id
  skillForm.name = String(row.name || '')
  skillForm.years = Number(row.years || 0)
  skillForm.level = Number(row.level || 0)
  openSec.value = 'skill'
}
function fillTraining(row: ChildRow) {
  trainingForm.id = row.id
  trainingForm.name = String(row.name || '')
  trainingForm.title = String(row.title || '')
  trainingForm.content = String(row.content || '')
  trainingForm.sdate_n = String(row.sdate_n || '')
  trainingForm.edate_n = String(row.edate_n || '')
  openSec.value = 'training'
}
function fillCert(row: ChildRow) {
  certForm.id = row.id
  certForm.name = String(row.name || '')
  certForm.title = String(row.title || '')
  certForm.content = String(row.content || '')
  certForm.sdate_n = String(row.sdate_n || '')
  certForm.edate_n = String(row.edate_n || '')
  openSec.value = 'cert'
}
function fillOther(row: ChildRow) {
  otherForm.id = row.id
  otherForm.name = String(row.name || '')
  otherForm.content = String(row.content || '')
  openSec.value = 'other'
}
function fillLanguage(row: ChildRow) {
  languageForm.id = row.id
  languageForm.name = String(row.name || '')
  languageForm.level = Number(row.level || 0)
  openSec.value = 'language'
}
watch(
  () => [hasResume.value, form.name] as const,
  ([ok, name]) => {
    if (ok && !name && !openSec.value) openSec.value = 'basic'
  },
  { immediate: true },
)
const topDays = ref(7)
const topMsg = ref('')
async function buyTop() {
  topMsg.value = ''
  const resumeid = Number(data.value?.def_job || (Array.isArray(expects.value) ? expects.value[0]?.id : 0) || 0)
  if (!resumeid) {
    topMsg.value = t('ui.no_expect')
    return
  }
  try {
    const r = await api.post<{ status?: number; pay_url?: string; msg?: string }>('/v1/mcenter/resume/top', {
      resumeid,
      days: topDays.value,
    })
    if (r.pay_url) {
      window.location.assign(r.pay_url)
      return
    }
    topMsg.value = r.msg || t('common.success')
  } catch (e: unknown) {
    topMsg.value = fail(e)
  }
}
useSeoMeta({ title: t('wap_user_00204') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00204')" :error="error && !isUnauthErr(error) ? error : undefined">
    <template #titExtra>
      <NuxtLink to="/user/expects" class="user_cjbth" style="float: right; margin-top: 7px; margin-right: 10px">{{ $t('wap_user_00197') }}</NuxtLink>
      <div class="user_czbth_r" style="float: right; background-color: #fff; padding-top: 8px">
        <NuxtLink to="/user/privacy" class="user_czbth_ys user_czbth_line">{{ $t('wap_user_00215') }}</NuxtLink>
        <NuxtLink to="/user/recommend" class="user_czbth_pp user_czbth_line">{{ $t('wap_user_00211') }}</NuxtLink>
        <NuxtLink to="/user/expects" class="user_czbth_zt">{{ $t('member_user_00273') }}</NuxtLink>
      </div>
    </template>
    <div v-if="!hasResume" class="site-h5">
      <div class="create_resume">
        <div class="create_resume_h1">{{ $t('wap_00932') }}</div>
        <div class="create_resume_p">{{ $t('wap_user_00022') }}</div>
        <div class="create_resume_img">
          <img src="/legacy/h5/images/resume_title.png" alt="" width="100%" height="100%" />
        </div>
      </div>
      <div class="create_resume_box">
        <form class="yun_createbox" @submit.prevent="saveResume">
          <div class="yun_createlist yun_createlist_pr">
            <div class="yun_create_name"><span class="m_bt">*</span>{{ $t('wap_00529') }}</div>
            <div class="yun_create_text"><input v-model="form.name" required /></div>
            <div class="yun_create_gender">
              <div :class="{ yun_create_genderselect: form.sex === 1 }" @click="form.sex = 1">{{ $t('common_02092') }}</div>
              <div :class="{ yun_create_genderselect: form.sex === 2 }" @click="form.sex = 2">{{ $t('common_02069') }}</div>
            </div>
          </div>
          <MemberField wap :label="$t('wap_00460')"><input v-model="expectForm.name" /></MemberField>
          <MemberField wap :label="$t('wap_user_00242')"><input v-model="form.living" /></MemberField>
          <MemberField wap :label="$t('common.phone')"><input v-model="form.telphone" /></MemberField>
          <MemberField wap :label="$t('ui.birthday')"><input v-model="form.birthday" /></MemberField>
          <MemberField wap :label="$t('wap_00459')">
            <select v-model.number="form.education">
              <option :value="0">{{ $t('wap_00459') }}</option>
              <option v-for="d in eduDict || []" :key="'c-edu-' + d.id" :value="d.id">{{ d.name }}</option>
            </select>
          </MemberField>
          <MemberField wap :label="$t('wap_00457')">
            <select v-model.number="form.exp">
              <option :value="0">{{ $t('wap_00457') }}</option>
              <option v-for="d in expDict || []" :key="'c-exp-' + d.id" :value="d.id">{{ d.name }}</option>
            </select>
          </MemberField>
          <button type="submit" class="Create_resume_btn">{{ $t('ui.save_resume') }}</button>
        </form>
      </div>
    </div>
    <div class="Edit_your_resume_min_body" :class="{ 'site-pc': !hasResume }">
      <div class="resume_min_body_cord">
        <div class="site-h5">
          <div class="resume_min_body_cord_data" @click="toggleSec('basic')">
            <div class="resume_min_body_cord_data_left">
              <div class="data_left_nameandmodification">
                <div class="data_left_name">{{ form.name || $t('wap_00529') }}</div>
                <div class="data_left_modification">
                  <img src="/legacy/h5/images/icon_question.png" alt="" width="100%" height="100%" />
                </div>
              </div>
              <div class="data_left_condition">
                <ul>
                  <li v-if="form.exp">{{ expDict.find((d) => d.id === form.exp)?.name }}</li>
                  <li v-if="form.education">{{ eduDict.find((d) => d.id === form.education)?.name }}</li>
                </ul>
              </div>
            </div>
            <div class="resume_min_body_cord_data_logo" @click.stop>
              <img :src="form.photo ? mediaUrl(form.photo) : '/legacy/h5/images/photograph.png'" alt="" width="100%" height="100%" />
              <input type="file" accept="image/jpeg,image/png,image/webp" @change="onAvatar" />
            </div>
          </div>
          <div class="resume_min_body_cord_intention">
            <div class="cord_intention_top">
              <div class="cord_intention_top_word">{{ $t('wap_00460') }}</div>
            </div>
            <div class="cord_intention_bom" @click="openExpect">
              <div class="data_left_condition">
                <ul>
                  <li>{{ expectRows[0]?.name || $t('ui.no_expect') }}</li>
                  <li v-if="expectRows[0]?.salary_n">· {{ expectRows[0].salary_n }}</li>
                  <li v-else-if="expectRows[0]?.salary">· {{ expectRows[0].salary }}</li>
                  <li v-if="expectRows[0]?.city_class_n">· {{ expectRows[0].city_class_n }}</li>
                </ul>
              </div>
              <div class="cord_intention_bom_icon">
                <img src="/legacy/h5/images/icon_more.png" alt="" width="100%" height="100%" />
              </div>
            </div>
          </div>
          <div class="resume_min_body_cord_work_experience" @click="toggleSec('basic')">
            <div class="cord_work_experience_one">
              <div class="cord_intention_top_word">{{ $t('wap_user_00326') }}</div>
              <div class="cord_intention_top_icon">
                <img src="/legacy/h5/images/icon_question.png" alt="" width="100%" height="100%" />
              </div>
            </div>
            <div v-if="form.description" class="cord_work_experience_four">{{ form.description }}</div>
          </div>
        </div>
    <div class="site-pc">
      <div class="user_resume_box">
        <div class="user_resume_photo">
          <img v-if="form.photo" :src="mediaUrl(form.photo)" alt="" />
        </div>
        <div class="user_resume_info">
          <div class="user_resume_name">
            {{ form.name }}
            <span v-if="expectRows[0]?.name" class="user_resume_job">{{ expectRows[0].name }}</span>
            <span class="user_resume_mr">{{ $t('wap_js_00098') }}</span>
          </div>
          <div class="user_resume_p">
            <template v-if="form.birthday">{{ form.birthday }}</template>
            <span v-if="form.exp" class="user_resume_line">|</span>
            <template v-if="form.exp">{{ expDict.find((d) => d.id === form.exp)?.name }}</template>
            <span v-if="form.education" class="user_resume_line">|</span>
            <template v-if="form.education">{{ eduDict.find((d) => d.id === form.education)?.name }}</template>
          </div>
        </div>
        <div class="user_resume_c">
          <div v-if="integrity" class="user_resume_wzd">
            <span class="user_resume_wzd_name">{{ $t('wap_00328') }}：</span>
            <div class="user_resume_wzd_b"><span class="user_resume_wzd_c" :style="{ width: `${integrity}%` }" /></div>
            <span class="user_resume_wzd_r">{{ integrity }}%</span>
          </div>
          <div class="user_resume_p user_resume_pd">{{ data?.lastupdate_n }}</div>
          <div v-if="data?.hits != null" class="user_resume_p">{{ $t('member_com_00268') }}：{{ data.hits }}</div>
        </div>
        <div class="user_resume_cz">
          <div class="user_resume_cz_p">
            <a href="javascript:;" class="user_resume_cz_a user_resume_cz_icon1" @click="buyTop">{{ $t('wap_user_00207') }}</a>
          </div>
          <div class="user_resume_cz_p">
            <a href="javascript:;" class="user_resume_cz_a user_resume_cz_icon3" @click="toggleSec('basic')">{{ $t('wap_00269') }}</a>
          </div>
          <div class="user_resume_cz_p">
            <NuxtLink :to="`/resumes/${data?.uid}`" class="user_resume_cz_a user_resume_cz_icon4">{{ $t('wap_user_00217') }}</NuxtLink>
          </div>
          <div class="user_resume_cz_p">
            <a href="javascript:;" class="user_resume_cz_a user_resume_cz_icon2" @click="refreshResume">{{ $t('wap_user_00199') }}</a>
          </div>
        </div>
      </div>
      <div v-if="missingBits.length" class="user_resume_boxtip">
        <div class="user_resume_boxtip_c">
          <div class="user_resume_boxtip_h1">{{ missingBits.map(missingLabel).join(' · ') }}</div>
          <a href="javascript:;" class="user_resume_boxtip_bth" @click="toggleSec('basic')">{{ $t('wap_00269') }}</a>
        </div>
      </div>
    </div>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-if="topMsg" class="muted">{{ topMsg }}</p>
    <form v-if="!error && openSec === 'basic'" class="verification_form yun_createbox" @submit.prevent="saveResume">
      <MemberField wap :label="$t('wap_00529')"><input v-model="form.name" /></MemberField>
      <MemberField wap :label="$t('common_02092')">
        <select v-model.number="form.sex">
          <option :value="1">{{ $t('common_02092') }}</option>
          <option :value="2">{{ $t('common_02069') }}</option>
        </select>
      </MemberField>
      <MemberField wap :label="$t('ui.birthday')"><input v-model="form.birthday" /></MemberField>
      <MemberField wap :label="$t('wap_00459')">
        <select v-model.number="form.education">
          <option :value="0">{{ $t('wap_00459') }}</option>
          <option v-for="d in eduDict || []" :key="d.id" :value="d.id">{{ d.name }}</option>
        </select>
      </MemberField>
      <MemberField wap :label="$t('wap_00457')">
        <select v-model.number="form.exp">
          <option :value="0">{{ $t('wap_00457') }}</option>
          <option v-for="d in expDict || []" :key="d.id" :value="d.id">{{ d.name }}</option>
        </select>
      </MemberField>
      <MemberField wap :label="$t('wap_user_00242')"><input v-model="form.living" /></MemberField>
      <MemberField wap :label="$t('member_user_00158')"><input v-model="form.domicile" /></MemberField>
      <MemberField wap :label="$t('member_user_00165')"><input v-model="form.height" /></MemberField>
      <MemberField wap :label="$t('member_user_00160')"><input v-model="form.weight" /></MemberField>
      <MemberField wap :label="$t('common.phone')"><input v-model="form.telphone" /></MemberField>
      <MemberField wap :label="$t('member_user_00282')"><input v-model="form.email" /></MemberField>
      <MemberField wap :label="$t('wap_user_00243')"><input v-model="form.address" /></MemberField>
      <MemberField wap label="QQ"><input v-model="form.qq" /></MemberField>
      <MemberField wap :label="$t('wap_user_00102')" area><textarea v-model="form.description" rows="4" /></MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('ui.save_resume') }}</button>
    </form>
    <MemberResumeSection :title="$t('home.intention')" icon="yun_resume_h1_iconyx" h5-kind="none" :open="openSec === 'expect'" @toggle="openAdd('expect')">
      <template #pc>
        <ul v-if="expectRows.length" class="yun_resume_job_intention_list">
          <li v-for="row in expectRows" :key="row.id">{{ row.name || row.id }}</li>
        </ul>
        <p v-else class="muted">{{ $t('ui.no_expect') }}</p>
      </template>
      <template #form>
        <form @submit.prevent="saveExpect">
          <MemberField wap :label="$t('wap_00460')"><input v-model="expectForm.name" /></MemberField>
          <MemberField wap :label="$t('ui.expect_salary')"><input v-model.number="expectForm.salary" type="number" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ $t('ui.add_expect') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeSection :title="$t('wap_00457')" icon="yun_resume_h1_iconjl" :open="openSec === 'work'" @toggle="openAdd('work')">
          <template #pc>
            <p v-if="!workRows.length" class="muted">{{ $t('ui.no_work') }}</p>
          </template>
          <template #h5>
            <MemberResumeExpItem
              v-for="row in workRows"
              :key="'h5-w-' + row.id"
              surface="h5"
              :title="String(row.name || '')"
              :sub="row.title"
              :time="timeOf(row)"
              :body="row.content"
              @edit="fillWork(row)"
              @remove="delChild('works', row, refreshWorks)"
            />
          </template>
          <template #form>
        <form @submit.prevent="saveChild('works', { ...workForm }, refreshWorks)">
          <MemberField wap :label="$t('common.company')"><input v-model="workForm.name" /></MemberField>
          <MemberField wap :label="$t('wap_com_00288')"><input v-model="workForm.title" /></MemberField>
          <MemberField wap :label="$t('default_00244')"><input v-model="workForm.department" /></MemberField>
          <MemberField wap :label="$t('member_user_00106')"><input v-model="workForm.sdate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('wap_00040')"><input v-model="workForm.edate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('ui.detail')" area><textarea v-model="workForm.content" rows="3" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ workForm.id ? $t('common.save') : $t('ui.add_work') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeExpItem
          v-for="row in workRows"
          :key="'w-' + row.id"
          surface="pc"
          :title="String(row.name || '')"
          :sub="row.title"
          :time="timeOf(row)"
          :body="row.content"
          @edit="fillWork(row)"
          @remove="delChild('works', row, refreshWorks)"
        />
        <MemberResumeSection :title="$t('wap_00459')" icon="yun_resume_h1_iconjy" h5-kind="edu" :open="openSec === 'edu'" @toggle="openAdd('edu')">
          <template #pc>
            <p v-if="!eduRows.length" class="muted">{{ $t('ui.no_edu') }}</p>
          </template>
          <template #h5>
            <MemberResumeExpItem
              v-for="row in eduRows"
              :key="'h5-edu-' + row.id"
              surface="h5"
              :title="String(row.name || '')"
              :sub="row.specialty || row.education_n"
              :time="timeOf(row)"
              @edit="fillEdu(row)"
              @remove="delChild('edus', row, refreshEdus)"
            />
          </template>
          <template #form>
        <form @submit.prevent="saveChild('edus', { ...eduForm }, refreshEdus)">
          <MemberField wap :label="$t('ui.edu')"><input v-model="eduForm.name" /></MemberField>
          <MemberField wap :label="$t('admin_user_00224')"><input v-model="eduForm.specialty" /></MemberField>
          <MemberField wap :label="$t('member_user_00106')"><input v-model="eduForm.sdate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('wap_00040')"><input v-model="eduForm.edate_n" placeholder="YYYY-MM" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ eduForm.id ? $t('common.save') : $t('ui.add_edu') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeExpItem
          v-for="row in eduRows"
          :key="'edu-' + row.id"
          surface="pc"
          :title="String(row.name || '')"
          :sub="row.specialty || row.education_n"
          :time="timeOf(row)"
          @edit="fillEdu(row)"
          @remove="delChild('edus', row, refreshEdus)"
        />
        <MemberResumeSection :title="$t('wap_00465')" icon="yun_resume_h1_iconxm" :open="openSec === 'project'" @toggle="openAdd('project')">
          <template #pc>
            <p v-if="!projectRows.length" class="muted">{{ $t('ui.no_items') }}</p>
          </template>
          <template #h5>
            <MemberResumeExpItem
              v-for="row in projectRows"
              :key="'h5-p-' + row.id"
              surface="h5"
              :title="String(row.name || '')"
              :sub="row.role"
              :time="timeOf(row)"
              :body="row.content"
              @edit="fillProject(row)"
              @remove="delChild('projects', row, refreshProjects)"
            />
          </template>
          <template #form>
        <form @submit.prevent="saveChild('projects', { ...projectForm }, refreshProjects)">
          <MemberField wap :label="$t('wap_user_00099')"><input v-model="projectForm.name" /></MemberField>
          <MemberField wap :label="$t('wap_com_00288')"><input v-model="projectForm.role" /></MemberField>
          <MemberField wap :label="$t('member_user_00106')"><input v-model="projectForm.sdate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('wap_00040')"><input v-model="projectForm.edate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('ui.detail')" area><textarea v-model="projectForm.content" rows="3" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ projectForm.id ? $t('common.save') : $t('common.submit') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeExpItem
          v-for="row in projectRows"
          :key="'p-' + row.id"
          surface="pc"
          :title="String(row.name || '')"
          :sub="row.role"
          :time="timeOf(row)"
          :body="row.content"
          @edit="fillProject(row)"
          @remove="delChild('projects', row, refreshProjects)"
        />
        <MemberResumeSection :title="$t('wap_00461')" icon="yun_resume_h1_iconjn" h5-kind="skill" :open="openSec === 'skill'" @toggle="openAdd('skill')">
          <template #pc>
            <p v-if="!skillRows.length" class="muted">{{ $t('ui.no_items') }}</p>
          </template>
          <template #h5>
            <div
              v-for="row in skillRows"
              :key="'h5-sk-' + row.id"
              class="cord_intention_bom"
              @click="fillSkill(row)"
            >
              <div class="data_left_skill">
                <ul>
                  <li class="cord_intention_jnmane">{{ row.name }}</li>
                  <li v-if="row.years">{{ row.years }}</li>
                </ul>
              </div>
            </div>
          </template>
          <template #form>
        <form @submit.prevent="saveChild('skills', { ...skillForm }, refreshSkills)">
          <MemberField wap :label="$t('wap_00461')"><input v-model="skillForm.name" required /></MemberField>
          <MemberField wap :label="$t('common_02067')"><input v-model.number="skillForm.years" type="number" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ skillForm.id ? $t('common.save') : $t('common.submit') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeExpItem
          v-for="row in skillRows"
          :key="'sk-' + row.id"
          surface="pc"
          :title="String(row.name || '')"
          :sub="row.years ? String(row.years) : ''"
          @edit="fillSkill(row)"
          @remove="delChild('skills', row, refreshSkills)"
        />
        <MemberResumeSection :title="$t('wap_00455')" icon="yun_resume_h1_iconpx" :open="openSec === 'training'" @toggle="openAdd('training')">
          <template #pc>
            <p v-if="!trainingRows.length" class="muted">{{ $t('ui.no_items') }}</p>
          </template>
          <template #h5>
            <MemberResumeExpItem
              v-for="row in trainingRows"
              :key="'h5-tr-' + row.id"
              surface="h5"
              :title="String(row.name || '')"
              :sub="row.title"
              :time="timeOf(row)"
              :body="row.content"
              @edit="fillTraining(row)"
              @remove="delChild('trainings', row, refreshTrainings)"
            />
          </template>
          <template #form>
        <form @submit.prevent="saveChild('trainings', { ...trainingForm }, refreshTrainings)">
          <MemberField wap :label="$t('member_user_00077')"><input v-model="trainingForm.name" /></MemberField>
          <MemberField wap :label="$t('wap_com_00288')"><input v-model="trainingForm.title" /></MemberField>
          <MemberField wap :label="$t('member_user_00106')"><input v-model="trainingForm.sdate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('wap_00040')"><input v-model="trainingForm.edate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('ui.detail')" area><textarea v-model="trainingForm.content" rows="3" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ trainingForm.id ? $t('common.save') : $t('member_user_00077') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeExpItem
          v-for="row in trainingRows"
          :key="'tr-' + row.id"
          surface="pc"
          :title="String(row.name || '')"
          :sub="row.title"
          :time="timeOf(row)"
          :body="row.content"
          @edit="fillTraining(row)"
          @remove="delChild('trainings', row, refreshTrainings)"
        />
        <MemberResumeSection :title="$t('wap_user_00090')" icon="yun_resume_h1_iconry" :open="openSec === 'cert'" @toggle="openAdd('cert')">
          <template #pc>
            <p v-if="!certRows.length" class="muted">{{ $t('ui.no_items') }}</p>
          </template>
          <template #h5>
            <MemberResumeExpItem
              v-for="row in certRows"
              :key="'h5-c-' + row.id"
              surface="h5"
              :title="String(row.name || '')"
              :sub="row.title"
              :time="timeOf(row)"
              :body="row.content"
              @edit="fillCert(row)"
              @remove="delChild('certs', row, refreshCerts)"
            />
          </template>
          <template #form>
        <form @submit.prevent="saveChild('certs', { ...certForm }, refreshCerts)">
          <MemberField wap :label="$t('wap_user_00090')"><input v-model="certForm.name" /></MemberField>
          <MemberField wap :label="$t('wap_com_00288')"><input v-model="certForm.title" /></MemberField>
          <MemberField wap :label="$t('member_user_00106')"><input v-model="certForm.sdate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('wap_00040')"><input v-model="certForm.edate_n" placeholder="YYYY-MM" /></MemberField>
          <MemberField wap :label="$t('ui.detail')" area><textarea v-model="certForm.content" rows="3" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ certForm.id ? $t('common.save') : $t('common.submit') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeExpItem
          v-for="row in certRows"
          :key="'c-' + row.id"
          surface="pc"
          :title="String(row.name || '')"
          :sub="row.title"
          :time="timeOf(row)"
          :body="row.content"
          @edit="fillCert(row)"
          @remove="delChild('certs', row, refreshCerts)"
        />
        <MemberResumeSection :title="$t('wap_00493')" icon="yun_resume_h1_iconqt" :open="openSec === 'other'" @toggle="openAdd('other')">
          <template #pc>
            <p v-if="!otherRows.length" class="muted">{{ $t('ui.no_items') }}</p>
          </template>
          <template #h5>
            <MemberResumeExpItem
              v-for="row in otherRows"
              :key="'h5-o-' + row.id"
              surface="h5"
              :title="String(row.name || '')"
              :body="row.content"
              @edit="fillOther(row)"
              @remove="delChild('others', row, refreshOthers)"
            />
          </template>
          <template #form>
        <form @submit.prevent="saveChild('others', { ...otherForm }, refreshOthers)">
          <MemberField wap :label="$t('member_user_00076')"><input v-model="otherForm.name" /></MemberField>
          <MemberField wap :label="$t('ui.detail')" area><textarea v-model="otherForm.content" rows="3" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ otherForm.id ? $t('common.save') : $t('member_user_00076') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeExpItem
          v-for="row in otherRows"
          :key="'o-' + row.id"
          surface="pc"
          :title="String(row.name || '')"
          :body="row.content"
          @edit="fillOther(row)"
          @remove="delChild('others', row, refreshOthers)"
        />
        <MemberResumeSection :title="$t('wap_com_00292')" icon="yun_resume_h1_iconpj" :open="openSec === 'language'" @toggle="openAdd('language')">
          <template #pc>
            <p v-if="!languageRows.length" class="muted">{{ $t('ui.no_items') }}</p>
          </template>
          <template #h5>
            <MemberResumeExpItem
              v-for="row in languageRows"
              :key="'h5-lg-' + row.id"
              surface="h5"
              :title="String(row.name || '')"
              :sub="row.level ? String(row.level) : ''"
              @edit="fillLanguage(row)"
              @remove="delChild('languages', row, refreshLanguages)"
            />
          </template>
          <template #form>
        <form @submit.prevent="saveChild('languages', { ...languageForm }, refreshLanguages)">
          <MemberField wap :label="$t('wap_com_00292')"><input v-model="languageForm.name" required /></MemberField>
          <MemberField wap :label="$t('wap_00459')"><input v-model.number="languageForm.level" type="number" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ languageForm.id ? $t('common.save') : $t('common.submit') }}</button>
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeExpItem
          v-for="row in languageRows"
          :key="'lg-' + row.id"
          surface="pc"
          :title="String(row.name || '')"
          :sub="row.level ? String(row.level) : ''"
          @edit="fillLanguage(row)"
          @remove="delChild('languages', row, refreshLanguages)"
        />
        <MemberResumeSection :title="$t('wap_00973')" icon="yun_resume_h1_iconzp" h5-kind="show" :open="openSec === 'show'" @toggle="openAdd('show')">
      <template #pc>
        <div v-for="row in shows?.list || []" :key="row.id" class="user_resume_box">
          <div class="user_resume_name">{{ row.title || row.id }}</div>
          <img v-if="row.picurl" :src="row.picurl" alt="" width="120" />
          <a href="javascript:;" class="user_resume_cz_a" @click="removeShow(row.id)">{{ $t('common.delete') }}</a>
        </div>
      </template>
      <template #h5>
        <div class="resume_min_body_Individual_works_photo">
          <ul>
            <li v-for="row in shows?.list || []" :key="'h5s-' + row.id">
              <img v-if="row.picurl" :src="mediaUrl(row.picurl)" alt="" width="100%" height="100%" />
            </li>
          </ul>
        </div>
      </template>
      <template #form>
        <form @submit.prevent>
          <MemberField wap :label="$t('wap_user_00103')"><input v-model="galleryTitle" /></MemberField>
          <input type="file" accept="image/jpeg,image/png,image/webp" @change="onShow" />
        </form>
      </template>
    </MemberResumeSection>
        <MemberResumeSection :title="$t('common.share')" icon="yun_resume_h1_iconfj" h5-kind="none" :open="openSec === 'share'" @toggle="toggleSec('share')">
      <template #pc>
        <div v-for="row in shareTokens?.list || []" :key="row.token" class="user_resume_box">
          <div class="user_resume_name">
            <NuxtLink :to="`/share/resume/${row.token}`">{{ row.token }}</NuxtLink>
          </div>
          <div class="user_resume_p">{{ row.view_count }} · {{ row.expires_at_n }}</div>
          <a v-if="row.active" href="javascript:;" class="user_resume_cz_a" @click="revokeShare(row.token)">{{ $t('common.delete') }}</a>
        </div>
      </template>
      <template #h5>
        <div v-for="row in shareTokens?.list || []" :key="'h5sh-' + row.token" class="work_list">
          <NuxtLink :to="`/share/resume/${row.token}`">{{ row.token }}</NuxtLink>
        </div>
      </template>
      <template #form>
        <form @submit.prevent="createShare">
          <MemberField wap :label="$t('member_user_00106')"><input v-model.number="shareTtl" type="number" min="60" max="2592000" /></MemberField>
          <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
        </form>
      </template>
    </MemberResumeSection>
      </div>
      <div class="site-h5 resume_bot">
        <div class="Edit_your_resume_tail">
          <div class="Edit_your_resume_Update_your_resume" @click="refreshResume">{{ $t('wap_user_00199') }}</div>
          <NuxtLink v-if="data?.uid" :to="`/resumes/${data.uid}`" class="Edit_your_resume_Preview_your_resume">{{ $t('wap_user_00217') }}</NuxtLink>
        </div>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
