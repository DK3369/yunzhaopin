<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const route = useRoute()
const id = computed(() => Number(route.params.id))
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData(
  () => `eval-log-${id.value}`,
  () =>
    api.post<{
      id: number
      paper_id?: number
      paper_name?: string
      score?: number
      comment?: string
      created_at_n?: string
      answers?: Record<string, string> | unknown[]
    }>('/v1/mcenter/eval-logs/detail', { id: id.value }),
)
const answers = computed(() => {
  const raw = data.value?.answers
  if (!raw || Array.isArray(raw)) return [] as Array<{ k: string; v: string }>
  return Object.entries(raw as Record<string, unknown>).map(([k, v]) => ({ k, v: String(v) }))
})
useSeoMeta({ title: t('wap_00194') })
</script>

<template>
  <section>
    <h1>{{ data?.paper_name || $t('wap_00194') }}</h1>
    <p>
      <NuxtLink to="/user/eval-logs">{{ $t('wap_00194') }}</NuxtLink>
    </p>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <template v-else-if="data">
      <p>{{ data.score }}</p>
      <p v-if="data.comment">{{ data.comment }}</p>
      <p class="muted">{{ data.created_at_n }}</p>
      <p v-if="data.paper_id">
        <NuxtLink :to="`/eval/${data.paper_id}`">{{ $t('wap_00194') }}</NuxtLink>
      </p>
      <ul v-if="answers.length" class="stack">
        <li v-for="row in answers" :key="row.k">{{ row.k }} · {{ row.v }}</li>
      </ul>
    </template>
  </section>
</template>
