<script setup lang="ts">
const route = useRoute()
const kw = computed(() => String(route.query.kw || ''))
const scope = computed(() => String(route.query.scope || 'all'))
const api = useApi()
const { data } = await useAsyncData(
  () => `search-${scope.value}-${kw.value}`,
  () =>
    kw.value
      ? api.get('/v1/wap/search', { kw: kw.value, scope: scope.value })
      : Promise.resolve(null),
)
useSeoMeta({ title: kw.value ? `${kw.value} - 搜索` : '搜索' })
</script>

<template>
  <section>
    <h1>搜索</h1>
    <form class="form" method="get" action="/search">
      <select name="scope" :value="scope">
        <option value="all">全部</option>
        <option value="job">职位</option>
        <option value="company">企业</option>
        <option value="article">资讯</option>
      </select>
      <input name="kw" :value="kw" placeholder="关键词" />
      <button type="submit">搜索</button>
    </form>
    <p v-if="!kw" class="muted">输入关键词后搜索职位、企业、资讯。</p>
    <template v-else>
      <h2>职位</h2>
      <p v-if="!(data?.jobs || []).length" class="muted">没有匹配的职位</p>
      <div class="stack">
        <JobCard v-for="job in data?.jobs || []" :key="job.id" :job="job" />
      </div>
      <h2>企业</h2>
      <p v-if="!(data?.companies || []).length" class="muted">没有匹配的企业</p>
      <div class="stack">
        <CompanyCard v-for="c in data?.companies || []" :key="c.uid" :company="c" />
      </div>
    </template>
  </section>
</template>
