<script setup lang="ts">
const username = ref('')
const password = ref('')
const err = ref('')
async function submit() {
  err.value = ''
  try {
    await $fetch('/api/auth/login', {
      method: 'POST',
      body: { username: username.value, password: password.value },
    })
    await navigateTo('/user')
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || '登录失败'
  }
}
useSeoMeta({ title: '登录' })
</script>

<template>
  <section>
    <h1>登录</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="username" autocomplete="username" placeholder="用户名 / 手机 / 邮箱" />
      <input v-model="password" type="password" autocomplete="current-password" placeholder="密码" />
      <button type="submit">登录</button>
      <p v-if="err" class="muted">{{ err }}</p>
      <NuxtLink to="/register">没有账号？注册</NuxtLink>
    </form>
  </section>
</template>
