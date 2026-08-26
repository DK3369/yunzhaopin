<template>
  <div>
    <el-alert
      v-if="error"
      :title="title"
      type="error"
      show-icon
      :closable="false"
      style="margin-bottom: 12px"
    />
    <el-empty v-else-if="empty" :description="$t('ui.no_data')" />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{ error?: unknown; empty?: boolean }>()
const { t } = useI18n()
const title = computed(() => {
  const e = props.error as
    | { data?: { key?: string; msg?: string }; message?: string; statusCode?: number }
    | undefined
  if (!e) return ''
  if (e.data?.key === 'rate_limit' || e.statusCode === 429) return e.data?.msg || t('ui.rate_limit')
  return e.data?.msg || e.message || t('ui.load_failed')
})
</script>
