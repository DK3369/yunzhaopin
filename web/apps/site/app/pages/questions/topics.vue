<script setup lang="ts">
import { listFailMsg } from '~/utils/site'

const { t } = useI18n()
const api = useApi()
const { data, error } = await useAsyncData('qna-topics', () =>
  api.get<Array<{ id: number; name: string; intro?: string; pid?: number }>>('/v1/wap/qna/categories').catch(() => []),
)
useSeoMeta({ title: t('wap_user_00223') })
const failMsg = computed(() => listFailMsg(error.value, t('ui.rate_limit'), t('ui.load_failed')))
const list = computed(() => (Array.isArray(data.value) ? data.value : []).filter((c) => Number(c.pid || 0) === 0 || true))
</script>

<template>
  <section>
    <NewsListShell :title="$t('wap_user_00223')" :error="error" :error-text="failMsg" :count="list.length">
      <SimpleCard
        v-for="row in list"
        :key="row.id"
        :to="`/questions?cid=${row.id}`"
        :title="row.name"
        :meta="row.intro || ''"
      />
    </NewsListShell>
  </section>
</template>
