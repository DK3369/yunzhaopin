<script setup lang="ts">
const { t } = useI18n()
const usertype = computed(() => (Number(useRoute().query.usertype) === 2 ? 2 : 1))
useSeoMeta({ title: t('loginPage.tab_qr') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ usertype === 2 ? $t('loginPage.qr_hire_title') : $t('loginPage.qr_seek_title') }}</h1>
    <p>{{ usertype === 2 ? $t('loginPage.qr_hire_hint') : $t('loginPage.qr_seek_hint') }}</p>
    <p>
      <NuxtLink to="/download">{{ $t('ui.app_download') }}</NuxtLink>
      ·
      <NuxtLink to="/login">{{ $t('common.login') }}</NuxtLink>
    </p>
  </section>
</template>
