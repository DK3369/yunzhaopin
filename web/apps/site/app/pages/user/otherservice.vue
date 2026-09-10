<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({ title: t('wap_user_00196') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00196') }}</h1>
    <div class="stack">
      <NuxtLink to="/user/resume" class="job-card">{{ $t('wap_user_00204') }}</NuxtLink>
      <NuxtLink to="/user/pay" class="job-card">{{ $t('ui.pay') }}</NuxtLink>
      <NuxtLink to="/user/integral" class="job-card">{{ $t('wap_user_00213') }}</NuxtLink>
      <NuxtLink to="/user/binding" class="job-card">{{ $t('wap_00389') }}</NuxtLink>
      <NuxtLink to="/user/account" class="job-card">{{ $t('wap_user_00338') }}</NuxtLink>
      <NuxtLink to="/user/set" class="job-card">{{ $t('wap_user_00214') }}</NuxtLink>
      <NuxtLink to="/user/eval-logs" class="job-card">{{ $t('wap_00194') }}</NuxtLink>
      <NuxtLink to="/user/resume" class="job-card">{{ $t('common.share') }}</NuxtLink>
    </div>
  </section>
</template>
