<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const editId = computed(() => Number(useRoute().query.id || 0))
const form = reactive({
  name: '',
  job1: 0,
  job1_son: 0,
  job_post: 0,
  provinceid: 1,
  cityid: 1,
  three_cityid: 0,
  salary: 0,
  minsalary: 8000,
  maxsalary: 15000,
  type: 57,
  number: 1,
  exp: 0,
  edu: 0,
  content: '',
  sdate: 0,
  edate: 0,
})
const msg = ref('')
if (editId.value) {
  const row = await api.post<Record<string, unknown>>('/v1/mcenter/jobs/detail', { id: editId.value }).catch(() => null)
  if (row) {
    form.name = String(row.name || '')
    form.job1 = Number(row.job1 || 0)
    form.job1_son = Number(row.job1_son || 0)
    form.job_post = Number(row.job_post || 0)
    form.provinceid = Number(row.provinceid || 1)
    form.cityid = Number(row.cityid || 1)
    form.three_cityid = Number(row.three_cityid || 0)
    form.minsalary = Number(row.minsalary || 0)
    form.maxsalary = Number(row.maxsalary || 0)
    form.type = Number(row.type || 57)
    form.number = Number(row.number || 1)
    form.exp = Number(row.exp || 0)
    form.edu = Number(row.edu || 0)
    form.content = String(row.description || row.content || '')
  }
}
async function submit() {
  msg.value = ''
  try {
    if (editId.value) {
      await api.post('/v1/mcenter/jobs/update', { id: editId.value, ...form })
    } else {
      await api.post('/v1/mcenter/jobs', { ...form })
    }
    msg.value = t('common.success')
    await navigateTo('/com/jobs')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_00322') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00322') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.name" :placeholder="$t('wap_com_00288')" required />
      <input v-model.number="form.provinceid" type="number" :placeholder="$t('ui.province_id')" />
      <input v-model.number="form.cityid" type="number" :placeholder="$t('ui.city_id')" />
      <input v-model.number="form.minsalary" type="number" :placeholder="$t('ui.min_salary')" />
      <input v-model.number="form.maxsalary" type="number" :placeholder="$t('ui.max_salary')" />
      <select v-model.number="form.type">
        <option :value="57">{{ $t('ui.fulltime') }}</option>
        <option :value="58">{{ $t('wap_user_00220') }}</option>
        <option :value="59">{{ $t('ui.intern') }}</option>
        <option :value="60">{{ $t('ui.temporary') }}</option>
      </select>
      <input v-model.number="form.number" type="number" :placeholder="$t('ui.headcount')" />
      <textarea v-model="form.content" :placeholder="$t('ui.job_desc')" rows="8" />
      <button type="submit">{{ $t('ui.submit_audit') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
