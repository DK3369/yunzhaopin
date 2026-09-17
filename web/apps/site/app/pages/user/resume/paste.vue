<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const route = useRoute()
const { data: dicts } = await usePublicDicts()
const editId = computed(() => Number(route.query.id || 0))
const industries = computed(() => dicts.value?.industries ?? [])
const jobTypes = computed(() => dicts.value?.job_types_user ?? [])
const reports = computed(() => dicts.value?.reports_user ?? [])
const form = reactive({
  id: 0,
  name: '',
  hy: 0,
  job_classid: 0,
  city_classid: 0,
  salary: 0,
  maxsalary: 0,
  type: 0,
  report: 0,
  jobstatus: 0,
  doc: '',
})
const msg = ref('')
const { error } = await useAsyncData(`resume-paste-${editId.value || 'new'}`, async () => {
  if (!editId.value) return null
  const r = await api.post<{
    expect?: {
      id?: number
      name?: string
      hy?: number
      job_classid?: number
      city_classid?: number
      salary?: number
      type?: number
      report?: number
      jobstatus?: number
    }
    doc?: string
  }>('/v1/mcenter/resume/paste/get', { id: editId.value })
  const e = r.expect
  if (e) {
    form.id = Number(e.id || editId.value)
    form.name = String(e.name || '')
    form.hy = Number(e.hy || 0)
    form.job_classid = Number(e.job_classid || 0)
    form.city_classid = Number(e.city_classid || 0)
    form.salary = Number(e.salary || 0)
    form.type = Number(e.type || 0)
    form.report = Number(e.report || 0)
    form.jobstatus = Number(e.jobstatus || 0)
  }
  form.doc = String(r.doc || '')
  return r
})

async function save() {
  msg.value = ''
  try {
    const r = await api.post<{ id?: number }>('/v1/mcenter/resume/paste', { ...form })
    msg.value = t('common.success')
    const id = Number(r.id || form.id || 0)
    if (id && id !== form.id) {
      await navigateTo(`/user/resume/paste?id=${id}`)
      return
    }
    await navigateTo('/user/resume')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('ui.paste_resume') })
</script>

<template>
  <MemberPanel
    :title="$t('ui.paste_resume')"
    user-title="h1"
    user-wrap="none"
    :error="error && !isUnauthErr(error) ? error : undefined"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <form v-else class="site-pc verification_form" @submit.prevent="save">
      <MemberField :label="$t('ui.intention_job')" required>
        <input v-model="form.name" required />
      </MemberField>
      <MemberField :label="$t('admin_user_company_00373')">
        <select v-model.number="form.hy">
          <option :value="0">{{ $t('common.not_limited') }}</option>
          <option v-for="h in industries" :key="h.id" :value="h.id">{{ h.name }}</option>
        </select>
      </MemberField>
      <MemberField :label="$t('ui.expect_salary')">
        <input v-model.number="form.salary" type="number" min="0" />
        <input v-model.number="form.maxsalary" type="number" min="0" />
      </MemberField>
      <MemberField :label="$t('ui.work_nature')">
        <select v-model.number="form.type">
          <option :value="0">{{ $t('common.not_limited') }}</option>
          <option v-for="row in jobTypes" :key="row.id" :value="row.id">{{ row.name }}</option>
        </select>
      </MemberField>
      <MemberField :label="$t('ui.report_time')">
        <select v-model.number="form.report">
          <option :value="0">{{ $t('common.not_limited') }}</option>
          <option v-for="row in reports" :key="row.id" :value="row.id">{{ row.name }}</option>
        </select>
      </MemberField>
      <MemberField :label="$t('ui.paste_body')" area>
        <RichEditor v-model="form.doc" />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('common.save') }}</button>
    </form>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="save">
        <MemberField wap :label="$t('ui.intention_job')">
          <input v-model="form.name" required />
        </MemberField>
        <MemberField wap :label="$t('admin_user_company_00373')">
          <select v-model.number="form.hy">
            <option :value="0">{{ $t('common.not_limited') }}</option>
            <option v-for="h in industries" :key="'h5hy-' + h.id" :value="h.id">{{ h.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('ui.expect_salary')">
          <input v-model.number="form.salary" type="number" min="0" />
        </MemberField>
        <MemberField wap :label="$t('ui.work_nature')">
          <select v-model.number="form.type">
            <option :value="0">{{ $t('common.not_limited') }}</option>
            <option v-for="row in jobTypes" :key="'h5t-' + row.id" :value="row.id">{{ row.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('ui.report_time')">
          <select v-model.number="form.report">
            <option :value="0">{{ $t('common.not_limited') }}</option>
            <option v-for="row in reports" :key="'h5r-' + row.id" :value="row.id">{{ row.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('ui.paste_body')" area>
          <RichEditor v-model="form.doc" />
        </MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.save') }}</button>
      </form>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
