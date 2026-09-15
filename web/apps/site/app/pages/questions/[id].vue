<script setup lang="ts">
import { seoJoin } from '~/utils/seo'
import { ensureLogin } from '~/utils/site'

const route = useRoute()
const id = Number(route.params.id)
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
  if (!(await ensureLogin(me.value, route.fullPath))) return
  try {
    const r = await api.post<{ on?: boolean }>('/v1/mcenter/questions/attention', { id })
    following.value = Boolean(r.on)
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function postComment(aid: number) {
  askMsg.value = ''
  if (!(await ensureLogin(me.value, route.fullPath))) return
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
  if (!(await ensureLogin(me.value, route.fullPath))) return
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
  if (!(await ensureLogin(me.value, route.fullPath))) return
  try {
    await api.post('/v1/mcenter/answers/support', { id: aid })
    await refreshAnswers()
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function deleteQuestion() {
  askMsg.value = ''
  if (!(await ensureLogin(me.value, route.fullPath))) return
  try {
    await api.post('/v1/mcenter/questions/delete', { id })
    await navigateTo('/questions')
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
const reportReason = ref('spam')
const reportDetail = ref('')
const captcha = ref<{ cid: string; image: string } | null>(null)
const captchaInput = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  captchaInput.value = ''
}
async function reportQuestion() {
  askMsg.value = ''
  if (!(await ensureLogin(me.value, route.fullPath))) return
  if (!captcha.value) await loadCaptcha()
  try {
    await api.post('/v1/mcenter/reports', {
      target_kind: 6,
      target_id: id,
      reason_code: reportReason.value,
      detail: reportDetail.value,
      captcha_cid: captcha.value?.cid || '',
      captcha_input: captchaInput.value,
    })
    askMsg.value = t('common.success')
    reportDetail.value = ''
    await loadCaptcha()
  } catch (e: unknown) {
    askMsg.value = e instanceof Error ? e.message : t('common_00888')
    await loadCaptcha()
  }
}
const isOwner = computed(() => Number(me.value?.uid || 0) === Number(row.value.uid || 0))
useSeoMeta({
  title: () => String(row.value.title || t('ui.qa')),
  description: () => seoJoin([row.value.content, row.value.title]),
})
useHead({
  link: [
    { rel: 'canonical', href: `/questions/${id}` },
    { rel: 'stylesheet', href: '/legacy/h5/css/ask/ask.css', media: 'screen and (max-width: 1199px)' },
  ],
})
</script>

<template>
  <article class="site-pc">
    <h1>{{ row.title || $t('ui.no_questions') }}</h1>
    <p v-if="row.catname" class="muted">{{ row.catname }} · {{ row.nickname }}</p>
    <div v-if="row.content" v-html="String(row.content)" />
    <p v-else-if="!row.title" class="muted">{{ $t('wap_00630') }}</p>
    <p>
      <a href="javascript:;" @click.prevent="toggleFollow">{{ following ? $t('wap_js_00140') : $t('wap_00164') }}</a>
      <a v-if="isOwner" href="javascript:;" @click.prevent="deleteQuestion"> {{ $t('common.delete') }}</a>
    </p>
    <form class="form" @submit.prevent="reportQuestion">
      <input v-model="reportReason" :placeholder="$t('ui.detail')" />
      <textarea v-model="reportDetail" rows="2" />
      <img v-if="captcha?.image" :src="captcha.image" alt="" @click="loadCaptcha" />
      <input v-model="captchaInput" :placeholder="$t('wap_00110')" @focus="!captcha && loadCaptcha()" />
      <button type="submit">{{ $t('wap_com_00350') }}</button>
    </form>
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
  <div class="site-h5">
    <div class="askct_iss">
      <div class="askct_iss_ct">
        <div class="askct_iss_p">
          <div class="askct_iss_p_ftp">{{ row.title || $t('ui.no_questions') }}</div>
          <div v-if="row.content" class="askct_iss_dt" v-html="String(row.content)" />
          <p v-else-if="!row.title" class="muted">{{ $t('wap_00630') }}</p>
        </div>
        <div class="askct_iss_tm">
          <div class="askct_iss_tm_ft">
            <span class="askct_iss_tm_ft_time">{{ row.created_at_n }}</span>
            <span class="ask_jx_data ask_jx_data_hd">{{ row.answer_count || 0 }} {{ $t('wap_00332') }}</span>
            <span class="ask_jx_data ask_jx_data_yl">{{ row.hits || 0 }} {{ $t('wap_01578') }}</span>
            <span class="ask_jx_data ask_jx_data_gz">{{ row.support_count || 0 }} {{ $t('common_01949') }}</span>
          </div>
          <div class="ask_user_tw">
            {{ $t('wap_01582') }}<span class="ask_jx_username">{{ row.nickname }}</span> {{ $t('wap_01583') }}
            <div class="askct_iss_tm_rt">
              <a class="askct_iss_tm_rt_b" href="javascript:;" @click.prevent="reportQuestion">{{ $t('wap_com_00350') }}</a>
            </div>
          </div>
          <div class="ask_content_cz">
            <a href="javascript:;" class="ask_content_cz_a" @click.prevent="toggleFollow">{{ following ? $t('wap_js_00140') : $t('wap_00164') }}</a>
            <a v-if="isOwner" href="javascript:;" class="ask_content_cz_a" @click.prevent="deleteQuestion">{{ $t('common.delete') }}</a>
            <a href="#ask-h5-answer" class="ask_content_cz_a ask_content_cz_a_cur">{{ $t('wap_01302') }}</a>
          </div>
        </div>
        <form class="form" style="padding: 0.32rem" @submit.prevent="reportQuestion">
          <input v-model="reportReason" :placeholder="$t('ui.detail')" />
          <textarea v-model="reportDetail" rows="2" />
          <img v-if="captcha?.image" :src="captcha.image" alt="" @click="loadCaptcha" />
          <input v-model="captchaInput" :placeholder="$t('wap_00110')" @focus="!captcha && loadCaptcha()" />
          <button type="submit">{{ $t('wap_com_00350') }}</button>
        </form>
      </div>
    </div>
    <div class="ask_c_pl mt10">
      <div class="ask_c_pl_titbox">
        <span class="ask_c_pl_titbox_all">{{ $t('wap_00163') }}</span>
      </div>
    </div>
    <div class="askct_com">
      <div v-if="!allAnswers.length" class="wap_member_no">{{ $t('wap_01309') }}</div>
      <div v-for="a in allAnswers" :key="'h5a-' + Number(a.id)" class="askct_com_rs">
        <div class="askct_com_rs_img">
          <img v-if="a.pic_n || a.pic" :src="String(a.pic_n || a.pic)" width="40" height="40" alt="" />
        </div>
        <div class="askct_com_rs_y">
          <div class="askct_com_rs_y_x">
            <div class="askct_com_rs_tit_ft">
              <span>{{ a.nickname }}</span>
              <span class="askct_com_rs_tit_ft_time">{{ a.created_at_n }}</span>
            </div>
            <div class="askct_com_rs_y_p" v-html="String(a.content || '')" />
          </div>
          <div class="ask_c_pl_bot">
            <a href="javascript:;" class="ask_c_pl_cz ask_c_pl_cz_z" @click.prevent="supportAnswer(Number(a.id))">
              {{ a.support_count || 0 }} {{ $t('wap_01307') }}
            </a>
            <a href="javascript:;" class="ask_c_pl_cz">{{ (comments[Number(a.id)] || []).length }} {{ $t('wap_00167') }}</a>
          </div>
          <div v-for="c in comments[Number(a.id)] || []" :key="Number(c.id)" class="muted">
            {{ c.nickname }}：{{ c.content }}
          </div>
          <form class="form" @submit.prevent="postComment(Number(a.id))">
            <input v-model="commentDraft[Number(a.id)]" class="menu_p1_nr_t_bor_tetx" :placeholder="$t('wap_01308')" />
            <button type="submit">{{ $t('common.submit') }}</button>
          </form>
        </div>
      </div>
    </div>
    <form id="ask-h5-answer" class="com_pop_up_area" style="padding: 0.32rem" @submit.prevent="postAnswer">
      <textarea v-model="answerText" class="com_area" rows="4" required :placeholder="$t('wap_01303')" />
      <div class="ask_yz_box_bth">
        <input type="submit" :value="$t('common.submit')" />
      </div>
    </form>
    <p v-if="askMsg">{{ askMsg }}</p>
  </div>
</template>
