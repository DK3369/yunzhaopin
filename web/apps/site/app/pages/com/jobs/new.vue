<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
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
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/jobs', { ...form })
    msg.value = t('ui.submit_audit')
    await navigateTo('/com/jobs')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.publish_job') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.publish_job') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.name" :placeholder="$t('ui.job_name')" required />
      <input v-model.number="form.provinceid" type="number" :placeholder="$t('ui.province_id')" />
      <input v-model.number="form.cityid" type="number" :placeholder="$t('ui.city_id')" />
      <input v-model.number="form.minsalary" type="number" :placeholder="$t('ui.min_salary')" />
      <input v-model.number="form.maxsalary" type="number" :placeholder="$t('ui.max_salary')" />
      <select v-model.number="form.type">
        <option :value="57">{{ $t('ui.fulltime') }}</option>
        <option :value="58">{{ $t('ui.parttime') }}</option>
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
