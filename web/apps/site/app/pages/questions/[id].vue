<script setup lang="ts">
import { seoJoin } from '~/utils/seo'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { me } = useSiteChrome()
const { data, refresh } = await useAsyncData(`question-${id}`, () =>
  api.get('/v1/wap/questions/detail', { id }),
)
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const answers = computed(
  () =>
    (Array.isArray(row.value.top_answers) ? row.value.top_answers : []) as Array<
      Record<string, unknown>
    >,
)
const { data: moreAnswers, refresh: refreshAnswers } = await useAsyncData(`question-answers-${id}`, () =>
  api.get<{ list?: Array<Record<string, unknown>> }>('/v1/wap/questions/answers', { id, page: 1, page_size: 20 }).catch(() => ({ list: [] })),
)
const allAnswers = computed(() => {
  const extra = moreAnswers.value?.list || []
  if (extra.length) return extra
  return answers.value
})
const answerText = ref('')
const askMsg = ref('')
const commentDraft = reactive<Record<number, string>>({})
const comments = ref<Record<number, Array<Record<string, unknown>>>>({})
const following = ref(false)
watch(
  () => Number(row.value.is_attention || row.value.qatn || 0),
  (v) => {
    following.value = v === 1
  },
  { immediate: true },
)
async function loadComments(aid: number) {
  try {
    const r = await api.get<{ list?: Array<Record<string, unknown>> }>('/v1/wap/answers/comments/list', {
      aid,
      page: 1,
      page_size: 20,
    })
    comments.value = { ...comments.value, [aid]: r.list || [] }
  } catch {
    comments.value = { ...comments.value, [aid]: [] }
  }
}
watch(
  allAnswers,
  (list) => {
    for (const a of list) {
      const aid = Number(a.id || 0)
      if (aid && !(aid in comments.value)) loadComments(aid)
    }
  },
  { immediate: true },
)
async function toggleFollow() {
  askMsg.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  try {
    const r = await api.post<{ on?: boolean }>('/v1/mcenter/questions/attention', { id })
    following.value = Boolean(r.on)
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function postComment(aid: number) {
  askMsg.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  try {
    await api.post('/v1/mcenter/answers/comments', { aid, content: commentDraft[aid] || '' })
    commentDraft[aid] = ''
    askMsg.value = t('common.success')
    await loadComments(aid)
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function postAnswer() {
  askMsg.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  try {
    await api.post('/v1/mcenter/questions/answers', { id, content: answerText.value })
    answerText.value = ''
    askMsg.value = t('common.success')
    await refresh()
    await refreshAnswers()
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function supportAnswer(aid: number) {
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  try {
    await api.post('/v1/mcenter/answers/support', { id: aid })
    await refreshAnswers()
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({
  title: () => String(row.value.title || t('ui.qa')),
  description: () => seoJoin([row.value.content, row.value.title]),
})
useHead({ link: [{ rel: 'canonical', href: `/questions/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ row.title || $t('ui.no_questions') }}</h1>
    <p v-if="row.catname" class="muted">{{ row.catname }} · {{ row.nickname }}</p>
    <div v-if="row.content" v-html="String(row.content)" />
    <p v-else-if="!row.title" class="muted">{{ $t('wap_00630') }}</p>
    <p>
      <a href="javascript:;" @click.prevent="toggleFollow">{{ following ? $t('wap_js_00140') : $t('wap_00164') }}</a>
    </p>
    <h2>{{ $t('ui.qa') }}</h2>
    <p v-if="!allAnswers.length" class="muted">{{ $t('common_02409') }}</p>
    <div v-for="a in allAnswers" :key="Number(a.id)" class="stack">
      <div v-html="String(a.content || '')" />
      <p class="muted">
        {{ a.nickname }} · {{ a.support_count || 0 }}
        <a href="javascript:;" @click.prevent="supportAnswer(Number(a.id))">{{ $t('common.like') }}</a>
      </p>
      <div v-for="c in comments[Number(a.id)] || []" :key="Number(c.id)" class="muted">
        {{ c.nickname }}：{{ c.content }}
      </div>
      <form class="form" @submit.prevent="postComment(Number(a.id))">
        <input v-model="commentDraft[Number(a.id)]" :placeholder="$t('common.message')" />
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
    </div>
    <form class="form" @submit.prevent="postAnswer">
      <textarea v-model="answerText" rows="4" required />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="askMsg">{{ askMsg }}</p>
  </article>
</template>
