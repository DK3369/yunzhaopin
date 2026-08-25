<script setup lang="ts">
definePageMeta({ layout: 'blank' })
const username = ref('')
const password = ref('')
const err = ref('')
async function submit() {
  err.value = ''
  try {
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/login', {
      method: 'POST',
      body: { username: username.value, password: password.value },
    })
    if (me.usertype !== 3) {
      err.value = '需要管理员账号'
      await $fetch('/api/auth/logout', { method: 'POST' })
      return
    }
    await navigateTo('/')
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : '登录失败'
  }
}
</script>

<template>
  <el-card>
    <h1>管理员登录</h1>
    <el-form @submit.prevent="submit">
      <el-form-item><el-input v-model="username" placeholder="用户名" /></el-form-item>
      <el-form-item><el-input v-model="password" type="password" placeholder="密码" /></el-form-item>
      <el-button type="primary" native-type="submit">登录</el-button>
      <p v-if="err">{{ err }}</p>
    </el-form>
  </el-card>
</template>
