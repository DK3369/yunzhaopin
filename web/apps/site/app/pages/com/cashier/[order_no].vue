<script setup lang="ts">
const { t } = useI18n()
const orderNo = String(useRoute().params.order_no || '')
useSeoMeta({ title: t('wap_user_00313') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00313')">
    <MemberCashier :order-no="orderNo" />
  </MemberPanel>
</template>
