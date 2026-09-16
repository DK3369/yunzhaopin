<script setup lang="ts">
import { listFailMsg, mediaUrl } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `specials-${page.value}`,
  () =>
    api.get<{
      list: Array<{
        id: number
        title: string
        intro?: string
        wappic_n?: string
        banner_n?: string
        created_at?: number
        start_at_n?: string
      }>
      total: number
    }>('/v1/wap/specials', {
      page: page.value,
      page_size: 20,
    }),
)
useSeoMeta({ title: t('wap_com_00310') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <div class="site-pc">
    <NewsListShell :title="$t('wap_com_00310')" :error="error" :error-text="failMsg" :count="list.length">
      <SimpleCard v-for="row in list" :key="row.id" :to="`/specials/${row.id}`" :title="row.title" :meta="row.intro" />
      <template #pager>
        <Pager
          :page="page"
          :page-size="20"
          :total="data?.total || 0"
          @update:page="(p) => navigateTo({ query: { page: p } })"
        />
      </template>
    </NewsListShell>
  </div>
  <div class="site-h5 special_indexbox">
    <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
    <div v-else-if="!list.length" class="wap_member_no">{{ $t('wap_01482') }}</div>
    <div v-for="row in list" :key="'h5-' + row.id" class="pic-txt">
      <div class="txt">
        <NuxtLink :to="`/specials/${row.id}`">
          <div style="width: 100%; height: 150px">
            <img
              v-if="row.wappic_n || row.banner_n"
              :src="mediaUrl(row.wappic_n || row.banner_n || '')"
              :alt="row.title"
              style="width: 100%; height: 100%; object-fit: cover"
            />
          </div>
        </NuxtLink>
        <div class="tit">
          <NuxtLink :to="`/specials/${row.id}`">{{ row.title }}</NuxtLink>
        </div>
        <div class="special_box_info">
          <span class="special_box_time">{{ row.start_at_n || row.intro }}</span>
        </div>
        <NuxtLink :to="`/specials/${row.id}`" class="s_bth">{{ $t('wap_01481') }}</NuxtLink>
      </div>
    </div>
    <Pager
      :page="page"
      :page-size="20"
      :total="data?.total || 0"
      @update:page="(p) => navigateTo({ query: { page: p } })"
    />
  </div>
</template>
