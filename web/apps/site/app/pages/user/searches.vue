<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('saved-searches', () =>
  api.post('/v1/mcenter/saved-searches/list', { page: 1, page_size: 20 }),
)
const form = reactive({ name: '', kind: 'job', params: '{"keyword":""}', notify: true })
const msg = ref('')
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/saved-searches', {
      name: form.name,
      kind: form.kind,
      params: JSON.parse(form.params || '{}'),
      notify: form.notify,
    })
    msg.value = '已保存'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '保存失败'
  }
}
async function remove(id: number) {
  await api.post('/v1/mcenter/saved-searches/delete', { id })
  refresh()
}
useSeoMeta({ title: '搜索器' })
</script>

<template>
  <section>
    <h1>搜索器</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <form class="form" @submit.prevent="create">
      <input v-model="form.name" placeholder="名称" />
      <input v-model="form.kind" placeholder="kind" />
      <textarea v-model="form.params" rows="3" placeholder='params JSON，如 {"keyword":"java"}' />
      <label><input v-model="form.notify" type="checkbox" /> 通知 notify</label>
      <button type="submit">添加</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">暂无搜索器</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.name }}</h3>
        <p class="muted">kind {{ row.kind }} · notify {{ row.notify }}</p>
        <button type="button" @click="remove(row.id)">删除</button>
      </article>
    </div>
  </section>
</template>
