<template>
  <section class="yun_m_rightbox">
    <h1>{{ title }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
    <EmptyState v-else-if="empty" />
    <slot />
  </section>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  error?: unknown
  empty?: boolean
}>()
</script>
