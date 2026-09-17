<script setup lang="ts">
import { isMemberModuleOn } from '~/utils/site'

const { t } = useI18n()
const { settings } = useSiteChrome()
useSeoMeta({ title: t('wap_user_00214') })
const items = computed(() =>
  [
    { to: '/user/resume', label: t('wap_user_00341'), hint: t('wap_00819') },
    { to: '/user/ident', label: t('wap_user_00340') },
    { to: '/user/password', label: t('wap_00817') },
    { to: '/user/privacy', label: t('wap_user_00215'), hint: t('wap_00820') },
    { to: '/advice', label: t('wap_user_00203') },
    { to: '/user/binding', label: t('wap_00389') },
    { to: '/user/account', label: t('wap_user_00338') },
    { to: '/user/blacklist', label: t('member_user_00044') },
    { to: '/user/inbox', label: t('admin_user_00263') },
    { to: '/user/reports', label: t('ui.my_reports') },
    { to: '/user/messages', label: t('common.message') },
    { to: '/user/chat', label: t('wap_user_00363') },
  ].filter((item) => isMemberModuleOn(settings.value, item.to)),
)
</script>

<template>
  <MemberSetList :title="$t('wap_user_00214')" :items="items" logoutable />
</template>
