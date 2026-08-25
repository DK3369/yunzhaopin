<script setup lang="ts">
const api = useApi()
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
    msg.value = '已提交审核'
    await navigateTo('/com/jobs')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '发布失败'
  }
}
useSeoMeta({ title: '发布职位' })
</script>

<template>
  <section>
    <h1>发布职位</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.name" placeholder="职位名称" required />
      <input v-model.number="form.provinceid" type="number" placeholder="省份 id" />
      <input v-model.number="form.cityid" type="number" placeholder="城市 id" />
      <input v-model.number="form.minsalary" type="number" placeholder="最低薪" />
      <input v-model.number="form.maxsalary" type="number" placeholder="最高薪" />
      <select v-model.number="form.type">
        <option :value="57">全职</option>
        <option :value="58">兼职</option>
        <option :value="59">实习</option>
        <option :value="60">临时</option>
      </select>
      <input v-model.number="form.number" type="number" placeholder="招聘人数" />
      <textarea v-model="form.content" placeholder="职位描述" rows="8" />
      <button type="submit">提交审核</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
