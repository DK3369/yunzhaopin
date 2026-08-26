<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('com-banners', () =>
  api.post<Array<Record<string, unknown>>>('/v1/mcenter/company-banners/list', {}),
)
const form = reactive({ pic: '', link: '', sort: 0 })
const msg = ref('')
async function add() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-banners', { ...form })
    form.pic = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '失败'
  }
}
async function remove(row: { id: number }) {
  await api.post('/v1/mcenter/company-banners/delete', { ids: [row.id] })
  await refresh()
}
useSeoMeta({ title: '企业 Banner' })
</script>

<template>
  <section>
    <h1>企业 Banner</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <form v-else class="form" @submit.prevent="add">
      <input v-model="form.pic" placeholder="pic" />
      <input v-model="form.link" placeholder="link" />
      <button type="submit">添加</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
    <div class="stack">
      <article v-for="row in data || []" :key="row.id">
        <p>{{ row.pic }}</p>
        <button type="button" @click="remove(row)">删除</button>
      </article>
    </div>
    <p><NuxtLink to="/com">返回企业中心</NuxtLink></p>
  </section>
</template>
