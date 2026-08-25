<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('com-apps', () =>
  api.post('/v1/mcenter/applications', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])
const invite = reactive({
  apply_id: 0,
  inter_time: Math.floor(Date.now() / 1000) + 86400,
  address: '',
  linkman: '',
  linktel: '',
  remark: '',
})
const msg = ref('')
function pick(id: number) {
  invite.apply_id = id
}
async function sendInvite() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/interviews/create', { ...invite })
    msg.value = '已发出面试邀请'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '邀请失败'
  }
}
useSeoMeta({ title: '收到的简历' })
</script>

<template>
  <section>
    <h1>收到的简历</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <p v-else-if="!list.length" class="muted">暂无投递</p>
    <div class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>投递 #{{ row.id }} · 求职者 {{ row.uid }}</h3>
        <p class="muted">职位 {{ row.job_id }} · {{ row.datetime_n }}</p>
        <button type="button" @click="pick(row.id)">邀面试</button>
      </article>
    </div>
    <h2>发出面试邀请</h2>
    <form class="form" @submit.prevent="sendInvite">
      <input v-model.number="invite.apply_id" type="number" placeholder="投递 id" />
      <input v-model.number="invite.inter_time" type="number" placeholder="面试时间 unix" />
      <input v-model="invite.address" placeholder="面试地点" />
      <input v-model="invite.linkman" placeholder="联系人" />
      <input v-model="invite.linktel" placeholder="联系电话" />
      <input v-model="invite.remark" placeholder="备注" />
      <button type="submit">发送邀请</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
