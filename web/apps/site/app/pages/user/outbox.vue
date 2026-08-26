<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('resume-outbox', () =>
  api.post('/v1/mcenter/resume-outbox/list', { page: 1, page_size: 20 }),
)
const form = reactive({
  resume_id: 0,
  email: '',
  com_name: '',
  job_name: '',
  resume_name: '',
})
const msg = ref('')
async function send() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-outbox', { ...form })
    msg.value = '已发送'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '发送失败'
  }
}
useSeoMeta({ title: '简历外发' })
</script>

<template>
  <section>
    <h1>简历外发</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <form class="form" @submit.prevent="send">
      <input v-model.number="form.resume_id" type="number" placeholder="resume_id" />
      <input v-model="form.email" placeholder="email" />
      <input v-model="form.com_name" placeholder="com_name" />
      <input v-model="form.job_name" placeholder="job_name" />
      <input v-model="form.resume_name" placeholder="resume_name" />
      <button type="submit">发送</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">暂无外发记录</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.com_name }} · {{ row.job_name }}</h3>
        <p class="muted">{{ row.email }} · {{ row.addtime_n }}</p>
      </article>
    </div>
  </section>
</template>
