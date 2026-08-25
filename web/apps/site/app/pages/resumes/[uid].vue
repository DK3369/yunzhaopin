<script setup lang="ts">
const route = useRoute()
const uid = Number(route.params.uid)
const api = useApi()
const { data } = await useAsyncData(`resume-${uid}`, () =>
  api.get('/v1/wap/resumes/detail', { uid }),
)
const name = computed(() => String(data.value?.name || data.value?.uname || ''))
useSeoMeta({ title: () => name.value || '简历详情' })
</script>

<template>
  <article>
    <h1>{{ name || '简历不存在' }}</h1>
    <p v-if="data?.exp_n || data?.edu_n" class="muted">{{ data?.exp_n }} · {{ data?.edu_n }}</p>
    <p v-if="data?.description || data?.description_n" v-html="data?.description || data?.description_n" />
    <p v-else-if="!name" class="muted">没有这份简历，或需要企业账号才能查看。</p>
  </article>
</template>
