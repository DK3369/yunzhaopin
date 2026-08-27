<script setup lang="ts">
import { resolvePhpScreen } from '~/utils/phpScreens'

const route = useRoute()
const Screen = computed(() => resolvePhpScreen(String(route.params.phpPath || '')))
</script>

<template>
  <component :is="Screen" />
</template>
