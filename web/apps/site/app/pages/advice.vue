<script setup lang="ts">
const api = useApi()
const form = reactive({ infotype: '建议', content: '', moblie: '' })
const msg = ref('')
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/wap/advice', { ...form })
    msg.value = '已提交'
    form.content = ''
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '提交失败'
  }
}
useSeoMeta({ title: '意见建议' })
</script>

<template>
  <section>
    <h1>意见建议</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.infotype" placeholder="infotype" />
      <textarea v-model="form.content" rows="5" placeholder="content" required />
      <input v-model="form.moblie" placeholder="moblie（PHP 字段名）" />
      <button type="submit">提交</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
