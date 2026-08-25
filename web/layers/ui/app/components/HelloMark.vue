<template>
  <p class="hello-mark">{{ text }}</p>
</template>

<script setup lang="ts">
defineProps<{
  text?: string
}>()
</script>

<style scoped>
.hello-mark {
  margin: 0;
  color: #2563eb;
  font-weight: 600;
}
</style>
