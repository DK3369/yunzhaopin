<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('my-resume', () =>
  api.post('/v1/mcenter/resume/list', {}),
)
const { data: expects } = await useAsyncData('my-expects', () =>
  api.post('/v1/mcenter/resume/expects/list', {}).catch(() => []),
)
const form = reactive({
  name: '',
  sex: 1,
  birthday: '',
  telphone: '',
  email: '',
})
watch(
  data,
  (row) => {
    if (!row) return
    form.name = String(row.name || '')
    form.sex = Number(row.sex || 1)
    form.birthday = String(row.birthday || '')
    form.telphone = String(row.telphone || '')
    form.email = String(row.email || '')
  },
  { immediate: true },
)
const expectForm = reactive({ name: '', salary: 8000, type: 57 })
const msg = ref('')
async function saveResume() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume', { ...form })
    msg.value = '简历已保存'
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '保存失败'
  }
}
async function saveExpect() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume/expects', { ...expectForm })
    msg.value = '求职意向已添加'
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '添加失败'
  }
}
useSeoMeta({ title: '我的简历' })
</script>

<template>
  <section>
    <h1>我的简历</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <form v-else class="form" @submit.prevent="saveResume">
      <input v-model="form.name" placeholder="姓名" />
      <select v-model.number="form.sex">
        <option :value="1">男</option>
        <option :value="2">女</option>
      </select>
      <input v-model="form.birthday" placeholder="生日 YYYY-MM-DD" />
      <input v-model="form.telphone" placeholder="手机" />
      <input v-model="form.email" placeholder="邮箱" />
      <button type="submit">保存简历</button>
    </form>
    <h2>求职意向</h2>
    <p v-if="!(Array.isArray(expects) ? expects : []).length" class="muted">暂无求职意向</p>
    <ul>
      <li v-for="row in Array.isArray(expects) ? expects : []" :key="row.id">{{ row.name || row.id }}</li>
    </ul>
    <form class="form" @submit.prevent="saveExpect">
      <input v-model="expectForm.name" placeholder="意向职位名称" />
      <input v-model.number="expectForm.salary" type="number" placeholder="期望月薪" />
      <button type="submit">添加意向</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
