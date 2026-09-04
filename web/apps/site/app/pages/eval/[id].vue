<script setup lang="ts">
const route = useRoute()
const { t } = useI18n()
const id = computed(() => Number(route.params.id))
const api = useApi()
const { me } = useSiteChrome()
const { data, error } = await useAsyncData(
  () => `eval-detail-${id.value}`,
  () => api.post('/v1/wap/eval-papers/detail', { id: id.value }),
)
const { data: messages, refresh: refreshMsg } = await useAsyncData(
  () => `eval-msg-${id.value}`,
  () =>
    api
      .post<{ list?: Array<{ id: number; message?: string; uid?: string; ctime_n?: string }> }>(
        '/v1/wap/eval-papers/messages/list',
        { id: id.value, page: 1, page_size: 20 },
      )
      .catch(() => ({ list: [] })),
)
const { data: examinees } = await useAsyncData(
  () => `eval-exam-${id.value}`,
  () =>
    api
      .post<Array<{ uid: number; last_taken_at_n?: string; papers_taken?: number }>>(
        '/v1/wap/eval-papers/recent-examinees',
        { id: id.value },
      )
      .catch(() => []),
)
const answers = reactive<Record<string, string>>({})
const result = ref('')
const comment = ref('')
const leave = ref('')
const leaveMsg = ref('')
function nuidCookie(): string {
  if (!import.meta.client) return ''
  const m = document.cookie.match(/(?:^|; )eval_nuid=([^;]*)/)
  return m ? decodeURIComponent(m[1]) : ''
}
function setNuid(v: string) {
  if (!import.meta.client || !v) return
  document.cookie = `eval_nuid=${encodeURIComponent(v)};path=/;max-age=3600`
}
async function submit() {
  result.value = ''
  comment.value = ''
  try {
    const r = await api.post<{ score?: number; comment?: string; nuid?: string }>('/v1/wap/eval-papers/submit', {
      id: id.value,
      answers: { ...answers },
      nuid: nuidCookie() || undefined,
    })
    if (r.nuid) setNuid(r.nuid)
    result.value = String(r.score ?? '')
    comment.value = r.comment || ''
  } catch (e: unknown) {
    result.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function postMessage() {
  leaveMsg.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  try {
    await api.post('/v1/mcenter/eval-papers/messages/post', { id: id.value, message: leave.value })
    leave.value = ''
    leaveMsg.value = t('common.success')
    await refreshMsg()
  } catch (e: unknown) {
    leaveMsg.value = e instanceof Error ? e.message : t('common_00888')
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
            <label v-for="(opt, idx) in (q.options || [])" :key="String(opt.label ?? idx)" class="muted">
              <input
                v-model="answers[String(q.id)]"
                type="radio"
                :name="`q-${q.id}`"
                :value="String(opt.label ?? idx)"
              />
              {{ opt.text || opt.label }}
            </label>
          </li>
        </ol>
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <p v-if="result">{{ result }}</p>
      <p v-if="comment" class="muted">{{ comment }}</p>
      <h2>{{ $t('common.message') }}</h2>
      <p v-if="!(messages?.list || []).length" class="muted">{{ $t('common_02409') }}</p>
      <p v-for="m in messages?.list || []" :key="m.id" class="muted">{{ m.uid }} · {{ m.ctime_n }} · {{ m.message }}</p>
      <form class="form" @submit.prevent="postMessage">
        <textarea v-model="leave" rows="3" required />
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <p v-if="leaveMsg">{{ leaveMsg }}</p>
      <h2>{{ $t('common.hot') }}</h2>
      <p v-if="!(examinees || []).length" class="muted">{{ $t('common_02409') }}</p>
      <p v-for="e in examinees || []" :key="e.uid" class="muted">{{ e.uid }} · {{ e.last_taken_at_n }} · {{ e.papers_taken }}</p>
    </template>
  </section>
</template>
