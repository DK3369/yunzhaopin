<template>
  <li>
    <NuxtLink :to="to">
      <i class="index_news_list_icon" />{{ title }}
      <em v-if="meta">{{ meta }}</em>
    </NuxtLink>
  </li>
</template>

<script setup lang="ts">
defineProps<{
  to: string
  title: string
  meta?: string
}>()
</script>
