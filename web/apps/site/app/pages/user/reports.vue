<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('my-reports', () =>
  api.post('/v1/mcenter/reports/list', { page: 1, page_size: 20 }),
)
const form = reactive({ target_kind: 1, target_id: 0, reason_code: 'spam', detail: '' })
const msg = ref('')
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/reports', { ...form })
    msg.value = '已提交'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '提交失败'
  }
}
useSeoMeta({ title: '我的举报' })
</script>

<template>
  <section>
    <h1>举报</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <form class="form" @submit.prevent="submit">
      <select v-model.number="form.target_kind">
        <option :value="1">职位</option>
        <option :value="2">企业</option>
        <option :value="3">简历</option>
        <option :value="4">资讯</option>
        <option :value="5">用户</option>
      </select>
      <input v-model.number="form.target_id" type="number" placeholder="target_id" />
      <input v-model="form.reason_code" placeholder="reason_code" />
      <textarea v-model="form.detail" rows="3" placeholder="detail" />
      <button type="submit">提交</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">暂无举报</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>kind {{ row.target_kind }} #{{ row.target_id }}</h3>
        <p class="muted">status {{ row.status }}</p>
      </article>
    </div>
  </section>
</template>
