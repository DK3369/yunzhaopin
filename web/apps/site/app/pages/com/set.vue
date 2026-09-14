<script setup lang="ts">
import { isMemberModuleOn } from '~/utils/site'

const { t } = useI18n()
const { settings } = useSiteChrome()
useSeoMeta({ title: t('wap_user_00214') })
const items = computed(() =>
  [
    { to: '/com/profile', label: t('wap_user_00341'), hint: t('wap_00803') },
    { to: '/com/addresses', label: t('wap_com_00402'), hint: t('wap_com_00331') },
    { to: '/com/cert', label: t('wap_user_00340') },
    { to: '/com/interview-tpls', label: t('wap_com_00404'), hint: t('wap_com_00403') },
    { to: '/com/gallery', label: t('wap_com_00401') },
    { to: '/com/binding', label: t('member_user_00059') },
    { to: '/com/password', label: t('wap_00817') },
    { to: '/com/otherservice', label: t('wap_user_00196') },
    { to: '/advice', label: t('wap_user_00203') },
  ].filter((item) => isMemberModuleOn(settings.value, item.to)),
)
</script>

<template>
  <MemberSetList kind="com" :title="$t('wap_user_00214')" :items="items" logoutable />
</template>
