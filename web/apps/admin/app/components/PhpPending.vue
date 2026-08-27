<script setup lang="ts">
import { PHP_TEMPLATES } from '~/utils/phpScreens'

const route = useRoute()
const phpPath = computed(() => String(route.params.phpPath || route.path.replace(/^\//, '')))
const tpl = computed(() => PHP_TEMPLATES[phpPath.value] || '')
</script>

<template>
  <el-card>
    <h2>/{{ phpPath }}</h2>
    <p>
      该页尚未按 PHP 后台 1:1 迁完。对照模板：
      <code>uploads/app/template/admin/{{ tpl || '（router.js 无独立页）' }}</code>
    </p>
  </el-card>
</template>
