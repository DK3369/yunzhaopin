<script setup lang="ts">
import { listFailMsg, mediaUrl } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const keyword = computed(() => String(route.query.keyword || ''))
const kw = ref(keyword.value)
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `fairs-${page.value}-${keyword.value}`,
  () =>
    api.get<{
      list: Array<{
        id: number
        title: string
        city_name?: string
        start_at?: number
        end_at?: number
        start_at_n?: string
        banner_wap_n?: string
        pic_n?: string
        is_themb_wap?: string
      }>
      total: number
    }>('/v1/wap/zph', {
      page: page.value,
      page_size: 20,
      keyword: keyword.value || undefined,
    }),
)
useSeoMeta({ title: t('wap_00558') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
const now = Math.floor(Date.now() / 1000)
function coverOf(row: { banner_wap_n?: string; pic_n?: string; is_themb_wap?: string }) {
  return mediaUrl(row.banner_wap_n || row.pic_n || row.is_themb_wap || '')
}
function phaseOf(row: { start_at?: number; end_at?: number }) {
  const s = Number(row.start_at || 0)
  const e = Number(row.end_at || 0)
  if (s && now < s) return { cls: 'newzph_bmz', key: 'wap_00601' }
  if (e && now > e) return { cls: '', key: 'wap_00602' }
  return { cls: 'newzph_jxz', key: 'wap_00600' }
}
</script>

<template>
  <div class="site-pc">
    <NewsListShell :title="$t('wap_00558')" :error="error" :error-text="failMsg" :count="list.length">
      <form class="yun_bth_box" @submit.prevent="navigateTo({ query: { keyword: kw || undefined, page: 1 } })">
        <input v-model="kw" :placeholder="$t('common.search')" />
        <button type="submit">{{ $t('common.search') }}</button>
      </form>
      <SimpleCard
        v-for="row in list"
        :key="row.id"
        :to="`/fairs/${row.id}`"
        :title="row.title"
        :meta="`${row.city_name || ''} · ${row.start_at_n || ''}`"
      />
      <template #pager>
        <Pager
          :page="page"
          :page-size="20"
          :total="data?.total || 0"
          @update:page="(p) => navigateTo({ query: { page: p, keyword: keyword || undefined } })"
        />
      </template>
    </NewsListShell>
  </div>
  <div class="site-h5">
    <div class="newzph_pd">
      <div class="newzph_tit">
        <ul class="tab_title">
          <li class="newzph_tit_cur">{{ $t('wap_00558') }}</li>
        </ul>
      </div>
      <form class="search_cont" style="padding: 0.2rem 0.32rem" @submit.prevent="navigateTo({ query: { keyword: kw || undefined, page: 1 } })">
        <input v-model="kw" class="input_search" :placeholder="$t('common.search')" />
      </form>
      <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
      <div v-else-if="!list.length" class="newzph_notip">
        <div class="wap_member_no">{{ $t('wap_00603') }}</div>
      </div>
      <div v-else class="newzph_list">
        <ul>
          <li v-for="row in list" :key="'h5-' + row.id">
            <NuxtLink :to="`/fairs/${row.id}`">
              <div class="newzph_showbox">
                <div class="newzph_snt">
                  <img v-if="coverOf(row)" :src="coverOf(row)" alt="" width="100%" />
                </div>
                <div class="newzph_name">{{ row.title }}</div>
              </div>
              <div class="newzph_sj">{{ row.city_name }} {{ row.start_at_n }}</div>
              <span :class="phaseOf(row).key === 'wap_00602' ? 'newzph_sj_bthwq' : 'newzph_sj_bth'">{{
                $t('wap_00561')
              }}</span>
              <div :class="phaseOf(row).cls">{{ $t(phaseOf(row).key) }}</div>
            </NuxtLink>
          </li>
        </ul>
      </div>
      <Pager
        :page="page"
        :page-size="20"
        :total="data?.total || 0"
        @update:page="(p) => navigateTo({ query: { page: p, keyword: keyword || undefined } })"
      />
    </div>
  </div>
</template>
