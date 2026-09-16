<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData('qna-topics', () =>
  api.get<Array<{ id: number; name: string; intro?: string; pid?: number; pic?: string }>>('/v1/wap/qna/categories').catch(() => []),
)
useSeoMeta({ title: t('wap_user_00223') })
useHead({
  link: [{ rel: 'stylesheet', href: '/legacy/h5/css/ask/ask.css', media: 'screen and (max-width: 1199px)' }],
})
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => (Array.isArray(data.value) ? data.value : []).filter((c) => Number(c.pid || 0) === 0 || true))
</script>

<template>
  <div class="site-pc">
    <NewsListShell :title="$t('wap_user_00223')" :error="error" :error-text="failMsg" :count="list.length">
      <SimpleCard
        v-for="row in list"
        :key="row.id"
        :to="`/questions?cid=${row.id}`"
        :title="row.name"
        :meta="row.intro || ''"
      />
    </NewsListShell>
  </div>
  <div class="site-h5">
    <p v-if="error" class="muted" style="padding: 0.4rem">{{ failMsg }}</p>
    <div v-else class="ask_topic_lb">
      <NuxtLink v-for="row in list" :key="'tag-' + row.id" :to="`/questions?cid=${row.id}`">{{ row.name }}</NuxtLink>
    </div>
    <div class="ask_topic_ct">
      <div v-for="row in list" :key="'h5-' + row.id" class="toppic_newlist">
        <div class="toppic_newname">
          <NuxtLink :to="`/questions?cid=${row.id}`" class="toppic_name_a">{{ row.name }}</NuxtLink>
        </div>
        <div v-if="row.intro" class="toppic_p">{{ row.intro }}</div>
      </div>
    </div>
  </div>
</template>
