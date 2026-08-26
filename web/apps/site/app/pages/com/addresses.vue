<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('com-addresses', () =>
  api.post('/v1/mcenter/company-addresses', { page: 1, page_size: 20 }),
)
const form = reactive({
  link_man: '',
  link_moblie: '',
  link_phone: '',
  email: '',
  link_address: '',
  province_id: 0,
  city_id: 0,
  three_city_id: 0,
  x: '',
  y: '',
})
const msg = ref('')
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-addresses/create', { ...form })
    msg.value = '已添加'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '添加失败'
  }
}
useSeoMeta({ title: '地图标注' })
</script>

<template>
  <section>
    <h1>工作地址 / 地图标注</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <p class="muted">字段 <code>x</code> / <code>y</code> 与 PHP 企业地址表一致。</p>
    <form class="form" @submit.prevent="create">
      <input v-model="form.link_man" placeholder="link_man" />
      <input v-model="form.link_moblie" placeholder="link_moblie" />
      <input v-model="form.link_address" placeholder="link_address" />
      <input v-model="form.x" placeholder="x 经度" />
      <input v-model="form.y" placeholder="y 纬度" />
      <button type="submit">添加</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">暂无地址</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.link_man }} · {{ row.link_address }}</h3>
        <p class="muted">x {{ row.x }} y {{ row.y }}</p>
      </article>
    </div>
  </section>
</template>
