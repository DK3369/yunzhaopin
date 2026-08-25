<script setup lang="ts">
const id = Number(useRoute().params.id)
const api = useApi()
const { data } = await useAsyncData(`tiny-${id}`, () => api.get('/v1/wap/tiny-resumes/show', { id }))
useSeoMeta({ title: () => String(data.value?.username || '普工简历详情') })
useHead({ link: [{ rel: 'canonical', href: `/tiny/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.username || '普工简历不存在' }}</h1>
    <p v-if="data?.job" class="muted">{{ data.job }} · 工龄 {{ data.exp }}</p>
    <p v-if="data?.production">{{ data.production }}</p>
    <p v-else-if="!data?.username" class="muted">没有这份普工简历，或暂时无法加载。</p>
  </article>
</template>
