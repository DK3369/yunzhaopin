<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({ title: t('wap_user_00196') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00196') }}</h1>
    <div class="stack">
      <NuxtLink to="/com/jobs" class="job-card">{{ $t('wap_com_00106') }}</NuxtLink>
      <NuxtLink to="/com/pay" class="job-card">{{ $t('common_01946') }}</NuxtLink>
      <NuxtLink to="/com/cert" class="job-card">{{ $t('wap_com_00075') }}</NuxtLink>
      <NuxtLink to="/com/binding" class="job-card">{{ $t('member_user_00059') }}</NuxtLink>
      <NuxtLink to="/user/account" class="job-card">{{ $t('member_com_00538') }}</NuxtLink>
      <NuxtLink to="/com/set" class="job-card">{{ $t('wap_user_00214') }}</NuxtLink>
      <NuxtLink to="/com/job-messages" class="job-card">{{ $t('member_user_00115') }}</NuxtLink>
      <NuxtLink to="/com/jobs" class="job-card">{{ $t('ui.poster') }}</NuxtLink>
    </div>
  </section>
</template>
