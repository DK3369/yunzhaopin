<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`question-${id}`, () =>
  api.get('/v1/wap/questions/detail', { id }),
)
useSeoMeta({ title: () => String(data.value?.title || '问答详情') })
useHead({ link: [{ rel: 'canonical', href: `/questions/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || '问答不存在' }}</h1>
    <p v-if="data?.catname" class="muted">{{ data.catname }} · {{ data.nickname }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.title" class="muted">没有这条问答，或暂时无法加载。</p>
  </article>
</template>
