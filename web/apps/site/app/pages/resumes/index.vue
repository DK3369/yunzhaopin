<script setup lang="ts">
const route = useRoute()
const page = computed(() => Number(route.query.page || 1))
const api = useApi()
const { data } = await useAsyncData(
  () => `resumes-${page.value}`,
  () => api.get('/v1/wap/resumes', { page: page.value, page_size: 20 }),
)
useSeoMeta({ title: '简历' })
</script>

<template>
  <section>
    <h1>简历</h1>
    <p v-if="!(data?.list || []).length" class="muted">暂无公开简历</p>
    <div class="stack">
      <NuxtLink
        v-for="r in data?.list || []"
        :key="r.uid"
        :to="`/resumes/${r.uid}`"
        class="job-card"
      >
        <h3>{{ r.name || r.uname }}</h3>
        <p class="muted">{{ r.job_classid_n || r.expect }}</p>
      </NuxtLink>
    </div>
  </section>
</template>
