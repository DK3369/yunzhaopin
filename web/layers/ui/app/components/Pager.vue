<template>
  <nav class="pager" v-if="totalPages > 1">
    <button :disabled="page <= 1" @click="$emit('update:page', page - 1)">上一页</button>
    <span>{{ page }} / {{ totalPages }}</span>
    <button :disabled="page >= totalPages" @click="$emit('update:page', page + 1)">下一页</button>
  </nav>
</template>

<script setup lang="ts">
const props = defineProps<{ page: number; pageSize: number; total: number }>()
defineEmits<{ 'update:page': [number] }>()
const totalPages = computed(() => Math.max(1, Math.ceil(props.total / props.pageSize)))
</script>

<style scoped>
.pager { display: flex; gap: 1rem; align-items: center; justify-content: center; margin: 1.5rem 0; }
button:disabled { opacity: 0.4; }
</style>
