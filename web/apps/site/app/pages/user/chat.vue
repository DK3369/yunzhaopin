<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({ title: t('wap_user_00363') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00363')">
    <MemberChat kind="user" />
  </MemberPanel>
</template>
