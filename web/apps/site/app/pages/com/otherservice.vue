<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({ title: t('wap_user_00196') })
const items = computed(() => [
  { to: '/com/jobs', label: t('wap_com_00106') },
  { to: '/com/jobs/new', label: t('wap_00322') },
  { to: '/com/parts', label: t('wap_user_00271') },
  { to: '/com/pay', label: t('common_01946') },
  { to: '/com/cert', label: t('wap_com_00075') },
  { to: '/com/job-messages', label: t('member_user_00115') },
  { to: '/com/messages', label: t('common.message') },
  { to: '/com/news', label: t('ui.com_news') },
  { to: '/com/products', label: t('ui.com_products') },
  { to: '/com/banners', label: t('ui.com_banner') },
  { to: '/com/tpls', label: t('ui.com_tpl') },
  { to: '/com/finder', label: t('member_com_00086') },
  { to: '/com/specials', label: t('wap_com_00310') },
  { to: '/com/stats', label: t('admin_tool_00224') },
  { to: '/com/report', label: t('member_com_00148') },
])
</script>

<template>
  <MemberSetList kind="com" :title="$t('wap_user_00196')" :items="items" />
</template>
