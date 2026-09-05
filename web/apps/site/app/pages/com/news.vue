<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({ title: t('ui.com_news') })
</script>

<template>
  <CompanyContentManager kind="news" :title="$t('ui.com_news')" />
</template>
