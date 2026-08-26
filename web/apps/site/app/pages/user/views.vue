<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('profile-views', () =>
  api.post('/v1/mcenter/profile-views', { kind: 3, page: 1, page_size: 20 }),
)
useSeoMeta({ title: '谁看过我' })
</script>

<template>
  <section>
    <h1>谁看过我</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <p v-else-if="!(data?.list || []).length" class="muted">暂无谁看过我</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>viewer_uid {{ row.viewer_uid }}</h3>
        <p class="muted">kind {{ row.kind }} · {{ row.datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
