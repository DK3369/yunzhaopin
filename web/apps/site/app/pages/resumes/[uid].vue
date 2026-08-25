<script setup lang="ts">
const route = useRoute()
const uid = Number(route.params.uid)
const api = useApi()
const { data } = await useAsyncData(`resume-${uid}`, () =>
  api.get('/v1/wap/resumes/detail', { uid }),
)
useSeoMeta({ title: '简历详情' })
</script>

<template>
  <article>
    <h1>{{ data?.name || data?.uname || '简历' }}</h1>
    <pre class="muted">{{ JSON.stringify(data, null, 2) }}</pre>
  </article>
</template>
