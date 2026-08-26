<script setup lang="ts">
const api = useApi()
const form = reactive({ uid: 0, code: '', username: '', password: '' })
const msg = ref('')
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/wap/claim', { ...form })
    msg.value = '认领已提交'
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '认领失败'
  }
}
useSeoMeta({ title: '企业认领' })
</script>

<template>
  <section>
    <h1>企业认领</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model.number="form.uid" type="number" placeholder="uid" required />
      <input v-model="form.code" placeholder="code" required />
      <input v-model="form.username" placeholder="username" required />
      <input v-model="form.password" type="password" placeholder="password" required />
      <button type="submit">认领</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
