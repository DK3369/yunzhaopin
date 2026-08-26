<script setup lang="ts">
const route = useRoute()
const { t } = useI18n()
const id = computed(() => Number(route.params.id))
const api = useApi()
const { data, error } = await useAsyncData(
  () => `eval-detail-${id.value}`,
  () => api.post('/v1/wap/eval-papers/detail', { id: id.value }),
)
useSeoMeta({ title: data.value?.name ? String(data.value.name) : t('ui.eval_detail') })
</script>

<template>
  <section>
    <h1>{{ data?.name || $t('ui.eval_detail') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.eval_load_fail') }}</p>
    <template v-else>
      <p class="muted">{{ data?.description }}</p>
      <p v-if="!(data?.questions || []).length" class="muted">{{ $t('ui.no_eval_q') }}</p>
      <ol class="stack">
        <li v-for="q in data?.questions || []" :key="q.id">{{ q.content }}</li>
      </ol>
    </template>
  </section>
</template>
