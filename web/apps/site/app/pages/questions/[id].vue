<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`question-${id}`, () =>
  api.get('/v1/wap/questions/detail', { id }),
)
useSeoMeta({ title: () => String(data.value?.title || t('ui.qa')) })
useHead({ link: [{ rel: 'canonical', href: `/questions/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || $t('ui.no_questions') }}</h1>
    <p v-if="data?.catname" class="muted">{{ data.catname }} · {{ data.nickname }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.title" class="muted">{{ $t('home.no_job_data') }}</p>
  </article>
</template>
