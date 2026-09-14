<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({ title: t('wap_user_00196') })
const items = computed(() => [
  { to: '/user/parts', label: t('wap_user_00220') },
  { to: '/user/pay', label: t('common_01946') },
  { to: '/user/integral', label: t('wap_user_00008') },
  { to: '/user/eval-logs', label: t('wap_00194') },
  { to: '/user/looks', label: t('wap_user_00275') },
  { to: '/user/follows', label: t('wap_01142') },
  { to: '/user/recommend', label: t('wap_user_00211') },
  { to: '/user/searches', label: t('member_user_00108') },
  { to: '/user/outbox', label: t('member_user_00188') },
  { to: '/user/resume-tpls', label: t('member_user_00189') },
  { to: '/user/expects', label: t('wap_00460') },
  { to: '/user/consults', label: t('member_user_00115') },
  { to: '/user/messages', label: t('common.message') },
  { to: '/user/inbox', label: t('admin_user_00263') },
  { to: '/user/invite', label: t('wap_user_00253') },
  { to: '/user/reports', label: t('ui.my_reports') },
  { to: '/user/blacklist', label: t('member_user_00044') },
  { to: '/advice', label: t('wap_user_00203') },
])
</script>

<template>
  <MemberSetList :title="$t('wap_user_00196')" :items="items" />
</template>
