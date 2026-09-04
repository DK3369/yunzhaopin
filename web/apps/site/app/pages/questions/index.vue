<script setup lang="ts">
import { listFailMsg, errKey } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const categoryId = computed(() => Number(route.query.cid || route.query.category_id || 0) || undefined)
const order = computed(() => String(route.query.order || 'latest'))
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `questions-${page.value}-${keyword.value}-${categoryId.value || 0}-${order.value}`,
  () =>
    api.get<{ list: Array<{ id: number; title: string; catname?: string; answer_count?: number }>; total: number }>(
      '/v1/wap/questions',
      { page: page.value, page_size: 20, keyword: keyword.value || undefined, category_id: categoryId.value, order: order.value },
    ),
)
useSeoMeta({ title: t('wap_00160') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
const { me } = useSiteChrome()
const mineTab = computed(() => String(route.query.mine || ''))
const { data: myQuestions } = await useAsyncData(
  () => `my-questions-${me.value?.uid || 0}`,
  () =>
    me.value
      ? api.post<{ list: Array<{ id: number; title: string }> }>('/v1/mcenter/my/questions', { page: 1, page_size: 20 }).catch(() => ({ list: [] }))
      : Promise.resolve({ list: [] }),
)
const { data: myAnswers } = await useAsyncData(
  () => `my-answers-${me.value?.uid || 0}`,
  () =>
    me.value
      ? api.post<{ list: Array<{ id: number; question_id: number; content: string }> }>('/v1/mcenter/my/answers', { page: 1, page_size: 20 }).catch(() => ({ list: [] }))
      : Promise.resolve({ list: [] }),
)
const { data: myAttended } = await useAsyncData(
  () => `my-attended-${me.value?.uid || 0}`,
  () =>
    me.value
      ? api.post<{ list: Array<{ id: number; title: string }> }>('/v1/mcenter/my/attended-questions', { page: 1, page_size: 20 }).catch(() => ({ list: [] }))
      : Promise.resolve({ list: [] }),
)
const { data: hotweek } = await useAsyncData('questions-hotweek', () =>
  api.get<Array<{ id: number; title: string }>>('/v1/wap/qna/hotweek', { limit: 10 }).catch(() => []),
)
const askTitle = ref('')
const askContent = ref('')
const askMsg = ref('')
async function ask() {
  askMsg.value = ''
  if (!me.value) {
    navigateTo({ path: '/login', query: { next: '/questions' } })
    return
  }
  try {
    await api.post('/v1/mcenter/questions', { title: askTitle.value, content: askContent.value, category_id: categoryId.value || 0 })
    askTitle.value = ''
    askContent.value = ''
    askMsg.value = t('common.success')
    await refreshNuxtData()
  } catch (e: unknown) {
    askMsg.value = errKey(e) === 'unauth' ? t('wap_00032') : t(errKey(e) || 'common_00888')
  }
}
</script>

<template>
  <section>
    <p>
      <NuxtLink to="/questions/topics">{{ $t('wap_user_00223') }}</NuxtLink>
      <template v-if="me">
        · <NuxtLink :to="{ query: { mine: 'q' } }">{{ $t('wap_00160') }}</NuxtLink>
        · <NuxtLink :to="{ query: { mine: 'a' } }">{{ $t('ui.qa') }}</NuxtLink>
        · <NuxtLink :to="{ query: { mine: 'f' } }">{{ $t('wap_00164') }}</NuxtLink>
      </template>
    </p>
    <template v-if="mineTab === 'q'">
      <NewsListShell :title="$t('wap_00160')" :count="(myQuestions?.list || []).length">
        <SimpleCard v-for="row in myQuestions?.list || []" :key="row.id" :to="`/questions/${row.id}`" :title="row.title" />
      </NewsListShell>
    </template>
    <template v-else-if="mineTab === 'a'">
      <NewsListShell :title="$t('ui.qa')" :count="(myAnswers?.list || []).length">
        <SimpleCard
          v-for="row in myAnswers?.list || []"
          :key="row.id"
          :to="`/questions/${row.question_id}`"
          :title="row.content"
        />
      </NewsListShell>
    </template>
    <template v-else-if="mineTab === 'f'">
      <NewsListShell :title="$t('wap_00164')" :count="(myAttended?.list || []).length">
        <SimpleCard v-for="row in myAttended?.list || []" :key="row.id" :to="`/questions/${row.id}`" :title="row.title" />
      </NewsListShell>
    </template>
    <template v-else>
    <p v-if="(hotweek || []).length">
      {{ $t('common.hot') }}
      <NuxtLink v-for="h in hotweek || []" :key="h.id" :to="`/questions/${h.id}`">{{ h.title }}</NuxtLink>
    </p>
    <form class="yun_bth_box" @submit.prevent="ask">
      <input v-model="askTitle" :placeholder="$t('wap_00160')" />
      <textarea v-model="askContent" rows="3" />
      <button type="submit">{{ $t('common.submit') }}</button>
      <p v-if="askMsg">{{ askMsg }}</p>
    </form>
    <NewsListShell :title="$t('wap_00160')" :error="error" :error-text="failMsg" :count="list.length">
      <SimpleCard
        v-for="row in list"
        :key="row.id"
        :to="`/questions/${row.id}`"
        :title="row.title"
        :meta="`${row.catname || ''} · ${row.answer_count || 0}`"
      />
      <template #pager>
        <Pager
          :page="page"
          :page-size="20"
          :total="data?.total || 0"
          @update:page="(p) => navigateTo({ query: { page: p } })"
        />
      </template>
    </NewsListShell>
    </template>
  </section>
</template>
