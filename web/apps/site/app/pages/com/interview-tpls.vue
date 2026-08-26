<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('interview-tpls', () =>
  api.post('/v1/mcenter/interview-templates/list', {}),
)
const form = reactive({
  name: '',
  content: '',
  address: '',
  linkman: '',
  linktel: '',
  intertime: 0,
})
const msg = ref('')
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as Array<Record<string, unknown>>)
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/interview-templates', { ...form })
    msg.value = '已添加'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '添加失败'
  }
}
useSeoMeta({ title: '面试模板' })
</script>

<template>
  <section>
    <h1>面试模板</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <form class="form" @submit.prevent="create">
      <input v-model="form.name" placeholder="name" />
      <textarea v-model="form.content" rows="4" placeholder="content" />
      <input v-model="form.address" placeholder="address" />
      <input v-model="form.linkman" placeholder="linkman" />
      <input v-model="form.linktel" placeholder="linktel" />
      <button type="submit">添加</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!list.length" class="muted">暂无面试模板</p>
    <div class="stack">
      <article v-for="row in list" :key="String(row.id)" class="job-card">
        <h3>{{ row.name }}</h3>
        <p class="muted">{{ row.address }} · {{ row.linkman }} {{ row.linktel }}</p>
      </article>
    </div>
  </section>
</template>
