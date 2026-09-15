<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('expects', () => api.post('/v1/mcenter/resume/expects/list', {}))
const form = reactive({ name: '', salary: 8000, type: 57, job_classid: 0, city_classid: 0 })
const msg = ref('')
const adding = ref(false)
type ExpectRow = {
  id: number
  name?: string
  job_classid?: number
  city_classid?: number
  job_classid_n?: string
  city_classid_n?: string
  salary?: number
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
const list = computed(() => childList(data.value))
async function add() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/expects', { ...form })
    msg.value = t('common.success')
    adding.value = false
    form.name = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function save(row: {
  id: number
  name?: string
  job_classid?: number
  city_classid?: number
  salary?: number
  type?: number
  report?: number
  jobstatus?: number
  hy?: number
}) {
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
            <div class="user_resume_p">{{ row.job_classid_n }} · {{ row.city_classid_n }}</div>
          </div>
          <div class="user_resume_cz">
            <a href="javascript:;" class="user_resume_cz_a" @click="save(row)">{{ $t('common.save') }}</a>
            <a href="javascript:;" class="user_resume_cz_a" @click="remove(row.id)">{{ $t('common.delete') }}</a>
          </div>
        </div>
        <form class="verification_form" @submit.prevent="add">
          <MemberField :label="$t('ui.intention_job')">
            <input v-model="form.name" required />
          </MemberField>
          <MemberField :label="$t('ui.expect_salary')">
            <input v-model.number="form.salary" type="number" />
          </MemberField>
          <button type="submit" class="verification_form_btn">{{ $t('ui.add_expect') }}</button>
        </form>
      </template>
    </MemberResumeSection>
    <div class="site-h5 Edit_your_resume_min_body">
      <div class="resume_min_body_cord">
        <div v-for="row in list" :key="'h5-' + row.id" class="resume_min_body_cord_intention">
          <div class="cord_intention_top">
            <div class="cord_intention_top_word">{{ $t('wap_00460') }}</div>
          </div>
          <div class="cord_intention_bom">
            <div class="data_left_condition">
              <ul>
                <li>{{ row.name }}</li>
                <li v-if="row.job_classid_n">{{ row.job_classid_n }}</li>
                <li v-if="row.city_classid_n">{{ row.city_classid_n }}</li>
              </ul>
            </div>
            <div class="cord_intention_bom_icon" @click="remove(row.id)">
              <img src="/legacy/h5/images/icon_more.png" alt="" width="100%" height="100%" />
            </div>
          </div>
        </div>
        <div class="cord_work_experience_one" @click="adding = !adding">
          <div class="cord_intention_top_word">{{ $t('ui.add_expect') }}</div>
          <div class="cord_intention_top_icon">
            <img src="/legacy/h5/images/addition.png" alt="" width="100%" height="100%" />
          </div>
        </div>
        <form v-if="adding || !list.length" class="yun_createbox" @submit.prevent="add">
          <MemberField wap :label="$t('ui.intention_job')">
            <input v-model="form.name" required />
          </MemberField>
          <MemberField wap :label="$t('ui.expect_salary')">
            <input v-model.number="form.salary" type="number" />
          </MemberField>
          <button type="submit" class="Create_resume_btn">{{ $t('ui.add_expect') }}</button>
        </form>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
