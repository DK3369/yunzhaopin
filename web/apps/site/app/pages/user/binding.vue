<script setup lang="ts">
const api = useApi()
const { data, error, refresh } = await useAsyncData('oauth-bindings', () =>
  api.post<{ providers?: string[] }>('/v1/mcenter/oauth-bindings', {}),
)
const msg = ref('')
async function unbind(provider: string) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/oauth-bindings/unbind', { provider })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '解绑失败'
  }
}
useSeoMeta({ title: '账号绑定' })
</script>

<template>
  <section>
    <h1>账号绑定</h1>
    <p v-if="error" class="muted">请先登录。</p>
    <p v-else-if="!(data?.providers || []).length" class="muted">暂无第三方绑定</p>
    <ul v-else class="stack">
      <li v-for="p in data?.providers || []" :key="p">
        {{ p }}
        <button type="button" @click="unbind(p)">解绑</button>
      </li>
    </ul>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
