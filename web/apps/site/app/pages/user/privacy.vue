<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('privacy-resume', () =>
  api.post<{ status?: number; nametype?: number }>('/v1/mcenter/resume/list', {}),
)
const status = ref(1)
watch(
  data,
  (row) => {
    if (!row) return
    status.value = Number(row.status || 1)
  },
  { immediate: true },
)
const msg = ref('')
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/status', { status: status.value })
    msg.value = '隐私设置已保存'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '保存失败'
  }
}
useSeoMeta({ title: '简历隐私' })
</script>

<template>
  <section>
    <h1>简历隐私</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <form v-else class="form" @submit.prevent="save">
      <p class="muted">对应 PHP 简历公开状态 <code>status</code>：1 公开 / 2 隐藏 / 3 仅投递企业可见。</p>
      <select v-model.number="status">
        <option :value="1">公开</option>
        <option :value="2">隐藏</option>
        <option :value="3">仅投递企业可见</option>
      </select>
      <button type="submit">保存</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
