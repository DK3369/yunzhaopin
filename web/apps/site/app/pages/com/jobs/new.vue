<script setup lang="ts">
const api = useApi()
const form = reactive({
  name: '',
  job1: 0,
  minsalary: 0,
  maxsalary: 0,
  content: '',
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
      <input v-model="form.name" placeholder="职位名称" />
      <input v-model.number="form.minsalary" placeholder="最低薪" type="number" />
      <input v-model.number="form.maxsalary" placeholder="最高薪" type="number" />
      <textarea v-model="form.content" placeholder="职位描述" rows="8" />
      <button type="submit">提交</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
