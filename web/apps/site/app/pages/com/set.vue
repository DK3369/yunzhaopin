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
      <NuxtLink to="/com/profile" class="job-card">{{ $t('wap_user_00341') }}</NuxtLink>
      <NuxtLink to="/com/addresses" class="job-card">{{ $t('wap_com_00402') }}</NuxtLink>
      <NuxtLink to="/com/cert" class="job-card">{{ $t('wap_user_00340') }}</NuxtLink>
      <NuxtLink to="/com/interview-tpls" class="job-card">{{ $t('wap_com_00404') }}</NuxtLink>
      <NuxtLink to="/com/gallery" class="job-card">{{ $t('wap_com_00401') }}</NuxtLink>
      <NuxtLink to="/com/binding" class="job-card">{{ $t('member_user_00059') }}</NuxtLink>
      <NuxtLink to="/com/password" class="job-card">{{ $t('wap_00817') }}</NuxtLink>
      <NuxtLink to="/advice" class="job-card">{{ $t('wap_user_00203') }}</NuxtLink>
    </div>
    <p>
      <button type="button" @click="logout">{{ $t('common.logout') }}</button>
    </p>
  </section>
</template>
