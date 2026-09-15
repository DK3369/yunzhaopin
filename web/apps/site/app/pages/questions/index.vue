<script setup lang="ts">
import { listFailMsg, errKey, ensureLogin } from '~/utils/site'

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
    api.get<{
      list: Array<{
        id: number
        title: string
        catname?: string
        answer_count?: number
        hits?: number
        support_count?: number
        content_excerpt?: string
        nickname?: string
        pic?: string
        pic_n?: string
        created_at_n?: string
      }>
      total: number
    }>(
      '/v1/wap/questions',
      { page: page.value, page_size: 20, keyword: keyword.value || undefined, category_id: categoryId.value, order: order.value },
    ),
)
useSeoMeta({ title: t('wap_00160') })
useHead({
  link: [{ rel: 'stylesheet', href: '/legacy/h5/css/ask/ask.css', media: 'screen and (max-width: 1199px)' }],
})
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
  if (!(await ensureLogin(me.value, route.fullPath))) return
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
  <div class="site-pc">
    <section>
      <p>
        <NuxtLink to="/questions/topics">{{ $t('wap_user_00223') }}</NuxtLink>
        <template v-if="me">
          · <NuxtLink :to="{ query: { mine: 'q' } }">{{ $t('wap_00331') }}</NuxtLink>
          · <NuxtLink :to="{ query: { mine: 'a' } }">{{ $t('wap_01443') }}</NuxtLink>
          · <NuxtLink :to="{ query: { mine: 'f' } }">{{ $t('wap_00164') }}</NuxtLink>
        </template>
      </p>
      <template v-if="mineTab === 'q'">
        <NewsListShell :title="$t('wap_00331')" :count="(myQuestions?.list || []).length">
          <SimpleCard v-for="row in myQuestions?.list || []" :key="row.id" :to="`/questions/${row.id}`" :title="row.title" />
        </NewsListShell>
      </template>
      <template v-else-if="mineTab === 'a'">
        <NewsListShell :title="$t('wap_01443')" :count="(myAnswers?.list || []).length">
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
  </div>
  <div class="site-h5">
    <div class="ask_header_bg">
      <div class="ask_search_box">
        <a href="#ask-h5-form" class="ask_search_box_tw"><span class="ask_search_box_tw_c">{{ $t('wap_01570') }}</span></a>
        <div class="ask_search_ct">
          <form method="get" action="/questions">
            <input v-if="categoryId" type="hidden" name="cid" :value="categoryId" />
            <input class="ask_search_text" name="keyword" :value="keyword" :placeholder="$t('wap_01580')" />
            <i class="ask_btn_icon"><input type="submit" value="" class="ask_search_bth" /></i>
          </form>
        </div>
      </div>
    </div>
    <ul class="ask_indexnav mt10">
      <li><NuxtLink to="/questions/topics">{{ $t('wap_user_00223') }}</NuxtLink></li>
      <li><NuxtLink to="/questions">{{ $t('wap_00160') }}</NuxtLink></li>
      <li v-if="me"><NuxtLink :to="{ query: { mine: 'q' } }">{{ $t('wap_00331') }}</NuxtLink></li>
    </ul>
    <p v-if="me" class="ask_jx_info" style="padding: 0.16rem 0.4rem">
      <NuxtLink :to="{ query: { mine: 'a' } }">{{ $t('wap_01443') }}</NuxtLink>
      ·
      <NuxtLink :to="{ query: { mine: 'f' } }">{{ $t('wap_00164') }}</NuxtLink>
    </p>
    <template v-if="mineTab === 'q'">
      <div v-for="row in myQuestions?.list || []" :key="'hq-' + row.id" class="ask_ct_list mt10">
        <div class="ask_hotweek_name"><NuxtLink :to="`/questions/${row.id}`">{{ row.title }}</NuxtLink></div>
      </div>
    </template>
    <template v-else-if="mineTab === 'a'">
      <div v-for="row in myAnswers?.list || []" :key="'ha-' + row.id" class="ask_ct_list mt10">
        <div class="ask_hotweek_name"><NuxtLink :to="`/questions/${row.question_id}`">{{ row.content }}</NuxtLink></div>
      </div>
    </template>
    <template v-else-if="mineTab === 'f'">
      <div v-for="row in myAttended?.list || []" :key="'hf-' + row.id" class="ask_ct_list mt10">
        <div class="ask_hotweek_name"><NuxtLink :to="`/questions/${row.id}`">{{ row.title }}</NuxtLink></div>
      </div>
    </template>
    <template v-else>
      <form id="ask-h5-form" class="com_pop_up_area" style="padding: 0.32rem" @submit.prevent="ask">
        <input v-model="askTitle" class="ask_yz_box_text" :placeholder="$t('wap_00160')" />
        <textarea v-model="askContent" class="com_area" rows="3" />
        <div class="ask_yz_box_bth">
          <input type="submit" :value="$t('common.submit')" />
        </div>
        <p v-if="askMsg">{{ askMsg }}</p>
      </form>
      <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
      <div v-else-if="!list.length" class="wap_member_no">{{ $t('wap_01296') }}</div>
      <div v-for="row in list" :key="'h5q-' + row.id" class="ask_ct_list mt10">
        <div class="ask_hotweek_name"><NuxtLink :to="`/questions/${row.id}`">{{ row.title }}</NuxtLink></div>
        <div v-if="row.content_excerpt" class="ask_hotweek_p">{{ row.content_excerpt }}</div>
        <div class="ask_jx_info">
          <span class="ask_jx_data ask_jx_data_hd">{{ row.answer_count || 0 }} {{ $t('wap_00332') }}</span>
          <span class="ask_jx_data ask_jx_data_yl">{{ row.hits || 0 }} {{ $t('wap_01578') }}</span>
          <span class="ask_jx_data ask_jx_data_gz">{{ row.support_count || 0 }} {{ $t('common_01949') }}</span>
        </div>
        <div class="ask_jx_info_img">
          <img v-if="row.pic_n || row.pic" :src="row.pic_n || row.pic" width="20" height="20" alt="" />
          <span class="ask_jx_username">{{ row.nickname }}</span>
          <span class="ask_jx_time">{{ row.created_at_n }}</span>
        </div>
      </div>
      <Pager
        :page="page"
        :page-size="20"
        :total="data?.total || 0"
        @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
      />
    </template>
  </div>
</template>
