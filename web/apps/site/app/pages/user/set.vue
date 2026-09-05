<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({ title: t('wap_user_00214') })
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await navigateTo('/login')
}
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00214') }}</h1>
    <div class="stack">
      <NuxtLink to="/user/resume" class="job-card">{{ $t('wap_user_00341') }}</NuxtLink>
      <NuxtLink to="/user/ident" class="job-card">{{ $t('wap_user_00340') }}</NuxtLink>
      <NuxtLink to="/user/password" class="job-card">{{ $t('wap_00817') }}</NuxtLink>
      <NuxtLink to="/user/privacy" class="job-card">{{ $t('wap_user_00215') }}</NuxtLink>
      <NuxtLink to="/user/binding" class="job-card">{{ $t('wap_00389') }}</NuxtLink>
      <NuxtLink to="/user/account" class="job-card">{{ $t('wap_user_00338') }}</NuxtLink>
      <NuxtLink to="/user/searches" class="job-card">{{ $t('member_user_00108') }}</NuxtLink>
      <NuxtLink to="/user/outbox" class="job-card">{{ $t('member_user_00188') }}</NuxtLink>
      <NuxtLink to="/advice" class="job-card">{{ $t('wap_user_00203') }}</NuxtLink>
    </div>
    <p>
      <button type="button" @click="logout">{{ $t('wap_user_00342') }}</button>
    </p>
  </section>
</template>
