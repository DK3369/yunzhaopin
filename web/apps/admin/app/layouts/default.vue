<script setup lang="ts">
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' }).catch(() => undefined)
  await navigateTo('/login')
}
</script>

<template>
  <el-container style="min-height: 100vh">
    <el-aside width="220px" style="border-right: 1px solid #eee">
      <h2 style="padding: 1rem 1rem 0; font-size: 16px">{{ $t('admin_index_00072') }}</h2>
      <div style="padding: 0 1rem 0.5rem">
        <LangSwitch cookie-key="admin_lang" />
      </div>
      <el-menu router :default-active="$route.path" :default-openeds="['review', 'content', 'ops', 'mall', 'sys']">
        <el-menu-item index="/">{{ $t('ui.dashboard') }}</el-menu-item>
        <el-sub-menu index="review">
          <template #title>{{ $t('ui.review') }}</template>
          <el-menu-item index="/users">{{ $t('common.user_center') }}</el-menu-item>
          <el-menu-item index="/companies">{{ $t('ui.companies') }}</el-menu-item>
          <el-menu-item index="/resumes">{{ $t('ui.resume_audit') }}</el-menu-item>
          <el-menu-item index="/resumes/tree">{{ $t('ui.resume_tree') }}</el-menu-item>
          <el-menu-item index="/jobs">{{ $t('ui.jobs_audit') }}</el-menu-item>
          <el-menu-item index="/jobs/parts">{{ $t('ui.parts_audit') }}</el-menu-item>
          <el-menu-item index="/jobs/once">{{ $t('ui.once_audit') }}</el-menu-item>
          <el-menu-item index="/jobs/tiny">{{ $t('ui.tiny_audit') }}</el-menu-item>
          <el-menu-item index="/certs">{{ $t('ui.certs') }}</el-menu-item>
          <el-menu-item index="/reports">{{ $t('ui.reports') }}</el-menu-item>
          <el-menu-item index="/feedback">{{ $t('ui.feedback') }}</el-menu-item>
          <el-menu-item index="/orders">{{ $t('ui.orders') }}</el-menu-item>
          <el-menu-item index="/finance/recharge">{{ $t('ui.recharge') }}</el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="content">
          <template #title>{{ $t('ui.content') }}</template>
          <el-menu-item index="/content/articles">{{ $t('ui.articles') }}</el-menu-item>
          <el-menu-item index="/content/announcements">{{ $t('ui.announcements') }}</el-menu-item>
          <el-menu-item index="/content/questions">{{ $t('ui.questions') }}</el-menu-item>
          <el-menu-item index="/content/pages">{{ $t('ui.pages') }}</el-menu-item>
          <el-menu-item index="/content/fairs">{{ $t('ui.fairs') }}</el-menu-item>
          <el-menu-item index="/content/gongzhao">{{ $t('ui.gongzhao') }}</el-menu-item>
          <el-menu-item index="/content/specials">{{ $t('ui.specials') }}</el-menu-item>
          <el-menu-item index="/content/links">{{ $t('ui.links') }}</el-menu-item>
          <el-menu-item index="/ads">{{ $t('ui.ads') }}</el-menu-item>
          <el-menu-item index="/nav">{{ $t('ui.nav') }}</el-menu-item>
          <el-menu-item index="/categories">{{ $t('ui.categories') }}</el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="mall">
          <template #title>{{ $t('ui.mall') }}</template>
          <el-menu-item index="/redeem/classes">{{ $t('ui.redeem_classes') }}</el-menu-item>
          <el-menu-item index="/redeem/rewards">{{ $t('ui.redeem_rewards') }}</el-menu-item>
          <el-menu-item index="/redeem/orders">{{ $t('ui.redeem_orders') }}</el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="ops">
          <template #title>{{ $t('ui.ops') }}</template>
          <el-menu-item index="/ops/broadcasts">{{ $t('ui.broadcasts') }}</el-menu-item>
          <el-menu-item index="/ops/warnings">{{ $t('ui.warnings') }}</el-menu-item>
          <el-menu-item index="/ops/audit">{{ $t('ui.audit') }}</el-menu-item>
          <el-menu-item index="/ops/recycle">{{ $t('ui.recycle') }}</el-menu-item>
          <el-menu-item index="/ops/app-versions">{{ $t('ui.app_versions') }}</el-menu-item>
          <el-menu-item index="/ops/accounts">{{ $t('ui.accounts') }}</el-menu-item>
          <el-menu-item index="/ops/hotjobs">{{ $t('ui.hotjobs') }}</el-menu-item>
          <el-menu-item index="/ops/expire">{{ $t('ui.expire') }}</el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="sys">
          <template #title>{{ $t('ui.system') }}</template>
          <el-menu-item index="/system/settings">{{ $t('ui.settings') }}</el-menu-item>
          <el-menu-item index="/system/seo">{{ $t('ui.seo') }}</el-menu-item>
          <el-menu-item index="/system/email">{{ $t('ui.email') }}</el-menu-item>
          <el-menu-item index="/system/sms">{{ $t('ui.sms') }}</el-menu-item>
          <el-menu-item index="/system/weixin">{{ $t('ui.weixin') }}</el-menu-item>
          <el-menu-item index="/system/cron">{{ $t('ui.cron') }}</el-menu-item>
          <el-menu-item index="/system/geo">{{ $t('ui.geo') }}</el-menu-item>
          <el-menu-item index="/system/admins">{{ $t('ui.admins') }}</el-menu-item>
          <el-menu-item index="/system/rbac">{{ $t('ui.rbac') }}</el-menu-item>
        </el-sub-menu>
      </el-menu>
      <div style="padding: 1rem">
        <el-button size="small" @click="logout">{{ $t('common.logout') }}</el-button>
      </div>
    </el-aside>
    <el-main>
      <slot />
    </el-main>
  </el-container>
</template>
