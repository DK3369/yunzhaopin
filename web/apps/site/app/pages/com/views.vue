<script setup lang="ts">
const api = useApi()
const { data, error } = await useAsyncData('com-profile-views', () =>
  api.post('/v1/mcenter/profile-views', { kind: 2, page: 1, page_size: 20 }),
)
useSeoMeta({ title: '谁看过企业' })
</script>

<template>
  <section>
    <h1>谁看过企业</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <p v-else-if="!(data?.list || []).length" class="muted">暂无访问记录</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>viewer_uid {{ row.viewer_uid }}</h3>
        <p class="muted">{{ row.datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
