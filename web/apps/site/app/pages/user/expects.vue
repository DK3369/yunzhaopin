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
    <article v-for="row in list" :key="row.id" class="look_resume_list">
      <h3>{{ row.name }}</h3>
      <p class="muted">{{ row.job_classid_n }} · {{ row.city_classid_n }}</p>
      <input v-model="row.name" />
      <button type="button" @click="save(row)">{{ $t('common.save') }}</button>
      <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
    </article>
    <form class="form" @submit.prevent="add">
      <input v-model="form.name" :placeholder="$t('ui.intention_job')" required />
      <input v-model.number="form.salary" type="number" :placeholder="$t('ui.expect_salary')" />
      <button type="submit">{{ $t('ui.add_expect') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
