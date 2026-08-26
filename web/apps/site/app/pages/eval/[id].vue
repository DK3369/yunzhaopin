<script setup lang="ts">
const route = useRoute()
const id = computed(() => Number(route.params.id))
const api = useApi()
const { data, error } = await useAsyncData(
  () => `eval-detail-${id.value}`,
  () => api.post('/v1/wap/eval-papers/detail', { id: id.value }),
)
useSeoMeta({ title: data.value?.name ? String(data.value.name) : '测评详情' })
</script>

<template>
  <section>
    <h1>{{ data?.name || '测评详情' }}</h1>
    <p v-if="error" class="muted">无法加载测评。</p>
    <template v-else>
      <p class="muted">{{ data?.description }}</p>
      <p v-if="!(data?.questions || []).length" class="muted">暂无题目</p>
      <ol class="stack">
        <li v-for="q in data?.questions || []" :key="q.id">{{ q.content }}</li>
      </ol>
    </template>
  </section>
</template>
