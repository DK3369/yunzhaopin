<template>
  <el-container style="min-height: 100vh">
    <el-aside width="220px">
      <h2 style="padding: 1rem">管理后台</h2>
      <el-menu router :default-active="$route.path">
        <el-menu-item index="/">仪表盘</el-menu-item>
        <el-menu-item index="/users">用户</el-menu-item>
        <el-menu-item index="/jobs">职位审核</el-menu-item>
        <el-menu-item index="/reports">举报</el-menu-item>
        <el-menu-item index="/content/articles">资讯</el-menu-item>
      </el-menu>
    </el-aside>
    <el-main>
      <slot />
    </el-main>
  </el-container>
</template>
