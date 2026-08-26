<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('com-tpls', () =>
  api.post<Array<Record<string, unknown>>>('/v1/mcenter/company-tpls', {}),
)
const msg = ref('')
async function apply(row: { id: number }) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-tpls/apply', { id: row.id })
    msg.value = '已应用'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '失败'
  }
}
useSeoMeta({ title: '企业模板' })
</script>

<template>
  <section>
    <h1>企业模板</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <p v-if="msg">{{ msg }}</p>
    <div class="stack">
      <article v-for="row in data || []" :key="row.id">
        <h3>{{ row.name }}</h3>
        <p class="muted">kind {{ row.kind }} status {{ row.status }}</p>
        <button type="button" @click="apply(row)">应用</button>
      </article>
    </div>
    <p><NuxtLink to="/com">返回企业中心</NuxtLink></p>
  </section>
</template>
