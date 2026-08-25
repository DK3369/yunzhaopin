<script setup lang="ts">
async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' }).catch(() => undefined)
  await navigateTo('/login')
}
</script>

<template>
  <el-container style="min-height: 100vh">
    <el-aside width="220px" style="border-right: 1px solid #eee">
      <h2 style="padding: 1rem 1rem 0; font-size: 16px">管理后台</h2>
      <el-menu router :default-active="$route.path" :default-openeds="['review', 'content', 'ops', 'mall', 'sys']">
        <el-menu-item index="/">仪表盘</el-menu-item>
        <el-sub-menu index="review">
          <template #title>审核运营</template>
          <el-menu-item index="/users">用户</el-menu-item>
          <el-menu-item index="/jobs">职位审核</el-menu-item>
          <el-menu-item index="/certs">企业认证</el-menu-item>
          <el-menu-item index="/reports">举报</el-menu-item>
          <el-menu-item index="/feedback">反馈</el-menu-item>
          <el-menu-item index="/orders">VIP 订单</el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="content">
          <template #title>内容</template>
          <el-menu-item index="/content/articles">资讯（只读）</el-menu-item>
          <el-menu-item index="/content/pages">单页 CMS</el-menu-item>
          <el-menu-item index="/ads">广告</el-menu-item>
          <el-menu-item index="/nav">导航</el-menu-item>
          <el-menu-item index="/categories">分类</el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="mall">
          <template #title>积分商城</template>
          <el-menu-item index="/redeem/classes">分类</el-menu-item>
          <el-menu-item index="/redeem/rewards">商品</el-menu-item>
          <el-menu-item index="/redeem/orders">兑换单</el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="ops">
          <template #title>运营工具</template>
          <el-menu-item index="/ops/broadcasts">广播</el-menu-item>
          <el-menu-item index="/ops/warnings">警告</el-menu-item>
          <el-menu-item index="/ops/audit">审计日志</el-menu-item>
          <el-menu-item index="/ops/recycle">回收站</el-menu-item>
          <el-menu-item index="/ops/app-versions">App 版本</el-menu-item>
          <el-menu-item index="/ops/accounts">账号工具</el-menu-item>
        </el-sub-menu>
        <el-sub-menu index="sys">
          <template #title>系统</template>
          <el-menu-item index="/system/settings">站点配置</el-menu-item>
          <el-menu-item index="/system/geo">地区/国家</el-menu-item>
        </el-sub-menu>
      </el-menu>
      <div style="padding: 1rem">
        <el-button size="small" @click="logout">退出</el-button>
      </div>
    </el-aside>
    <el-main>
      <slot />
    </el-main>
  </el-container>
</template>
