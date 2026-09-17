<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('user-home-bundle', () =>
  api.post<{ expects?: unknown }>('/v1/mcenter/resume/bundle', {}).catch(() => null),
)
const form = reactive({ id: 0, name: '', salary: 8000, type: 57, job_classid: 0, city_classid: 0 })
const msg = ref('')
const adding = ref(false)
type ExpectRow = {
  id: number
  name?: string
  job_classid?: number
  city_classid?: number
  job_class_n?: string
  city_class_n?: string
  salary?: number
  salary_n?: string
  type?: number
  report?: number
  jobstatus?: number
  hy?: number
}
function childList(v: unknown): ExpectRow[] {
  if (!v) return []
  if (Array.isArray(v)) return v as ExpectRow[]
  if (typeof v === 'object' && v && 'list' in v) return ((v as { list?: ExpectRow[] }).list || [])
  return []
}
const list = computed(() => childList((data.value as { expects?: unknown } | null)?.expects))
function resetForm() {
  Object.assign(form, { id: 0, name: '', salary: 8000, type: 57, job_classid: 0, city_classid: 0 })
}
function startAdd() {
  if (adding.value && !form.id) {
    adding.value = false
    return
  }
  resetForm()
  adding.value = true
}
function startEdit(row: ExpectRow) {
  form.id = row.id
  form.name = String(row.name || '')
  form.salary = Number(row.salary || 8000)
  form.job_classid = Number(row.job_classid || 0)
  form.city_classid = Number(row.city_classid || 0)
  form.type = Number(row.type || 57)
  adding.value = true
}
async function submit() {
  msg.value = ''
  try {
    if (form.id) {
      await api.post('/v1/mcenter/resume/expects/update', { ...form })
    } else {
      const first = list.value.length === 0
      await api.post('/v1/mcenter/resume/expects', { ...form })
      if (first) {
        await navigateTo('/user/resume/success')
        return
      }
    }
    msg.value = t('common.success')
    adding.value = false
    resetForm()
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function save(row: ExpectRow) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/expects/update', {
      id: row.id,
      name: row.name,
      job_classid: row.job_classid,
      city_classid: row.city_classid,
      salary: row.salary || 0,
      type: row.type,
      report: row.report,
      jobstatus: row.jobstatus,
      hy: row.hy,
    })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/expects/update', { id, status: 2 })
    msg.value = t('common.success')
    if (form.id === id) {
      adding.value = false
      resetForm()
    }
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('home.intention') })
</script>

<template>
  <MemberPanel :title="$t('home.intention')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="false">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <MemberResumeSection :title="$t('home.intention')" icon="yun_resume_h1_iconyx" h5-kind="none" :open="false">
      <template #pc>
        <div v-for="row in list" :key="row.id" class="user_resume_box">
          <div class="user_resume_info">
            <div class="user_resume_name">{{ row.name }}</div>
            <div class="user_resume_p">{{ row.job_class_n }} · {{ row.city_class_n }}</div>
          </div>
          <div class="user_resume_cz">
            <a href="javascript:;" class="user_resume_cz_a" @click="save(row)">{{ $t('common.save') }}</a>
            <a href="javascript:;" class="user_resume_cz_a" @click="remove(row.id)">{{ $t('common.delete') }}</a>
          </div>
        </div>
        <form class="verification_form" @submit.prevent="submit">
          <MemberField :label="$t('ui.intention_job')">
            <input v-model="form.name" required />
          </MemberField>
          <MemberField :label="$t('ui.expect_salary')">
            <input v-model.number="form.salary" type="number" />
          </MemberField>
          <button type="submit" class="verification_form_btn">{{ form.id ? $t('common.save') : $t('ui.add_expect') }}</button>
        </form>
      </template>
    </MemberResumeSection>
    <div class="site-h5 Edit_your_resume_min_body">
      <div class="resume_min_body_cord">
        <div class="resume_min_body_cord_intention">
          <div class="cord_intention_top">
            <div class="cord_intention_top_word">{{ $t('wap_00460') }}</div>
            <div class="cord_intention_top_icon" @click="startAdd">
              <img src="/legacy/h5/images/addition.png" alt="" width="100%" height="100%" />
            </div>
          </div>
          <div
            v-for="row in list"
            :key="'h5-' + row.id"
            class="cord_intention_bom"
            @click="startEdit(row)"
          >
            <div class="data_left_condition">
              <ul>
                <li>{{ row.name }}</li>
                <li v-if="row.salary_n">· {{ row.salary_n }}</li>
                <li v-if="row.job_class_n">· {{ row.job_class_n }}</li>
                <li v-if="row.city_class_n">· {{ row.city_class_n }}</li>
              </ul>
            </div>
            <div class="cord_intention_bom_icon">
              <img src="/legacy/h5/images/icon_more.png" alt="" width="100%" height="100%" />
            </div>
          </div>
        </div>
        <form v-if="adding || !list.length" class="yun_createbox" @submit.prevent="submit">
          <MemberField wap :label="$t('ui.intention_job')">
            <input v-model="form.name" required />
          </MemberField>
          <MemberField wap :label="$t('ui.expect_salary')">
            <input v-model.number="form.salary" type="number" />
          </MemberField>
          <button type="submit" class="Create_resume_btn">{{ form.id ? $t('common.save') : $t('ui.add_expect') }}</button>
          <p v-if="form.id" class="member-expect-del" @click.prevent="remove(form.id)">{{ $t('common.delete') }}</p>
        </form>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
