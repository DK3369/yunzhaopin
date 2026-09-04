<script setup lang="ts">
const route = useRoute()
const { t } = useI18n()
const { me } = useSiteChrome()
const id = computed(() => Number(route.params.id))
const api = useApi()
const { data, error } = await useAsyncData(
  () => `eval-detail-${id.value}`,
  () => api.post('/v1/wap/eval-papers/detail', { id: id.value }),
)
const answers = reactive<Record<string, string>>({})
const result = ref('')
async function submit() {
  result.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  try {
    const r = await api.post<{ score?: number }>('/v1/mcenter/eval-papers/submit', {
      id: id.value,
      answers: { ...answers },
    })
    result.value = String(r.score ?? t('common.confirm'))
  } catch (e: unknown) {
    result.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({ title: data.value?.name ? String(data.value.name) : t('ui.eval_detail') })
</script>

<template>
  <section>
    <h1>{{ data?.name || $t('ui.eval_detail') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.eval_load_fail') }}</p>
    <template v-else>
      <p class="muted">{{ data?.description }}</p>
      <p v-if="!(data?.questions || []).length" class="muted">{{ $t('ui.no_eval_q') }}</p>
      <form v-else class="form" @submit.prevent="submit">
        <ol class="stack">
          <li v-for="q in data?.questions || []" :key="q.id">
            <p>{{ q.content }}</p>
            <input v-model="answers[String(q.id)]" />
          </li>
        </ol>
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <p v-if="result">{{ result }}</p>
    </template>
  </section>
</template>
