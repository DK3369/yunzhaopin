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
  years?: number
  level?: number
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
const form = reactive({
  name: '',
  sex: 1,
  birthday: '',
  telphone: '',
  email: '',
  photo: '',
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
  },
  { immediate: true },
)
const expectForm = reactive({ id: 0, name: '', salary: 8000, type: 57 })
const workForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', department: '', title: '' })
const eduForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', specialty: '', education: 65 })
const projectForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', role: '', content: '' })
const skillForm = reactive({ id: 0, name: '', level: 0, years: 0 })
const trainingForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', title: '', content: '' })
const certForm = reactive({ id: 0, name: '', sdate_n: '', edate_n: '', title: '', content: '' })
const otherForm = reactive({ id: 0, name: '', content: '' })
const languageForm = reactive({ id: 0, name: '', level: 0 })
const galleryTitle = ref('')
const msg = ref('')

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}
async function saveResume() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume', { ...form })
    msg.value = t('common.success')
    await refresh()
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
    await api.post('/v1/mcenter/resume/expects', { ...expectForm })
    msg.value = t('common.success')
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
function fillWork(row: ChildRow) {
  workForm.id = row.id
  workForm.name = String(row.name || '')
  workForm.title = String(row.title || '')
  workForm.department = String(row.department || '')
}
function fillEdu(row: ChildRow) {
  eduForm.id = row.id
  eduForm.name = String(row.name || '')
  eduForm.specialty = String(row.specialty || '')
}
function fillProject(row: ChildRow) {
  projectForm.id = row.id
  projectForm.name = String(row.name || '')
  projectForm.role = String(row.role || '')
  projectForm.content = String(row.content || '')
}
function fillSkill(row: ChildRow) {
  skillForm.id = row.id
  skillForm.name = String(row.name || '')
  skillForm.years = Number(row.years || 0)
  skillForm.level = Number(row.level || 0)
}
function fillTraining(row: ChildRow) {
  trainingForm.id = row.id
  trainingForm.name = String(row.name || '')
  trainingForm.title = String(row.title || '')
  trainingForm.content = String(row.content || '')
}
function fillCert(row: ChildRow) {
  certForm.id = row.id
  certForm.name = String(row.name || '')
  certForm.title = String(row.title || '')
  certForm.content = String(row.content || '')
}
function fillOther(row: ChildRow) {
  otherForm.id = row.id
  otherForm.name = String(row.name || '')
  otherForm.content = String(row.content || '')
}
function fillLanguage(row: ChildRow) {
  languageForm.id = row.id
  languageForm.name = String(row.name || '')
  languageForm.level = Number(row.level || 0)
}
useSeoMeta({ title: t('wap_user_00204') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00204') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form v-else class="form" @submit.prevent="saveResume">
      <img v-if="form.photo" :src="mediaUrl(form.photo)" alt="" width="72" height="72" />
      <input type="file" accept="image/jpeg,image/png,image/webp" @change="onAvatar" />
      <input v-model="form.name" :placeholder="$t('wap_00529')" />
      <select v-model.number="form.sex">
        <option :value="1">{{ $t('common_02092') }}</option>
        <option :value="2">{{ $t('common_02069') }}</option>
      </select>
      <input v-model="form.birthday" :placeholder="$t('ui.birthday')" />
      <input v-model="form.telphone" :placeholder="$t('common.phone')" />
      <input v-model="form.email" :placeholder="$t('member_user_00282')" />
      <button type="submit">{{ $t('ui.save_resume') }}</button>
      <button type="button" @click="refreshResume">{{ $t('wap_user_00199') }}</button>
    </form>
    <h2>{{ $t('home.intention') }}</h2>
    <p v-if="!(Array.isArray(expects) ? expects : []).length" class="muted">{{ $t('ui.no_expect') }}</p>
    <ul>
      <li v-for="row in Array.isArray(expects) ? expects : []" :key="row.id">{{ row.name || row.id }}</li>
    </ul>
    <form class="form" @submit.prevent="saveExpect">
      <input v-model="expectForm.name" :placeholder="$t('ui.intention_job')" />
      <input v-model.number="expectForm.salary" type="number" :placeholder="$t('ui.expect_salary')" />
      <button type="submit">{{ $t('ui.add_expect') }}</button>
    </form>
    <h2>{{ $t('wap_00457') }}</h2>
    <p v-if="!(Array.isArray(works) ? works : []).length" class="muted">{{ $t('ui.no_work') }}</p>
    <ul>
      <li v-for="row in Array.isArray(works) ? works : []" :key="row.id">
        {{ row.name }} {{ row.title }}
        <button type="button" @click="fillWork(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="delChild('works', row, refreshWorks)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <form class="form" @submit.prevent="saveChild('works', { ...workForm }, refreshWorks)">
      <input v-model="workForm.name" :placeholder="$t('common.company')" />
      <input v-model="workForm.title" :placeholder="$t('wap_com_00288')" />
      <input v-model="workForm.department" placeholder="department" />
      <input v-model="workForm.sdate_n" placeholder="sdate YYYY-MM" />
      <input v-model="workForm.edate_n" placeholder="edate YYYY-MM" />
      <button type="submit">{{ workForm.id ? $t('common.save') : $t('ui.add_work') }}</button>
    </form>
    <h2>{{ $t('wap_00459') }}</h2>
    <p v-if="!(Array.isArray(edus) ? edus : []).length" class="muted">{{ $t('ui.no_edu') }}</p>
    <ul>
      <li v-for="row in Array.isArray(edus) ? edus : []" :key="row.id">
        {{ row.name }} {{ row.specialty }}
        <button type="button" @click="fillEdu(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="delChild('edus', row, refreshEdus)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <form class="form" @submit.prevent="saveChild('edus', { ...eduForm }, refreshEdus)">
      <input v-model="eduForm.name" :placeholder="$t('ui.edu')" />
      <input v-model="eduForm.specialty" placeholder="specialty" />
      <input v-model="eduForm.sdate_n" placeholder="sdate YYYY-MM" />
      <input v-model="eduForm.edate_n" placeholder="edate YYYY-MM" />
      <button type="submit">{{ eduForm.id ? $t('common.save') : $t('ui.add_edu') }}</button>
    </form>
    <h2>{{ $t('wap_00465') }}</h2>
    <p v-if="!(Array.isArray(projects) ? projects : []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <ul>
      <li v-for="row in Array.isArray(projects) ? projects : []" :key="row.id">
        {{ row.name }} {{ row.role }}
        <button type="button" @click="fillProject(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="delChild('projects', row, refreshProjects)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <form class="form" @submit.prevent="saveChild('projects', { ...projectForm }, refreshProjects)">
      <input v-model="projectForm.name" :placeholder="$t('wap_com_00288')" />
      <input v-model="projectForm.role" />
      <input v-model="projectForm.sdate_n" placeholder="sdate YYYY-MM" />
      <input v-model="projectForm.edate_n" placeholder="edate YYYY-MM" />
      <textarea v-model="projectForm.content" rows="3" />
      <button type="submit">{{ projectForm.id ? $t('common.save') : $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('wap_00461') }}</h2>
    <p v-if="!(Array.isArray(skills) ? skills : []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <ul>
      <li v-for="row in Array.isArray(skills) ? skills : []" :key="row.id">
        {{ row.name }}
        <button type="button" @click="fillSkill(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="delChild('skills', row, refreshSkills)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <form class="form" @submit.prevent="saveChild('skills', { ...skillForm }, refreshSkills)">
      <input v-model="skillForm.name" required />
      <input v-model.number="skillForm.years" type="number" />
      <button type="submit">{{ skillForm.id ? $t('common.save') : $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('wap_00455') }}</h2>
    <p v-if="!(Array.isArray(trainings) ? trainings : []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <ul>
      <li v-for="row in Array.isArray(trainings) ? trainings : []" :key="row.id">
        {{ row.name }} {{ row.title }}
        <button type="button" @click="fillTraining(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="delChild('trainings', row, refreshTrainings)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <form class="form" @submit.prevent="saveChild('trainings', { ...trainingForm }, refreshTrainings)">
      <input v-model="trainingForm.name" :placeholder="$t('member_user_00077')" />
      <input v-model="trainingForm.title" />
      <input v-model="trainingForm.sdate_n" placeholder="sdate YYYY-MM" />
      <input v-model="trainingForm.edate_n" placeholder="edate YYYY-MM" />
      <textarea v-model="trainingForm.content" rows="3" />
      <button type="submit">{{ trainingForm.id ? $t('common.save') : $t('member_user_00077') }}</button>
    </form>
    <h2>{{ $t('wap_user_00090') }}</h2>
    <p v-if="!(Array.isArray(certs) ? certs : []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <ul>
      <li v-for="row in Array.isArray(certs) ? certs : []" :key="row.id">
        {{ row.name }} {{ row.title }}
        <button type="button" @click="fillCert(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="delChild('certs', row, refreshCerts)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <form class="form" @submit.prevent="saveChild('certs', { ...certForm }, refreshCerts)">
      <input v-model="certForm.name" :placeholder="$t('wap_user_00090')" />
      <input v-model="certForm.title" />
      <input v-model="certForm.sdate_n" placeholder="sdate YYYY-MM" />
      <input v-model="certForm.edate_n" placeholder="edate YYYY-MM" />
      <textarea v-model="certForm.content" rows="3" />
      <button type="submit">{{ certForm.id ? $t('common.save') : $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('wap_00493') }}</h2>
    <p v-if="!(Array.isArray(others) ? others : []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <ul>
      <li v-for="row in Array.isArray(others) ? others : []" :key="row.id">
        {{ row.name }}
        <button type="button" @click="fillOther(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="delChild('others', row, refreshOthers)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <form class="form" @submit.prevent="saveChild('others', { ...otherForm }, refreshOthers)">
      <input v-model="otherForm.name" :placeholder="$t('member_user_00076')" />
      <textarea v-model="otherForm.content" rows="3" />
      <button type="submit">{{ otherForm.id ? $t('common.save') : $t('member_user_00076') }}</button>
    </form>
    <h2>{{ $t('wap_com_00292') }}</h2>
    <p v-if="!(Array.isArray(languages) ? languages : []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <ul>
      <li v-for="row in Array.isArray(languages) ? languages : []" :key="row.id">
        {{ row.name }}
        <button type="button" @click="fillLanguage(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="delChild('languages', row, refreshLanguages)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <form class="form" @submit.prevent="saveChild('languages', { ...languageForm }, refreshLanguages)">
      <input v-model="languageForm.name" required />
      <input v-model.number="languageForm.level" type="number" />
      <button type="submit">{{ languageForm.id ? $t('common.save') : $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('wap_user_00157') }}</h2>
    <form class="form" @submit.prevent>
      <input v-model="galleryTitle" />
      <input type="file" accept="image/jpeg,image/png,image/webp" @change="onShow" />
    </form>
    <article v-for="row in shows?.list || []" :key="row.id" class="job-card">
      <h3>{{ row.title || row.id }}</h3>
      <img v-if="row.picurl" :src="row.picurl" alt="" width="120" />
      <button type="button" @click="removeShow(row.id)">{{ $t('common.delete') }}</button>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
