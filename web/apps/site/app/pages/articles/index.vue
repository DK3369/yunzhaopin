<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const COOKIE = 'article_nid'
function writeNidCookie(nid: string) {
  if (!import.meta.client) return
  document.cookie = `${COOKIE}=${encodeURIComponent(nid)}; path=/; max-age=${60 * 60 * 24 * 30}; SameSite=Lax`
}

const route = useRoute()
const { t } = useI18n()
const api = useApi()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const category = computed(() => String(route.query.category || route.query.nid || ''))
const subscribed = ref<number[]>([])
onMounted(() => {
  if (!import.meta.client) return
  try {
    const raw = localStorage.getItem('phpyun_article_newc')
    if (raw) {
      const parsed = JSON.parse(raw)
      if (Array.isArray(parsed)) subscribed.value = parsed.map((x: unknown) => Number(x)).filter((n) => n > 0)
    }
  } catch {
    subscribed.value = []
  }
})
watch(
  category,
  (v) => {
    if (v) writeNidCookie(v)
  },
)
const { data, error } = await useAsyncData(
  () => `articles-${page.value}-${keyword.value}-${category.value}`,
  () =>
    api.get<{
      list: Array<{
        id: number
        title: string
        datetime_n?: string
        published_at_n?: string
        summary?: string
        summary_short?: string
        picurl?: string
        s_thumb?: string
      }>
      total: number
    }>(
      '/v1/wap/articles',
      {
        page: page.value,
        page_size: 20,
        keyword: keyword.value || undefined,
        category: category.value || undefined,
      },
    ),
)
const { data: groups } = await useAsyncData('article-groups', () =>
  api.get<Array<{ id: number; name: string; parent_id?: number }>>('/v1/wap/articles/groups').catch(() => []),
)
const groupItems = computed(() => {
  const all = (groups.value || [])
    .filter((g) => !g.parent_id)
    .map((g) => ({ id: g.id, name: g.name }))
  if (!subscribed.value.length) return all
  const set = new Set(subscribed.value)
  const picked = all.filter((g) => set.has(g.id))
  return picked.length ? picked : all
})
useSeoMeta({ title: keyword.value ? `${keyword.value} - ${t('common.article')}` : t('common.article') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <div class="site-pc">
    <NewsListShell :title="$t('common.article')" :error="error" :error-text="failMsg" :count="list.length">
      <form class="form" method="get" action="/articles">
        <input name="keyword" :value="keyword" :placeholder="$t('common.search')" />
        <input v-if="category" type="hidden" name="category" :value="category" />
        <button type="submit">{{ $t('common.search') }}</button>
        <NuxtLink to="/articles/channels">{{ $t('wap_00145') }}</NuxtLink>
      </form>
      <FilterRow
        v-if="groupItems.length"
        :label="$t('common.article')"
        param="nid"
        :items="groupItems"
        :current="Number(category) || undefined"
        path="/articles"
        :all-label="$t('common.all')"
      />
      <SimpleCard
        v-for="a in list"
        :key="a.id"
        :to="`/articles/${a.id}`"
        :title="a.title"
        :meta="a.datetime_n || a.published_at_n"
      />
      <template #pager>
        <Pager
          :page="page"
          :page-size="20"
          :total="data?.total || 0"
          @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
        />
      </template>
    </NewsListShell>
  </div>
  <div class="site-h5 news_in_body">
    <div class="newheaderbox">
      <div class="news-h5-cats">
        <NuxtLink
          class="news-h5-cat"
          :class="{ 'swiper-slidea': !category }"
          :to="{ path: '/articles', query: mergeQuery(route.query, { nid: undefined, category: undefined }) }"
        >{{ $t('common.all') }}</NuxtLink>
        <NuxtLink
          v-for="g in groupItems"
          :key="g.id"
          class="news-h5-cat"
          :class="{ 'swiper-slidea': Number(category) === g.id }"
          :to="{ path: '/articles', query: mergeQuery(route.query, { nid: g.id, category: undefined }) }"
        >{{ g.name }}</NuxtLink>
      </div>
      <NuxtLink class="news_nav_box_more" to="/articles/channels" />
    </div>
    <section class="search_cont">
      <form method="get" action="/articles" class="formFiled">
        <input v-if="category" type="hidden" name="nid" :value="category" />
        <input class="input_search" name="keyword" :value="keyword" :placeholder="$t('wap_01466')" />
        <input class="input_btn" type="submit" value=" " />
      </form>
    </section>
    <section class="news_in_cont">
      <p v-if="error" class="muted">{{ failMsg }}</p>
      <p v-else-if="!list.length" class="muted">{{ $t('wap_01469') }}</p>
      <NuxtLink v-for="a in list" :key="'h5-' + a.id" :to="`/articles/${a.id}`">
        <div class="news_in_list">
          <div class="news_in_list_box_left">
            <h2>{{ a.title }}</h2>
            <div class="news_in_list_w65" :style="a.picurl || a.s_thumb ? undefined : { width: '100%' }">
              <div v-if="a.summary_short || a.summary" class="news_in_list_p">{{ a.summary_short || a.summary }}</div>
              <div class="news_in_list_date">
                <span class="news_in_eye_n">{{ a.datetime_n || a.published_at_n }}</span>
              </div>
            </div>
            <div v-if="a.picurl || a.s_thumb" class="news_in_cont_img">
              <img :src="a.picurl || a.s_thumb" alt="" width="120" height="80" />
            </div>
          </div>
        </div>
      </NuxtLink>
      <Pager
        :page="page"
        :page-size="20"
        :total="data?.total || 0"
        @update:page="(p) => navigateTo({ query: { ...route.query, page: p } })"
      />
    </section>
  </div>
</template>
