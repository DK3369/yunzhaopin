<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('com-profile', () =>
  api.post('/v1/mcenter/company/list', {}),
)
const form = reactive({
  name: '',
  shortname: '',
  content: '',
  linkman: '',
  linkphone: '',
})
watch(
  data,
  (row) => {
    if (!row) return
    form.name = String(row.name || '')
    form.shortname = String(row.shortname || '')
    form.content = String(row.content || '')
    form.linkman = String(row.linkman || '')
    form.linkphone = String(row.linkphone || '')
  },
  { immediate: true },
)
const msg = ref('')
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company', { ...form })
    msg.value = '已保存'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '保存失败'
  }
}
useSeoMeta({ title: '企业资料' })
</script>

<template>
  <section>
    <h1>企业资料</h1>
    <p v-if="error" class="muted">请先登录企业账号。</p>
    <form v-else class="form" @submit.prevent="save">
      <input v-model="form.name" placeholder="企业名称" />
      <input v-model="form.shortname" placeholder="简称" />
      <textarea v-model="form.content" placeholder="企业简介" rows="6" />
      <input v-model="form.linkman" placeholder="联系人" />
      <input v-model="form.linkphone" placeholder="联系电话" />
      <button type="submit">保存</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
