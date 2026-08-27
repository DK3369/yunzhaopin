<script setup lang="ts">
definePageMeta({ layout: 'blank' })
const { t } = useI18n()
const username = ref('')
const password = ref('')
const err = ref('')
async function submit() {
  err.value = ''
  try {
    const me = await $fetch<{ uid: number; usertype: number; path?: string }>(bffUrl('/api/auth/admin-login'), {
      method: 'POST',
      credentials: 'include',
      body: { username: username.value, password: password.value },
    })
    if (me.usertype !== 3) {
      err.value = t('ui.need_admin')
      await $fetch(bffUrl('/api/auth/logout'), { method: 'POST', credentials: 'include' })
      return
    }
    await navigateTo(me.path || '/')
  } catch (e: unknown) {
    const anyErr = e as { data?: { msg?: string; statusMessage?: string }; statusMessage?: string }
    err.value = anyErr?.data?.msg || anyErr?.statusMessage || (e instanceof Error ? e.message : t('ui.login_failed'))
  }
}
</script>

<template>
  <el-card>
    <div style="display: flex; justify-content: space-between; align-items: center">
      <h1>{{ $t('ui.admin_login') }}</h1>
      <LangSwitch cookie-key="admin_lang" />
    </div>
    <el-form @submit.prevent="submit">
      <el-form-item>
        <el-input v-model="username" :placeholder="$t('ui.username')" />
      </el-form-item>
      <el-form-item>
        <el-input v-model="password" type="password" :placeholder="$t('ui.password')" />
      </el-form-item>
      <el-button type="primary" native-type="submit">{{ $t('common.login') }}</el-button>
      <p v-if="err">{{ err }}</p>
    </el-form>
  </el-card>
</template>
