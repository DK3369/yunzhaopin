<template>
  <li>
    <div class="com_release_name">
      <i v-if="required" class="ff0">*</i>
      {{ label }}
    </div>
    <div class="com_release_textnew" :class="{ 'com_release_textnew--area': area }">
      <slot />
    </div>
  </li>
</template>

<script setup lang="ts">
defineProps<{
  label: string
  required?: boolean
  area?: boolean
}>()
</script>
