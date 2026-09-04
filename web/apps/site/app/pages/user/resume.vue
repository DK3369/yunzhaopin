<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

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
const form = reactive({
  name: '',
  sex: 1,
  birthday: '',
  telphone: '',
  email: '',
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
  },
  { immediate: true },
)
const expectForm = reactive({ name: '', salary: 8000, type: 57 })
const workForm = reactive({ name: '', sdate_n: '', edate_n: '', department: '', title: '' })
const eduForm = reactive({ name: '', sdate_n: '', edate_n: '', specialty: '', education: 65 })
const projectForm = reactive({ name: '', sdate_n: '', edate_n: '', role: '', content: '' })
const skillForm = reactive({ name: '', level: 0, years: 0 })
const msg = ref('')
async function saveResume() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume', { ...form })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function refreshResume() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/refresh', {})
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function saveExpect() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/expects', { ...expectForm })
    msg.value = t('common.success')
    await refreshExpects()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function saveWork() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/works', { ...workForm })
    msg.value = t('common.success')
    await refreshWorks()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function saveEdu() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/edus', { ...eduForm })
    msg.value = t('common.success')
    await refreshEdus()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function saveProject() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/projects', { ...projectForm })
    msg.value = t('common.success')
    await refreshProjects()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function saveSkill() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/skills', { ...skillForm })
    msg.value = t('common.success')
    await refreshSkills()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.my_resume') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.my_resume') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form v-else class="form" @submit.prevent="saveResume">
      <input v-model="form.name" :placeholder="$t('ui.fullname')" />
      <select v-model.number="form.sex">
        <option :value="1">{{ $t('common_02092') }}</option>
        <option :value="2">{{ $t('common_02069') }}</option>
      </select>
      <input v-model="form.birthday" :placeholder="$t('ui.birthday')" />
      <input v-model="form.telphone" :placeholder="$t('common.phone')" />
      <input v-model="form.email" :placeholder="$t('ui.email_addr')" />
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
    <h2>{{ $t('ui.work_exp') }}</h2>
    <p v-if="!(Array.isArray(works) ? works : []).length" class="muted">{{ $t('ui.no_work') }}</p>
    <ul>
      <li v-for="row in Array.isArray(works) ? works : []" :key="row.id">{{ row.name }} {{ row.title }}</li>
    </ul>
    <form class="form" @submit.prevent="saveWork">
      <input v-model="workForm.name" :placeholder="$t('common.company')" />
      <input v-model="workForm.title" :placeholder="$t('ui.job_name')" />
      <input v-model="workForm.department" placeholder="department" />
      <input v-model="workForm.sdate_n" placeholder="sdate YYYY-MM" />
      <input v-model="workForm.edate_n" placeholder="edate YYYY-MM" />
      <button type="submit">{{ $t('ui.add_work') }}</button>
    </form>
    <h2>{{ $t('ui.edu_exp') }}</h2>
    <p v-if="!(Array.isArray(edus) ? edus : []).length" class="muted">{{ $t('ui.no_edu') }}</p>
    <ul>
      <li v-for="row in Array.isArray(edus) ? edus : []" :key="row.id">{{ row.name }} {{ row.specialty }}</li>
    </ul>
    <form class="form" @submit.prevent="saveEdu">
      <input v-model="eduForm.name" :placeholder="$t('ui.edu')" />
      <input v-model="eduForm.specialty" placeholder="specialty" />
      <input v-model="eduForm.sdate_n" placeholder="sdate YYYY-MM" />
      <input v-model="eduForm.edate_n" placeholder="edate YYYY-MM" />
      <button type="submit">{{ $t('ui.add_edu') }}</button>
    </form>
    <h2>{{ $t('wap_00465') }}</h2>
    <p v-if="!(Array.isArray(projects) ? projects : []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <ul>
      <li v-for="row in Array.isArray(projects) ? projects : []" :key="row.id">{{ row.name }} {{ row.role }}</li>
    </ul>
    <form class="form" @submit.prevent="saveProject">
      <input v-model="projectForm.name" :placeholder="$t('ui.job_name')" />
      <input v-model="projectForm.role" />
      <input v-model="projectForm.sdate_n" placeholder="sdate YYYY-MM" />
      <input v-model="projectForm.edate_n" placeholder="edate YYYY-MM" />
      <textarea v-model="projectForm.content" rows="3" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <h2>{{ $t('wap_00461') }}</h2>
    <p v-if="!(Array.isArray(skills) ? skills : []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <ul>
      <li v-for="row in Array.isArray(skills) ? skills : []" :key="row.id">{{ row.name }}</li>
    </ul>
    <form class="form" @submit.prevent="saveSkill">
      <input v-model="skillForm.name" required />
      <input v-model.number="skillForm.years" type="number" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
