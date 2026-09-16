<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData(
  () => `gz-${page.value}`,
  () =>
    api.get<{ list: Array<{ id: number; title: string; start_at_n?: string }>; total: number }>('/v1/wap/gongzhao', {
      page: page.value,
      page_size: 20,
    }),
)
useSeoMeta({ title: t('default_00134') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => data.value?.list || [])
</script>

<template>
  <div class="site-pc">
    <NewsListShell :title="$t('default_00134')" :error="error" :error-text="failMsg" :count="list.length">
      <SimpleCard v-for="row in list" :key="row.id" :to="`/gongzhao/${row.id}`" :title="row.title" :meta="row.start_at_n" />
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
  <div class="site-h5 news_in_body">
    <section class="news_in_cont">
      <p v-if="error" class="muted">{{ failMsg }}</p>
      <p v-else-if="!list.length" class="muted">{{ $t('wap_01469') }}</p>
      <NuxtLink v-for="row in list" :key="'h5-' + row.id" :to="`/gongzhao/${row.id}`">
        <div class="news_in_list">
          <div class="news_in_list_box_left">
            <h2>{{ row.title }}</h2>
            <div class="news_in_list_w65" style="width: 100%">
              <div class="news_in_list_date">
                <span class="news_in_eye_n">{{ row.start_at_n }}</span>
              </div>
            </div>
          </div>
        </div>
      </NuxtLink>
      <Pager
        :page="page"
        :page-size="20"
        :total="data?.total || 0"
        @update:page="(p) => navigateTo({ query: { page: p } })"
      />
    </section>
  </div>
</template>
