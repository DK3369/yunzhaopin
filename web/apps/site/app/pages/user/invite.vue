<script setup lang="ts">
const api = useApi()
const form = reactive({ email: '', content: '邀请你注册本站' })
const msg = ref('')
async function send() {
  msg.value = ''
  try {
    const r = await api.post<{ invite_id: number }>('/v1/mcenter/invite-reg', { ...form })
    msg.value = `已发送 invite_id ${r.invite_id}`
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '发送失败'
  }
}
useSeoMeta({ title: '邀请注册' })
</script>

<template>
  <section>
    <h1>邀请注册</h1>
    <form class="form" @submit.prevent="send">
      <input v-model="form.email" type="email" placeholder="email" required />
      <textarea v-model="form.content" rows="4" placeholder="content" />
      <button type="submit">发送邀请</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
