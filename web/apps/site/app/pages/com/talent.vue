<script setup lang="ts">
const api = useApi()
const { data: pool, error, refresh } = await useAsyncData('talent-pool', () =>
  api.post('/v1/mcenter/talent-pool/list', { page: 1, page_size: 20 }),
)
const { data: publicResumes } = await useAsyncData('talent-search', () =>
  api.get('/v1/wap/resumes', { page: 1, page_size: 20 }),
)
const msg = ref('')
async function add(uid: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/talent-pool', { eid: uid, seeker_uid: uid })
    msg.value = '已加入人才库'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '添加失败'
  }
}
useSeoMeta({ title: '人才库' })
</script>

<template>
  <section>
    <h1>人才库</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <h2>公开简历</h2>
    <p v-if="!(publicResumes?.list || []).length" class="muted">暂无公开简历</p>
    <div class="stack">
      <article v-for="r in publicResumes?.list || []" :key="r.uid" class="job-card">
        <h3>{{ r.display_name || r.name }}</h3>
        <p class="muted">{{ r.education_n }} · {{ r.exp_n }}</p>
        <NuxtLink :to="`/resumes/${r.uid}`">查看</NuxtLink>
        <button type="button" @click="add(r.uid)">加入人才库</button>
      </article>
    </div>
    <h2>已收藏</h2>
    <p v-if="!(pool?.list || []).length" class="muted">人才库为空</p>
    <div class="stack">
      <article v-for="row in pool?.list || []" :key="row.id" class="job-card">
        <h3>求职者 {{ row.seeker_uid }}</h3>
        <p v-if="row.remark" class="muted">{{ row.remark }}</p>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
