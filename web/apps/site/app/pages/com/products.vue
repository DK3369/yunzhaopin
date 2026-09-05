<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({ title: t('ui.com_products') })
</script>

<template>
  <CompanyContentManager kind="product" :title="$t('ui.com_products')" />
</template>
