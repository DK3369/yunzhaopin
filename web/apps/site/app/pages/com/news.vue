<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('com-news', () =>
  api.post('/v1/mcenter/company/news/list', { page: 1, page_size: 20 }),
)
const form = reactive({ title: '', body: '', sort: 0 })
const msg = ref('')
const list = computed(() => data.value?.list || [])
async function add() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/news', { ...form })
    form.title = ''
    form.body = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '失败'
  }
}
useSeoMeta({ title: '企业新闻' })
</script>

<template>
  <section>
    <h1>企业新闻</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <form v-else class="form" @submit.prevent="add">
      <input v-model="form.title" placeholder="title" />
      <textarea v-model="form.body" placeholder="body" rows="4" />
      <button type="submit">发布</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
    <div class="stack">
      <article v-for="row in list" :key="row.id">
        <h3>{{ row.title }}</h3>
        <p class="muted">status {{ row.status }}</p>
      </article>
    </div>
    <p><NuxtLink to="/com">返回企业中心</NuxtLink></p>
  </section>
</template>
