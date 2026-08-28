<template>
  <section>
    <h1>{{ title }}</h1>
    <p v-if="error" class="muted">{{ errorText }}</p>
    <EmptyState v-else-if="!count" :title="emptyText || $t('ui.no_data')" />
    <div v-else class="yun_content news_list_box">
      <div class="index_news_box">
        <ul class="index_news_list_list">
          <slot />
        </ul>
      </div>
    </div>
    <slot name="pager" />
  </section>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  error?: unknown
  errorText?: string
  count: number
  emptyText?: string
}>()
</script>
