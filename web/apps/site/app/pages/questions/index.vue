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
    askMsg.value = t('common_00887')
    await refreshNuxtData()
  } catch (e: unknown) {
    askMsg.value = errKey(e) === 'unauth' ? t('wap_00032') : t(errKey(e) || 'common_00888')
  }
}
</script>

<template>
  <NewsListShell :title="$t('wap_00160')" :error="error" :error-text="failMsg" :count="list.length">
    <form class="yun_bth_box" @submit.prevent="ask">
      <input v-model="askTitle" :placeholder="$t('wap_00160')" />
      <textarea v-model="askContent" rows="3" />
      <button type="submit">{{ $t('common.submit') }}</button>
      <p v-if="askMsg">{{ askMsg }}</p>
    </form>
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
