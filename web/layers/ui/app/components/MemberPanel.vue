<template>
  <section class="yun_m_rightbox">
    <h1>{{ title }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <p v-else-if="empty" class="muted">{{ $t('ui.no_data') }}</p>
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
