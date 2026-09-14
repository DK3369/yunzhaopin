<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('expects', () => api.post('/v1/mcenter/resume/expects/list', {}))
const form = reactive({ name: '', salary: 8000, type: 57, job_classid: 0, city_classid: 0 })
const msg = ref('')
const list = computed(() => data.value?.list || data.value || [])
async function add() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/expects', { ...form })
    msg.value = t('common.success')
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
  <MemberPanel :title="$t('home.intention')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(list || []).length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div v-for="row in list" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <span class="user_new_jobname">{{ row.name }}</span>
        <div class="user_new_comname">{{ row.job_classid_n }} · {{ row.city_classid_n }}</div>
      </div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_a" @click="save(row)">{{ $t('common.save') }}</a>
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in list"
          :key="'h5-' + row.id"
          :title="row.name"
          :sub="`${row.job_classid_n || ''} ${row.city_classid_n || ''}`"
        />
      </div>
    </div>
    <form class="form verification_form" @submit.prevent="add">
      <MemberField :label="$t('ui.intention_job')">
        <input v-model="form.name" required />
      </MemberField>
      <MemberField :label="$t('ui.expect_salary')">
        <input v-model.number="form.salary" type="number" />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('ui.add_expect') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
